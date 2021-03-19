use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use structopt::StructOpt;
use tempfile::NamedTempFile;

#[derive(Debug, StructOpt)]
struct Opts {
    /// The source file containing the syscalls to generate.
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

static CPP: &str = "cpp";

#[cfg(not(target_os = "linux"))]
compile_error!("syscalls only supports Linux");

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
compile_error!("syscalls only supports x86_64");

#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
static UNISTD_H: &str = "asm/unistd_x32.h";
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
static UNISTD_H: &str = "asm/unistd_64.h";
#[cfg(target_arch = "x86")]
static UNISTD_H: &str = "asm/unistd_32.h";

fn get_asm_unistd_h() -> Result<String> {
    let mut file = NamedTempFile::new()?;
    writeln!(file.as_file_mut(), "#include <sys/syscall.h>")?;
    let subp = Command::new(CPP)
        .arg("-xc")
        .arg(file.path())
        .stdout(Stdio::piped())
        .spawn()?;
    let output = subp.wait_with_output()?;
    let cpp_out: String = unsafe { String::from_utf8_unchecked(output.stdout) };
    let asm_unistd: Option<&str> = cpp_out
        .lines()
        .filter(|x| x.contains(UNISTD_H))
        .next()
        .map(|line| {
            line.split_whitespace()
                .filter(|x| x.contains(UNISTD_H))
                .next()
        })
        .unwrap_or(None);
    if let Some(unistd) = asm_unistd {
        Ok(unistd.chars().filter(|x| *x != '"').collect())
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "cpp returned expected result.",
        ))
    }
}

fn parse_syscalls(unistd: String) -> Result<Vec<(usize, String)>> {
    let mut file = File::open(unistd)?;
    let mut buff = String::new();
    let mut ret = Vec::new();
    file.read_to_string(&mut buff)?;
    for candidate in buff
        .lines()
        .filter(|x| x.starts_with("#define") && x.contains("__NR_"))
    {
        let words = candidate.split_whitespace();
        let mut it = words.skip_while(|x| !x.starts_with("__NR_"));
        let name_ = it.next();
        let nr_ = it.filter(|x| x.parse::<usize>().is_ok()).next();
        if let Some((name, nr)) = name_.and_then(|x| {
            nr_.and_then(|y| {
                y.parse::<usize>()
                    .ok()
                    .and_then(|z| Some((x.to_string(), z)))
            })
        }) {
            let name = name.strip_prefix("__NR_").unwrap_or(&name);
            ret.push((nr, name.to_owned()));
        }
    }

    Ok(ret)
}

/// Converts a list of syscall to an index vector of syscalls (where holes are
/// denoted with `None`). This also sorts the syscalls.
///
/// Not every syscall number maps to a real syscall.
fn syscall_table(syscalls: Vec<(usize, String)>) -> Vec<Option<String>> {
    // Find the syscall with the highest number. This will be the length of our
    // table.
    let max = syscalls.iter().max_by_key(|(nr, _)| nr).unwrap().0;

    let mut table = vec![None; max + 1];

    for (nr, name) in syscalls {
        table[nr] = Some(name);
    }

    table
}

fn gen_syscalls() -> Result<Vec<Option<String>>> {
    get_asm_unistd_h().and_then(|x| parse_syscalls(x).map(syscall_table))
}

fn gen_syscall_nrs(dest: &Path) -> Result<()> {
    let syscalls = gen_syscalls()?;
    assert!(syscalls.len() > 100);

    let mut f = File::create(dest)?;
    writeln!(f, "// AUTOMATICALLY GENERATED. DO NOT EDIT.\n")?;
    writeln!(f, "pub use self::SyscallNo::*;")?;
    writeln!(f, "use core::fmt;")?;
    writeln!(f, "#[cfg(feature = \"serde_repr\")]")?;
    writeln!(f, "use serde_repr::{{Deserialize_repr, Serialize_repr}};")?;

    writeln!(
        f,
        "#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]"
    )?;
    writeln!(f, "#[derive(PartialEq, Eq, Clone, Copy)]")?;
    writeln!(f, "#[cfg_attr(feature = \"serde_repr\", derive(Serialize_repr, Deserialize_repr))]")?;
    writeln!(f, "#[repr(i32)]")?;
    writeln!(f, "pub enum SyscallNo {{")?;

    for (nr, name) in syscalls.iter().enumerate() {
        if let Some(name) = name {
            writeln!(f, "    SYS_{} = {},", name, nr)?;
        }
    }
    writeln!(f, "}}")?;

    writeln!(
        f,
        "static SYSCALL_NAMES: [Option<&str>; {}] = [",
        syscalls.len()
    )?;
    for name in &syscalls {
        writeln!(f, "    {:?},", name)?;
    }
    writeln!(f, "];\n")?;

    f.write(
        br#"impl SyscallNo {
    /// Returns the name of the syscall.
    #[inline]
    pub fn name(&self) -> &'static str {
        SYSCALL_NAMES[*self as usize].unwrap()
    }

    /// Constructs a `SyscallNo` from an ID. Returns `None` if the number falls
    /// outside the bounds of possible enum values.
    pub fn new(id: usize) -> Option<Self> {
        SYSCALL_IDS.get(id).and_then(|x| *x)
    }
}

"#,
    )?;

    f.write(
        br#"impl fmt::Display for SyscallNo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

"#,
    )?;

    f.write(
        br#"impl fmt::Debug for SyscallNo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

"#,
    )?;

    writeln!(
        f,
        "static SYSCALL_IDS: [Option<SyscallNo>; {}] = [",
        syscalls.len()
    )?;
    for name in &syscalls {
        if let Some(name) = name {
            writeln!(f, "    Some(SYS_{}),", name)?;
        } else {
            writeln!(f, "    None,")?;
        }
    }
    writeln!(f, "];")?;

    f.write(
        br#"impl From<i32> for SyscallNo {
    fn from(id: i32) -> Self {
        Self::new(id as usize)
            .unwrap_or_else(|| panic!("invalid syscall: {}", id))
    }
}
"#,
    )?;

    Ok(())
}

fn main() {
    let opt = Opts::from_args();
    gen_syscall_nrs(&opt.output).unwrap();
}
