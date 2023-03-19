#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::upper_case_acronyms)]

use std::borrow::Cow;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use color_eyre::eyre::{eyre, Result, WrapErr};
use lazy_static::lazy_static;
use regex::Regex;

/// URL of the Linux repository to pull the syscall tables from.
static LINUX_REPO: &str = "https://raw.githubusercontent.com/torvalds/linux";

/// Linux version to pull the syscall tables from.
static LINUX_VERSION: &str = "v5.17";

lazy_static! {
    /// List of syscall tables for each architecture.
    static ref SOURCES: Vec<Source<'static>> = vec![
        Source::Table(Table {
            arch: "x86",
            path: "arch/x86/entry/syscalls/syscall_32.tbl",
            abi: &[ABI::I386],
        }),
        Source::Table(Table {
            arch: "x86_64",
            path: "arch/x86/entry/syscalls/syscall_64.tbl",
            abi: &[ABI::COMMON, ABI::B64],
        }),
        Source::Table(Table {
            arch: "arm",
            path: "arch/arm/tools/syscall.tbl",
            abi: &[ABI::COMMON],
        }),
        // NOTE: arm64/aarch64 is a little different from all the other tables.
        // These are defined in `unistd.h`, which is supposed to be the method
        // used for all new architectures going forward.
        Source::Header(Header {
            arch: "aarch64",
            headers: &[
                "include/uapi/asm-generic/unistd.h",
                //"arch/arm64/include/asm/unistd.h",
            ],
            blocklist: &[
                // This syscall was renamed to `sync_file_range2` on aarch64.
                // Thus, only `sync_file_range2` should appear in the syscall
                // table.
                "sync_file_range",
            ],
        }),
        Source::Table(Table {
            arch: "sparc",
            path: "arch/sparc/kernel/syscalls/syscall.tbl",
            abi: &[ABI::COMMON, ABI::B32],
        }),
        Source::Table(Table {
            arch: "sparc64",
            path: "arch/sparc/kernel/syscalls/syscall.tbl",
            abi: &[ABI::COMMON, ABI::B64],
        }),
        Source::Table(Table {
            arch: "powerpc",
            path: "arch/powerpc/kernel/syscalls/syscall.tbl",
            abi: &[ABI::COMMON, ABI::NOSPU, ABI::B32],
        }),
        Source::Table(Table {
            arch: "powerpc64",
            path: "arch/powerpc/kernel/syscalls/syscall.tbl",
            abi: &[ABI::COMMON, ABI::NOSPU, ABI::B64],
        }),
        Source::Table(Table {
            arch: "mips",
            path: "arch/mips/kernel/syscalls/syscall_o32.tbl",
            abi: &[ABI::O32],
        }),
        Source::Table(Table {
            arch: "mips64",
            path: "arch/mips/kernel/syscalls/syscall_n64.tbl",
            abi: &[ABI::N64],
        }),
        Source::Table(Table {
            arch: "s390x",
            path: "arch/s390/kernel/syscalls/syscall.tbl",
            abi: &[ABI::COMMON, ABI::B64],
        }),
    ];
}

struct ABI<'a> {
    name: &'a str,
    offset: u32,
}

impl<'a> ABI<'a> {
    // Different syscall ABIs have different offsets. This currently only
    // applies to MIPS and ia64. (Search for `__NR_Linux` in the kernel source
    // to find syscall offsets.)
    pub const COMMON: Self = Self::new("common", 0);
    pub const I386: Self = Self::new("i386", 0);
    pub const NOSPU: Self = Self::new("nospu", 0);
    pub const B32: Self = Self::new("32", 0);
    pub const B64: Self = Self::new("64", 0);
    pub const O32: Self = Self::new("o32", 4000);
    pub const N64: Self = Self::new("n64", 5000);

    pub const fn new(name: &'a str, offset: u32) -> Self {
        Self { name, offset }
    }
}

struct Table<'a> {
    arch: &'a str,
    path: &'a str,
    abi: &'a [ABI<'a>],
}

struct Header<'a> {
    arch: &'a str,
    headers: &'a [&'a str],
    blocklist: &'a [&'a str],
}

