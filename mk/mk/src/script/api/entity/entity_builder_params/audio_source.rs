use super::EntityBuilderParam;
use crate::handles::*;
use anyhow::Context;
use mlua::prelude::*;

#[derive(Default)]
pub struct AudioSourceParams {
    pub volume: Option<f32>,
    pub clip: Option<AudioClipHandle>,
}

impl EntityBuilderParam for AudioSourceParams {
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self> {
        Ok(Self {
            volume: table
                .get("volume")
                .with_context(|| "invalid value for 'volume' of AudioSourceParams")
                .to_lua_err()?,
            clip: table
                .get("clip")
                .with_context(|| "invalid value for 'clip' of AudioSourceParams")
                .to_lua_err()?,
        })
    }
}
