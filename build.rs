use std::path::PathBuf;

fn main() {
    // The target architecture should be the first value in the TARGET triple.
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH");
    let target_arch = target_arch.as_ref().map(String::as_str);
    let target_arch = target_arch.unwrap_or("x64_64");

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
