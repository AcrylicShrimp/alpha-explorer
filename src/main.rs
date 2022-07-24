use mk::{run, EngineError};
use std::env::current_dir;

fn main() -> Result<(), EngineError> {
    run(
        "alpha-explorer",
        640,
        480,
        false,
        current_dir()?.join("assets"),
        "assets/scripts/entry.lua",
    )
}
