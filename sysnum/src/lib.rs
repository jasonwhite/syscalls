#![allow(dead_code, unused_imports)]

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

static CPP: &str = "cpp";

#[cfg(not(target_os = "linux"))]
compile_error!("sysnr only supports Linux");

#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
compile_error!("sysnr only supports x86_64");

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

pub fn gen_syscalls() -> Result<Vec<(String, i32)>> {
    get_asm_unistd_h().and_then(|x| gen_syscalls_from(x))
}

#[test]
fn can_parse_syscalls() {
    let syscalls = gen_syscalls().expect("fail to parse asm/unistd.h");
    assert!(syscalls.len() > 100);
}

#[test]
fn syscalls_are_continous() {
    let mut dict = BTreeMap::new();

    let syscalls = gen_syscalls().expect("fail to parse asm/unistd.h");

    for (name, nr) in syscalls {
        dict.insert(nr, name);
    }

    dict.keys().zip(0..).for_each(|(k, i)| assert_eq!(*k, i));
}
