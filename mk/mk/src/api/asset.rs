use crate::api::use_context;
use crate::codegen_traits::LuaApiTable;
use crate::render::{
    LuaFontHandle, LuaRcSprite, LuaRcSpriteAtlas, LuaRcSpriteAtlasGrid, LuaRcSpriteNinePatch,
    LuaShaderHandle,
};
use mlua::prelude::*;

pub struct FontAsset;

impl LuaApiTable for FontAsset {
    fn api_name() -> &'static str {
        "Font"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "load",
            lua.create_function(|_, name: String| {
                let asset_mgr = use_context().asset_mgr();
                let asset: LuaFontHandle = asset_mgr.load(&name).map_err(|err| {
                    format!("unable to load font '{}' due to: {}", name, err).to_lua_err()
                })?;
                Ok(asset)
            })?,
        )?;
        Ok(())
    }
}

pub struct ShaderAsset;

impl LuaApiTable for ShaderAsset {
    fn api_name() -> &'static str {
        "Shader"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "load",
            lua.create_function(|_, name: String| {
                let asset_mgr = use_context().asset_mgr();
                let asset: LuaShaderHandle = asset_mgr.load(&name).map_err(|err| {
                    format!("unable to load shader '{}' due to: {}", name, err).to_lua_err()
                })?;
                Ok(asset)
            })?,
        )?;
        Ok(())
    }
}

pub struct SpriteAsset;

impl LuaApiTable for SpriteAsset {
    fn api_name() -> &'static str {
        "Sprite"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "load",
            lua.create_function(|_, name: String| {
                let asset_mgr = use_context().asset_mgr();
                let asset: LuaRcSprite = asset_mgr.load(&name).map_err(|err| {
                    format!("unable to load sprite '{}' due to: {}", name, err).to_lua_err()
                })?;
                Ok(asset)
            })?,
        )?;
        Ok(())
    }
}

pub struct SpriteAtlasAsset;

impl LuaApiTable for SpriteAtlasAsset {
    fn api_name() -> &'static str {
        "SpriteAtlas"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "load",
            lua.create_function(|_, name: String| {
                let asset_mgr = use_context().asset_mgr();
                let asset: LuaRcSpriteAtlas = asset_mgr.load(&name).map_err(|err| {
                    format!("unable to load sprite atlas '{}' due to: {}", name, err).to_lua_err()
                })?;
                Ok(asset)
            })?,
        )?;
        Ok(())
    }
}

pub struct SpriteAtlasGridAsset;

impl LuaApiTable for SpriteAtlasGridAsset {
    fn api_name() -> &'static str {
        "SpriteAtlasGrid"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "load",
            lua.create_function(|_, name: String| {
                let asset_mgr = use_context().asset_mgr();
                let asset: LuaRcSpriteAtlasGrid = asset_mgr.load(&name).map_err(|err| {
                    format!(
                        "unable to load sprite atlas grid '{}' due to: {}",
                        name, err
                    )
                    .to_lua_err()
                })?;
                Ok(asset)
            })?,
        )?;
        Ok(())
    }
}

pub struct SpriteNinePatchAsset;

impl LuaApiTable for SpriteNinePatchAsset {
    fn api_name() -> &'static str {
        "SpriteNinePatch"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "load",
            lua.create_function(|_, name: String| {
                let asset_mgr = use_context().asset_mgr();
                let asset: LuaRcSpriteNinePatch = asset_mgr.load(&name).map_err(|err| {
                    format!(
                        "unable to load sprite nine-patch '{}' due to: {}",
                        name, err
                    )
                    .to_lua_err()
                })?;
                Ok(asset)
            })?,
        )?;
        Ok(())
    }
}
