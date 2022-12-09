use crate::script::api::LuaApiTable;
use mlua::prelude::*;

mod mat22;
mod mat33;
mod size;
mod vec2;
mod vec3;

pub use mat22::*;
pub use mat33::*;
pub use size::*;
pub use vec2::*;
pub use vec3::*;

pub struct StructureModule;

impl LuaApiTable for StructureModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Mat22", mat22::Mat22::create_api_table(lua)?)?;
        table.set("Mat33", mat33::Mat33::create_api_table(lua)?)?;
        table.set("Size", size::Size::create_api_table(lua)?)?;
        table.set("Vec2", vec2::Vec2::create_api_table(lua)?)?;
        table.set("Vec3", vec3::Vec3::create_api_table(lua)?)?;

        Ok(table)
    }
}
