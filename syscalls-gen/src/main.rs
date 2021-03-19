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
static LINUX_VERSION: &str = "v5.13-rc1";

lazy_static! {
    /// List of syscall tables for each architecture.
    static ref SOURCES: Vec<Source<'static>> = vec![
        Source {
            arch: "x86",
            path: "arch/x86/entry/syscalls/syscall_32.tbl",
            abi: vec!["i386"],
        },
        Source {
            arch: "x86_64",
            path: "arch/x86/entry/syscalls/syscall_64.tbl",
            abi: vec!["common", "64"],
        },
        Source {
            arch: "arm",
            path: "arch/arm/tools/syscall.tbl",
            abi: vec!["common"],
        },
        Source {
            arch: "sparc",
            path: "arch/sparc/kernel/syscalls/syscall.tbl",
            abi: vec!["common", "32"],
        },
        Source {
            arch: "sparc64",
            path: "arch/sparc/kernel/syscalls/syscall.tbl",
            abi: vec!["common", "64"],
        },
        Source {
            arch: "powerpc",
            path: "arch/powerpc/kernel/syscalls/syscall.tbl",
            abi: vec!["common", "nospu", "32"],
        },
        Source {
            arch: "powerpc64",
            path: "arch/powerpc/kernel/syscalls/syscall.tbl",
            abi: vec!["common", "nospu", "64"],
        },
        Source {
            arch: "mips",
            path: "arch/mips/kernel/syscalls/syscall_o32.tbl",
            abi: vec!["o32"],
        },
        Source {
            arch: "mips64",
            path: "arch/mips/kernel/syscalls/syscall_n64.tbl",
            abi: vec!["n64"],
        },
        Source {
            arch: "s390x",
            path: "arch/s390/kernel/syscalls/syscall.tbl",
            abi: vec!["common", "64"],
        },
    ];
}

// `src/{arch}/mod.rs`
static ARCH_MOD: &str = r#"
mod syscalls;

pub use self::syscalls::*;
"#;

struct Source<'a> {
    arch: &'a str,
    path: &'a str,
    abi: Vec<&'a str>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct TableEntry {
    id: u32,
    name: String,
    entry_point: Option<String>,
}

impl TableEntry {
    fn ident(&self) -> Cow<str> {
        let is_keyword = match self.name.as_str() {
            "break" => true,
            _ => false,
        };

        if is_keyword {
            Cow::Owned(format!("r#{}", self.name))
        } else {
            Cow::Borrowed(&self.name)
        }
    }
}

impl<'a> Source<'a> {
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

            let id = fields
                .next()
                .ok_or_else(|| {
                    eyre!("Missing syscall number (line {:?})", line)
                })?
                .parse()
                .wrap_err_with(|| eyre!("Failed parsing line {:?}", line))?;
            let abi = fields.next().ok_or_else(|| {
                eyre!("Missing syscall abi field (line {:?})", line)
            })?;
            let name = fields
                .next()
                .ok_or_else(|| {
                    eyre!("Missing syscall name field (line {:?})", line)
                })?
                .into();
            let entry_point = fields.next().map(Into::into);

            if self.abi.contains(&abi) {
                table.push(TableEntry {
                    id,
                    name,
                    entry_point,
                });
            }
        }

        // The table should already be sorted, but lets make sure.
        table.sort();

        Ok(table)
    }

    /// Generates the source file.
    async fn generate(&self, dir: &Path) -> Result<(&str, PathBuf)> {
        let table = self
            .fetch_table()
            .await
            .wrap_err_with(|| eyre!("Failed fetching table {:?}", self.path))?;

        let dir = dir.join(format!("src/arch/{}", self.arch));

        fs::create_dir_all(&dir)
            .wrap_err_with(|| eyre!("Failed to create directory {:?}", dir))?;

        // Generate `src/{arch}/mod.rs`
        let module = dir.join("mod.rs");

        let mut file = fs::File::create(&module)
            .wrap_err_with(|| eyre!("Failed to create file {:?}", module))?;
        writeln!(file, "//! Syscalls for the {} architecture.", self.arch)?;
        file.write_all(ARCH_MOD.as_bytes())?;

        // Generate `src/{arch}/syscalls.rs`
        let syscalls = dir.join("syscalls.rs");

        let mut file = fs::File::create(&syscalls)
            .wrap_err_with(|| eyre!("Failed to create file {:?}", syscalls))?;
        writeln!(file, "//! Syscalls for the {} architecture.\n", self.arch)?;
        write!(file, "{}", SyscallFile(&table))?;

        Ok((self.arch, dir))
    }
}

struct SyscallFile<'a>(&'a [TableEntry]);

impl<'a> fmt::Display for SyscallFile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "// This file is automatically generated. Do not edit!")?;
        writeln!(f, "")?;

        writeln!(f, "syscall_enum! {{")?;
        writeln!(f, "    pub enum Sysno {{")?;
        for entry in self.0 {
            if entry.entry_point.is_some() {
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
    let url = format!(
        "{repo}/{version}/{path}",
        repo = LINUX_REPO,
        version = LINUX_VERSION,
        path = path
    );

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
        writeln!(f, "")?;

        writeln!(f, "errno_enum! {{")?;
        writeln!(f, "    pub enum Errno {{")?;

        for value in self.0.iter() {
            match value {
                Errno::Definition {
                    name,
                    num,
                    description,
                } => {
                    let description = description
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or_else(|| {
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
                        });

                    writeln!(
                        f,
                        "        {}({}) = \"{}\",",
                        name, num, description
                    )?;
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
        let (arch, path) = source.generate(&base_dir).await?;

        println!("Generated syscalls for {} at {:?}", arch, path);
    }

    let errno = base_dir.join("src/errno/generated.rs");
    generate_errno(&errno).await?;
    println!("Generated errno table at {:?}", errno);

    Ok(())
}
