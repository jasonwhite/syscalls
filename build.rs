use std::path::PathBuf;

fn main() {
    // The target architecture should be the first value in the TARGET triple.
    let target = std::env::var("TARGET").ok();

    let target_arch = target
        .as_ref()
        .and_then(|target| target.split('-').next())
        .unwrap_or("x86_64");

    let file = PathBuf::from(format!("src/arch/{}/syscall.c", target_arch));

    println!("cargo:rerun-if-changed={}", file.display());

    if file.exists() {
        cc::Build::new().file(file).compile("syscall");
    } else {
        println!(
            "cargo:warning=The '{}' architecture is currently unsupported in the syscalls crate. Raw syscalls for that architecture will not be available.",
            target_arch
        );
    }
}