enum Source<'a> {
    /// The definitions are in a `syscall.tbl` file.
    Table(Table<'a>),
    /// The definitions are in a unistd.h header file.
    Header(Header<'a>),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct TableEntry {
    id: u32,
    name: String,
    entry_point: Option<String>,
}

impl TableEntry {
    fn ident(&self) -> Cow<str> {
        if self.name.as_str() == "break" {
            Cow::Owned(format!("r#{}", self.name))
        } else {
            Cow::Borrowed(&self.name)
        }
    }
}

impl<'a> Table<'a> {
    async fn fetch_table(&self) -> Result<Vec<TableEntry>> {
        let contents = fetch_path(self.path).await?;

        let mut table = Vec::new();

        for line in contents.lines() {
            let line = line.trim();

            // Skip over empty lines and comments.
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut fields =
                line.split(char::is_whitespace).filter(|x| !x.is_empty());

            let id: u32 = fields
                .next()
                .ok_or_else(|| {
                    eyre!("Missing syscall number (line {:?})", line)
                })?
                .parse()
                .wrap_err_with(|| eyre!("Failed parsing line {:?}", line))?;
            let abi_name = fields.next().ok_or_else(|| {
                eyre!("Missing syscall abi field (line {:?})", line)
            })?;
            let name = fields
                .next()
                .ok_or_else(|| {
                    eyre!("Missing syscall name field (line {:?})", line)
                })?
                .into();
            let entry_point = fields.next().map(Into::into);

            for abi in self.abi {
                if abi.name == abi_name {
                    table.push(TableEntry {
                        id: id + abi.offset,
                        name,
                        entry_point,
                    });
                    break;
                }
            }
        }

        // The table should already be sorted, but lets make sure.
        table.sort();

        Ok(table)
    }
}

impl<'a> Header<'a> {
    async fn fetch_table(&self) -> Result<Vec<TableEntry>> {
        lazy_static! {
            // Pattern for matching the syscall definition.
            static ref RE_SYSCALLNR: Regex = Regex::new(r"^#define\s+__NR(?:3264)?_([a-z0-9_]+)\s+(\d+)").unwrap();
        }

        let mut table = Vec::new();

        for header in self.headers {
            let contents = fetch_path(header).await?;

            for line in contents.lines() {
                let line = line.trim();

                if let Some(cap) = RE_SYSCALLNR.captures(line) {
                    let name: &str = cap[1].into();
                    let id: u32 = cap[2].parse()?;

                    if name == "syscalls" {
                        // This just keeps track of the number of syscalls in
                        // the table and isn't a real syscall.
                        continue;
                    }

                    if name == "arch_specific_syscall" {
                        // This is a placeholder for a block of 16 syscalls
                        // that are reserved for future use.
                        continue;
                    }

                    if self.blocklist.contains(&name) {
                        continue;
                    }

                    table.push(TableEntry {
                        id,
                        name: name.into(),
                        entry_point: Some(format!("sys_{name}")),
                    });
                }
            }
        }

        // The table should already be sorted, but lets make sure.
        table.sort();

        Ok(table)
    }
}

impl<'a> Source<'a> {
    pub fn arch(&self) -> &'a str {
        match self {
            Self::Table(table) => table.arch,
            Self::Header(header) => header.arch,
        }
    }

    async fn fetch_table(&self) -> Result<Vec<TableEntry>> {
        match self {
            Self::Table(table) => table.fetch_table().await,
            Self::Header(header) => header.fetch_table().await,
        }
    }

    /// Generates the source file.
    async fn generate(&self, dir: &Path) -> Result<(&str, PathBuf)> {
        let table = self.fetch_table().await.wrap_err_with(|| {
            eyre!("Failed fetching table for {}", self.arch())
        })?;

        // Generate `src/arch/{arch}.rs`
        let syscalls = dir.join(format!("src/arch/{}.rs", self.arch()));

        let mut file = fs::File::create(&syscalls)
            .wrap_err_with(|| eyre!("Failed to create file {:?}", syscalls))?;
        writeln!(
            file,
            "//! Syscalls for the `{}` architecture.\n",
            self.arch()
        )?;
        write!(file, "{}", SyscallFile(&table))?;

        Ok((self.arch(), syscalls))
    }
}

struct SyscallFile<'a>(&'a [TableEntry]);

impl<'a> fmt::Display for SyscallFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "// This file is automatically generated. Do not edit!")?;
        writeln!(f)?;

        writeln!(f, "syscall_enum! {{")?;
        writeln!(f, "    pub enum Sysno {{")?;
        for entry in self.0 {
            if entry.entry_point.is_some() {
                writeln!(
                    f,
                    "        /// See [{name}(2)](https://man7.org/linux/man-pages/man2/{name}.2.html) for more info on this syscall.",
                    name = entry.ident(),
                )?;
                writeln!(
                    f,
                    "        {name} = {id},",
                    name = entry.ident(),
                    id = entry.id
                )?;
            } else {
                // This syscall has no entry point in the kernel, so we could
                // technically exclude this from our list, but that will leave
                // gaps in the syscall table. Our match statements can be better
                // optimized by the compiler if we don't have gaps in the
                // numbering.
                writeln!(
                    f,
                    "        /// NOTE: `{name}` is not implemented in the kernel.",
                    name = entry.ident(),
                )?;
                writeln!(
                    f,
                    "        {name} = {id},",
                    name = entry.ident(),
                    id = entry.id
                )?;
            }
        }
        writeln!(f, "    }}")?;
        writeln!(f, "    LAST: {};", self.0.last().unwrap().ident())?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

