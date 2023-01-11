use mlua::prelude::*;

pub trait LuaApiTable {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>>;
}

pub mod asset;
pub mod audio;
pub mod component;
pub mod entity;
pub mod event;
pub mod gfx;
pub mod screen;
pub mod structure;
pub mod time;
pub mod ui;

pub struct Module;

impl LuaApiTable for Module {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("asset", asset::AssetModule::create_api_table(lua)?)?;
        table.set("audio", audio::AudioModule::create_api_table(lua)?)?;
        table.set(
            "component",
            component::ComponentModule::create_api_table(lua)?,
        )?;
        table.set("entity", entity::EntityModule::create_api_table(lua)?)?;
        table.set("event", event::EventModule::create_api_table(lua)?)?;
        table.set("gfx", gfx::GfxModule::create_api_table(lua)?)?;
        table.set("screen", screen::ScreenModule::create_api_table(lua)?)?;
        table.set(
            "structure",
            structure::StructureModule::create_api_table(lua)?,
        )?;
        table.set("time", time::TimeModule::create_api_table(lua)?)?;
        table.set("ui", ui::UIModule::create_api_table(lua)?)?;

        Ok(table)
    }
}
