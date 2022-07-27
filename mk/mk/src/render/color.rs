use crate::codegen_traits::LuaApiTable;
use codegen::LuaStruct;
use mlua::prelude::*;

#[derive(LuaStruct, Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new() -> Self {
        Self {
            r: 0f32,
            g: 0f32,
            b: 0f32,
            a: 0f32,
        }
    }

    pub fn white() -> Self {
        Self {
            r: 1f32,
            g: 1f32,
            b: 1f32,
            a: 1f32,
        }
    }

    pub fn black() -> Self {
        Self {
            r: 0f32,
            g: 0f32,
            b: 0f32,
            a: 1f32,
        }
    }
}

impl LuaApiTable for Color {
    fn api_name() -> &'static str {
        "Color"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "rgb",
            LuaValue::Function(lua.create_function(|_lua, (r, g, b): (f32, f32, f32)| {
                Ok(Color { r, g, b, a: 1f32 })
            })?),
        )?;
        table.set(
            "rgba",
            LuaValue::Function(lua.create_function(
                |_lua, (r, g, b, a): (f32, f32, f32, f32)| Ok(Color { r, g, b, a }),
            )?),
        )?;
        table.set(
            "white",
            LuaValue::Function(lua.create_function(|_lua, ()| Ok(Color::white()))?),
        )?;
        table.set(
            "black",
            LuaValue::Function(lua.create_function(|_lua, ()| Ok(Color::black()))?),
        )?;
        Ok(())
    }
}