/// Fetches a file path from the repository.
async fn fetch_path(path: &str) -> Result<String> {
    let url = format!("{LINUX_REPO}/{LINUX_VERSION}/{path}");

    let contents = reqwest::get(&url)
        .await
        .wrap_err_with(|| eyre!("Failed to fetch URL '{}'", url))?
        .text()
        .await
        .wrap_err_with(|| eyre!("Failed to parse contents of URL '{}'", url))?;

    Ok(contents)
}

async fn generate_errno(dest: &Path) -> Result<()> {
    let table = fetch_errno(&[
        "include/uapi/asm-generic/errno-base.h",
        "include/uapi/asm-generic/errno.h",
        // error codes private to the Kernel, but are still useful when
        // ptracing.
        "include/linux/errno.h",
    ])
    .await?;

    let mut file = fs::File::create(dest)
        .wrap_err_with(|| eyre!("Failed to create file {:?}", dest))?;
    write!(file, "{}", ErrnoFile(&table))?;

    Ok(())
}

#[derive(Debug, Clone)]
enum Errno {
    Definition {
        name: String,
        num: u32,
        description: Option<String>,
    },
    #[allow(unused)]
    Alias {
        alias: String,
        name: String,
        description: Option<String>,
    },
}

async fn fetch_errno(paths: &[&str]) -> Result<Vec<Errno>> {
    let mut errnos = Vec::new();

    for path in paths {
        let contents = fetch_path(path).await?;

        parse_errno(&contents, &mut errnos)?;
    }

    Ok(errnos)
}

fn parse_errno(contents: &str, errnos: &mut Vec<Errno>) -> Result<()> {
    lazy_static! {
        // Pattern for matching the errno definition
        static ref RE_DEFINITION: Regex = Regex::new(r"^#define\s+(E\w+)\s+(\d+)(?:\s+/\*([^\\*]+)\*/)?").unwrap();
        // Pattern for matching errno aliases
        static ref RE_ALIAS: Regex = Regex::new(r#"^#define\s+(E\w+)\s+(E\w+)(?:\s+/\*([^\\*]+)\*/)?"#).unwrap();
    }

    for line in contents.lines() {
        if let Some(cap) = RE_DEFINITION.captures(line) {
            let name = cap[1].into();
            let num: u32 = cap[2].parse()?;
            let description = cap.get(3).map(|m| m.as_str().trim().to_string());

            errnos.push(Errno::Definition {
                name,
                num,
                description,
            });
        } else if let Some(cap) = RE_ALIAS.captures(line) {
            let alias = cap[1].into();
            let name = cap[2].into();
            let description = cap.get(3).map(|m| m.as_str().trim().to_string());

            errnos.push(Errno::Alias {
                alias,
                name,
                description,
            });
        }
    }

    Ok(())
}

struct ErrnoFile<'a>(&'a Vec<Errno>);

impl<'a> fmt::Display for ErrnoFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "// This file is automatically generated. Do not edit!")?;
        writeln!(f)?;

        writeln!(f, "errno_enum! {{")?;
        writeln!(f, "    pub enum Errno {{")?;

        for value in self.0.iter() {
            match value {
                Errno::Definition {
                    name,
                    num,
                    description,
                } => {
                    let description = description.as_ref().map_or_else(
                        || {
                            // Try to make a best-effort guess for error codes that
                            // don't have a description.
                            match name.as_str() {
                                "ERESTARTSYS" => "Restart syscall",
                                "ERESTARTNOINTR" => "Restart if no interrupt",
                                _ => panic!(
                                    "Could not find a description for {}",
                                    name
                                ),
                            }
                        },
                        |s| s.as_str(),
                    );

                    writeln!(f, r#"        {name}({num}) = "{description}","#)?;
                }
                Errno::Alias {
                    alias: _,
                    name: _,
                    description: _,
                } => {
                    // Don't include aliases since it makes our macro more
                    // complicated. We can add these manually later.
                }
            }
        }

        writeln!(f, "    }}")?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let base_dir = Path::new("..");

    for source in SOURCES.iter() {
        let (arch, path) = source.generate(base_dir).await?;

        println!("Generated syscalls for {arch} at {path:?}");
    }

    let errno = base_dir.join("src/errno/generated.rs");
    generate_errno(&errno).await?;
    println!("Generated errno table at {errno:?}");

    Ok(())
}
