use crate::codegen_traits::LuaApiTable;
use mlua::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl<'lua> FromLua<'lua> for Color {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(value) => Ok(Self {
                r: value.get("r")?,
                g: value.get("g")?,
                b: value.get("b")?,
                a: value.get("a")?,
            }),
            _ => {
                return Err(format!("the type {} must be a {}", "Color", "table").to_lua_err());
            }
        }
    }
}

impl<'lua> ToLua<'lua> for Color {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::Table(lua.create_table_from([
            ("r", self.r),
            ("g", self.g),
            ("b", self.b),
            ("a", self.a),
        ])?))
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
