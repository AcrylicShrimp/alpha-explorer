use super::EntityBuilderParam;
use crate::{structure::Size, ui::UIScaleMode};
use anyhow::Context;
use mlua::prelude::*;

pub struct UIScalerParams {
    pub mode: UIScaleMode,
    pub reference_size: Size,
}

impl EntityBuilderParam for UIScalerParams {
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self> {
        Ok(Self {
            mode: table
                .get("mode")
                .with_context(|| "invalid value for 'mode' of UIScalerParams")
                .to_lua_err()?,
            reference_size: table
                .get("reference_size")
                .with_context(|| "invalid value for 'reference_size' of UIScalerParams")
                .to_lua_err()?,
        })
    }
}
