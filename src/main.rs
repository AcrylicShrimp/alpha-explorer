use anyhow::Result;
use mk::run;
use pollster::FutureExt;
use std::env::current_dir;

fn main() -> Result<()> {
    run(
        "alpha-explorer",
        1024,
        768,
        true,
        current_dir()?.join("assets"),
        "assets/scripts/entry.lua",
    )
    .block_on()
}
