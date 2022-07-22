#[cfg(feature = "asset_loader")]
fn test() {}

#[cfg(feature = "fs_loader")]
fn test() {
    use res::fs_loader::*;
}
