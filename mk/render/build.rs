use std::env::{var as env_var, VarError};

fn main() -> Result<(), VarError> {
    match (
        env_var("CARGO_CFG_TARGET_OS")?.as_str(),
        env_var("CARGO_CFG_TARGET_ARCH")?.as_str(),
    ) {
        ("linux", "x86_64") | ("macos", "x86_64") | ("macos", "aarch64") | ("windows", "x86_64") => {
            println!("cargo:rustc-cfg=gl33");
        }
        _ => {
            println!("cargo:warning={}", "the given target is not available");
        }
    }

    Ok(())
}
