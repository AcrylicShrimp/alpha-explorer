use anyhow::Result;
use mk::run;
use std::env::current_dir;

fn main() -> Result<()> {
    run(
        "alpha-explorer",
        1024,
        768,
        false,
        current_dir()?.join("assets"),
        "assets/scripts/entry.lua",
    )
}
