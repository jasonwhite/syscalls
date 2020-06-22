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

fn gen_syscalls_from(unistd: String) -> Result<Vec<(String, i32)>> {
    let mut file = File::open(unistd)?;
    let mut buff = String::new();
    let mut ret: Vec<(String, i32)> = Vec::new();
    file.read_to_string(&mut buff)?;
    for candidate in buff
        .lines()
        .filter(|x| x.starts_with("#define") && x.contains("__NR_"))
    {
        let words = candidate.split_whitespace();
        let mut it = words.skip_while(|x| !x.starts_with("__NR_"));
        let name_ = it.next();
        let nr_ = it.filter(|x| x.parse::<i32>().is_ok()).next();
        if let Some((name, nr)) = name_.and_then(|x| {
            nr_.and_then(|y| {
                y.parse::<i32>().ok().and_then(|z| Some((x.to_string(), z)))
            })
        }) {
            ret.push((name, nr));
        }
    }

    Ok(ret)
}

fn gen_syscalls() -> Result<Vec<(String, i32)>> {
    get_asm_unistd_h().and_then(|x| gen_syscalls_from(x))
}

fn gen_syscall_nrs(dest: &Path) -> Result<()> {
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

    let syscalls = gen_syscalls()?;
    assert!(syscalls.len() > 100);

    for (name, nr) in &syscalls {
        writeln!(
            f,
            "    SYS{} = {},",
            name.chars().skip(4).collect::<String>(),
            nr
        )?;
    }
    writeln!(f, "}}")?;

    writeln!(f, "static SYSCALL_NAMES: [&str; {}] = [", syscalls.len())?;
    for (name, _) in &syscalls {
        writeln!(
            f,
            "    \"{}\",",
            name.chars().skip(5).collect::<String>().as_str()
        )?;
    }
    writeln!(f, "];\n")?;

    f.write(
        br#"impl SyscallNo {
    #[inline]
    pub fn name(&self) -> &'static str {
        SYSCALL_NAMES[*self as usize]
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

    writeln!(f, "static SYSCALL_IDS: [SyscallNo; {}] = [", syscalls.len())?;
    for (name, _) in &syscalls {
        writeln!(f, "    SYS{},", name.chars().skip(4).collect::<String>())?;
    }
    writeln!(f, "];")?;

    writeln!(f, "impl From<i32> for SyscallNo {{")?;
    writeln!(f, "    fn from(item: i32) -> Self {{")?;
    writeln!(f, "        if item as usize > SYSCALL_IDS.len() {{")?;
    writeln!(f, "            panic!(\"invalid syscall: {{}}\", item)")?;
    writeln!(f, "        }} else {{")?;
    writeln!(f, "            SYSCALL_IDS[item as usize]")?;
    writeln!(f, "        }}")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;

    Ok(())
}

fn main() {
    let opt = Opts::from_args();
    gen_syscall_nrs(&opt.output).unwrap();
}
