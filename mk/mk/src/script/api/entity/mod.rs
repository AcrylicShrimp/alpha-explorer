use crate::script::api::LuaApiTable;
use mlua::prelude::*;

mod entity;
mod entity_builder;
mod entity_builder_params;

pub use entity::*;
pub use entity_builder::*;
pub use entity_builder_params::*;

pub struct EntityModule;

impl LuaApiTable for EntityModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Entity", entity::Entity::create_api_table(lua)?)?;
        table.set(
            "EntityBuilder",
            entity_builder::EntityBuilder::create_api_table(lua)?,
        )?;

        Ok(table)
    }
}
