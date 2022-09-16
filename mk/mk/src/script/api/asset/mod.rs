use crate::{
    audio::AudioClip,
    emit_diagnostic_warn,
    engine::use_context,
    render::{Shader, Sprite, SpriteAtlas, SpriteAtlasGrid, SpriteNinePatch, Tilemap},
    script::api::{IntoShared, LuaApiTable},
};
use fontdue::Font;
use mlua::prelude::*;
use std::sync::Arc;

pub struct AssetModule;

impl LuaApiTable for AssetModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "load_audio_clip",
            lua.create_function(|_lua, path: LuaString| {
                let path = path.to_str()?;
                Ok(
                    match use_context().asset_mgr().load::<Arc<AudioClip>>(path) {
                        Ok(asset) => Some(asset.into_shared()),
                        Err(err) => {
                            emit_diagnostic_warn!(format!(
                                "failed to load audio clip from {} due to: {}",
                                path, err
                            ));
                            None
                        }
                    },
                )
            })?,
        )?;
        table.set(
            "load_font",
            lua.create_function(|_lua, path: LuaString| {
                let path = path.to_str()?;
                Ok(match use_context().asset_mgr().load::<Arc<Font>>(path) {
                    Ok(asset) => Some(asset.into_shared()),
                    Err(err) => {
                        emit_diagnostic_warn!(format!(
                            "failed to load font from {} due to: {}",
                            path, err
                        ));
                        None
                    }
                })
            })?,
        )?;
        table.set(
            "load_shader",
            lua.create_function(|_lua, path: LuaString| {
                let path = path.to_str()?;
                Ok(match use_context().asset_mgr().load::<Arc<Shader>>(path) {
                    Ok(asset) => Some(asset.into_shared()),
                    Err(err) => {
                        emit_diagnostic_warn!(format!(
                            "failed to load shader from {} due to: {}",
                            path, err
                        ));
                        None
                    }
                })
            })?,
        )?;
        table.set(
            "load_sprite",
            lua.create_function(|_lua, path: LuaString| {
                let path = path.to_str()?;
                Ok(match use_context().asset_mgr().load::<Arc<Sprite>>(path) {
                    Ok(asset) => Some(asset.into_shared()),
                    Err(err) => {
                        emit_diagnostic_warn!(format!(
                            "failed to load sprite from {} due to: {}",
                            path, err
                        ));
                        None
                    }
                })
            })?,
        )?;
        table.set(
            "load_sprite_atlas",
            lua.create_function(|_lua, path: LuaString| {
                let path = path.to_str()?;
                Ok(
                    match use_context().asset_mgr().load::<Arc<SpriteAtlas>>(path) {
                        Ok(asset) => Some(asset.into_shared()),
                        Err(err) => {
                            emit_diagnostic_warn!(format!(
                                "failed to load sprite atlas from {} due to: {}",
                                path, err
                            ));
                            None
                        }
                    },
                )
            })?,
        )?;
        table.set(
            "load_sprite_atlas_grid",
            lua.create_function(|_lua, path: LuaString| {
                let path = path.to_str()?;
                Ok(
                    match use_context().asset_mgr().load::<Arc<SpriteAtlasGrid>>(path) {
                        Ok(asset) => Some(asset.into_shared()),
                        Err(err) => {
                            emit_diagnostic_warn!(format!(
                                "failed to load sprite atlas grid from {} due to: {}",
                                path, err
                            ));
                            None
                        }
                    },
                )
            })?,
        )?;
        table.set(
            "load_sprite_nine_patch",
            lua.create_function(|_lua, path: LuaString| {
                let path = path.to_str()?;
                Ok(
                    match use_context().asset_mgr().load::<Arc<SpriteNinePatch>>(path) {
                        Ok(asset) => Some(asset.into_shared()),
                        Err(err) => {
                            emit_diagnostic_warn!(format!(
                                "failed to load sprite nine patch from {} due to: {}",
                                path, err
                            ));
                            None
                        }
                    },
                )
            })?,
        )?;
        table.set(
            "load_tilemap",
            lua.create_function(|_lua, path: LuaString| {
                let path = path.to_str()?;
                Ok(match use_context().asset_mgr().load::<Arc<Tilemap>>(path) {
                    Ok(asset) => Some(asset.into_shared()),
                    Err(err) => {
                        emit_diagnostic_warn!(format!(
                            "failed to load tilemap from {} due to: {}",
                            path, err
                        ));
                        None
                    }
                })
            })?,
        )?;

        Ok(table)
    }
}
