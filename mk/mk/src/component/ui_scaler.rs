use crate::structure::Size;
use codegen::{Animation, LuaComponent};
use mlua::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UIScaleMode {
    Constant,
    Stretch,
    Fit,
    Fill,
    MatchWidth,
    MatchHeight,
}

impl<'lua> FromLua<'lua> for UIScaleMode {
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let str = String::from_lua(value, lua)?;
        let str = str.as_str();
        match str {
            "constant" => Ok(UIScaleMode::Constant),
            "stretch" => Ok(UIScaleMode::Stretch),
            "fit" => Ok(UIScaleMode::Fit),
            "fill" => Ok(UIScaleMode::Fill),
            "match-width" => Ok(UIScaleMode::MatchWidth),
            "match-height" => Ok(UIScaleMode::MatchHeight),
            _ => Err(
                format!("{:?} is invalid value for the type {}", str, "UIScaleMode",).to_lua_err(),
            ),
        }
    }
}

impl<'lua> ToLua<'lua> for UIScaleMode {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::String(lua.create_string(match self {
            UIScaleMode::Constant => "constant",
            UIScaleMode::Stretch => "stretch",
            UIScaleMode::Fit => "fit",
            UIScaleMode::Fill => "fill",
            UIScaleMode::MatchWidth => "match-width",
            UIScaleMode::MatchHeight => "match-height",
        })?))
    }
}

#[derive(Animation, LuaComponent)]
pub struct UIScaler {
    pub mode: UIScaleMode,
    pub reference_size: Size,
}

impl UIScaler {
    pub fn new(mode: UIScaleMode, reference_size: Size) -> Self {
        Self {
            mode,
            reference_size,
        }
    }
}
