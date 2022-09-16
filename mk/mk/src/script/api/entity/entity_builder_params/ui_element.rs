use super::EntityBuilderParam;
use crate::ui::{UIAnchor, UIMargin};
use anyhow::Context;
use mlua::prelude::*;

pub struct UIElementParams {
    pub anchor: UIAnchor,
    pub margin: UIMargin,
    pub is_interactible: Option<bool>,
    pub order_index: u32,
}

impl EntityBuilderParam for UIElementParams {
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self> {
        Ok(Self {
            anchor: table
                .get("anchor")
                .with_context(|| "invalid value for 'anchor' of UIElementParams")
                .to_lua_err()?,
            margin: table
                .get("margin")
                .with_context(|| "invalid value for 'margin' of UIElementParams")
                .to_lua_err()?,
            is_interactible: table
                .get("is_interactible")
                .with_context(|| "invalid value for 'is_interactible' of UIElementParams")
                .to_lua_err()?,
            order_index: table
                .get("order_index")
                .with_context(|| "invalid value for 'order_index' of UIElementParams")
                .to_lua_err()?,
        })
    }
}
