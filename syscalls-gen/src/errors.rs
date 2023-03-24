use crate::fetch_path;
use color_eyre::eyre::{eyre, Result, WrapErr};
use futures::future::try_join_all;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct ErrnoFile<'a>(&'a Vec<Errno>);

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
                                    "Could not find a description for {name}"
                                ),
                            }
                        },
                        |s| s.as_str(),
                    );

                    writeln!(f, r#"        {name}({num}) = "{description}","#)?;
                }
                Errno::Alias { .. } => {
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

pub async fn generate_errno(path: PathBuf) -> Result<()> {
    let table = fetch_errno(&[
        "include/uapi/asm-generic/errno-base.h",
        "include/uapi/asm-generic/errno.h",
        // error codes private to the Kernel, but are still useful when
        // ptracing.
        "include/linux/errno.h",
    ])
    .await?;

    let mut file = File::create(&path)
        .wrap_err_with(|| eyre!("Failed to create file {}", &path.display()))?;
    write!(file, "{}", ErrnoFile(&table))?;

    println!("Generated errno table at {}", &path.display());
    Ok(())
}

#[derive(Debug, Clone)]
pub enum Errno {
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
    let futures: Vec<_> = paths.iter().map(|path| fetch_path(path)).collect();

    let mut errnos = Vec::new();
    for content in try_join_all(futures).await? {
        parse_errno(&content, &mut errnos)?;
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
