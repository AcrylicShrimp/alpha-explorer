use anyhow::Result;
use mk::{run, winit::window::Window, EngineContext};
use pollster::FutureExt;
use script::GameModule;
use std::env::current_dir;

mod script;

fn main() -> Result<()> {
    run(
        "alpha-explorer",
        1024,
        768,
        true,
        current_dir()?.join("assets"),
        "assets/scripts/entry.lua",
        once_engine_initialized,
    )
    .block_on()
}

fn once_engine_initialized(_window: &Window, context: &EngineContext) -> Result<()> {
    context
        .script_mgr()
        .append_api_table::<GameModule>("game")?;
    Ok(())
}
