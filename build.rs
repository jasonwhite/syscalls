use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Automatically detect if thumb-mode is an available feature by looking at
    // the prefix of the target. Currently, the thumb-mode target feature is
    // only set automatically in nightly builds, so we must do the manual
    // feature detect here.
    if env::var("TARGET").map_or(false, |t| t.starts_with("thumb")) {
        println!("cargo:rustc-cfg=target_feature=\"thumb-mode\"");
    }
}
