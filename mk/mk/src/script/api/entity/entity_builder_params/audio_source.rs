use super::EntityBuilderParam;
use crate::audio::AudioClip;
use anyhow::Context;
use mlua::prelude::*;
use std::sync::Arc;

#[derive(Default)]
pub struct AudioSourceParams {
    pub volume: Option<f32>,
    pub clip: Option<Arc<AudioClip>>,
}

impl EntityBuilderParam for AudioSourceParams {
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self> {
        Ok(Self {
            volume: table
                .get("volume")
                .with_context(|| "invalid value for 'volume' of AudioSourceParams")
                .to_lua_err()?,
            clip: table
                .get::<_, Option<crate::script::api::audio::AudioClip>>("clip")
                .with_context(|| "invalid value for 'clip' of AudioSourceParams")
                .to_lua_err()?
                .map(|clip| clip.into_inner()),
        })
    }
}
