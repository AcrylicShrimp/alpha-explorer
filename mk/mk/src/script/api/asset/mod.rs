use crate::{
    audio::AudioClip,
    emit_diagnostic_warn,
    engine::use_context,
    render::{Shader, Sprite, SpriteAtlas, SpriteAtlasGrid, SpriteNinePatch, Tilemap},
    script::api::{ModuleType, OptionToDynamic},
};
use fontdue::Font;
use rhai::{ImmutableString, Module};
use std::sync::Arc;

pub struct AssetModule;

impl ModuleType for AssetModule {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        sub_module.set_native_fn("load_audio_clip", |path: ImmutableString| {
            Ok(match use_context()
                .asset_mgr()
                .load::<Arc<AudioClip>>(path.as_str())
            {
                Ok(asset) => Some(asset),
                Err(err) => {
                    emit_diagnostic_warn!(format!(
                        "failed to load audio clip from {} due to: {}",
                        path, err
                    ));
                    None
                }
            }
            .to_dynamic())
        });
        sub_module.set_native_fn("load_font", |path: ImmutableString| {
            Ok(
                match use_context().asset_mgr().load::<Arc<Font>>(path.as_str()) {
                    Ok(asset) => Some(asset),
                    Err(err) => {
                        emit_diagnostic_warn!(format!(
                            "failed to load font from {} due to: {}",
                            path, err
                        ));
                        None
                    }
                }
                .to_dynamic(),
            )
        });
        sub_module.set_native_fn("load_shader", |path: ImmutableString| {
            Ok(
                match use_context().asset_mgr().load::<Arc<Shader>>(path.as_str()) {
                    Ok(asset) => Some(asset),
                    Err(err) => {
                        emit_diagnostic_warn!(format!(
                            "failed to load shader from {} due to: {}",
                            path, err
                        ));
                        None
                    }
                }
                .to_dynamic(),
            )
        });
        sub_module.set_native_fn("load_sprite", |path: ImmutableString| {
            Ok(
                match use_context().asset_mgr().load::<Arc<Sprite>>(path.as_str()) {
                    Ok(asset) => Some(asset),
                    Err(err) => {
                        emit_diagnostic_warn!(format!(
                            "failed to load sprite from {} due to: {}",
                            path, err
                        ));
                        None
                    }
                }
                .to_dynamic(),
            )
        });
        sub_module.set_native_fn("load_sprite_atlas", |path: ImmutableString| {
            Ok(match use_context()
                .asset_mgr()
                .load::<Arc<SpriteAtlas>>(path.as_str())
            {
                Ok(asset) => Some(asset),
                Err(err) => {
                    emit_diagnostic_warn!(format!(
                        "failed to load sprite atlas from {} due to: {}",
                        path, err
                    ));
                    None
                }
            }
            .to_dynamic())
        });
        sub_module.set_native_fn("load_sprite_atlas_grid", |path: ImmutableString| {
            Ok(match use_context()
                .asset_mgr()
                .load::<Arc<SpriteAtlasGrid>>(path.as_str())
            {
                Ok(asset) => Some(asset),
                Err(err) => {
                    emit_diagnostic_warn!(format!(
                        "failed to load sprite atlas grid from {} due to: {}",
                        path, err
                    ));
                    None
                }
            }
            .to_dynamic())
        });
        sub_module.set_native_fn("load_sprite_nine_patch", |path: ImmutableString| {
            Ok(match use_context()
                .asset_mgr()
                .load::<Arc<SpriteNinePatch>>(path.as_str())
            {
                Ok(asset) => Some(asset),
                Err(err) => {
                    emit_diagnostic_warn!(format!(
                        "failed to load sprite nine patch from {} due to: {}",
                        path, err
                    ));
                    None
                }
            }
            .to_dynamic())
        });
        sub_module.set_native_fn("load_tilemap", |path: ImmutableString| {
            Ok(match use_context()
                .asset_mgr()
                .load::<Arc<Tilemap>>(path.as_str())
            {
                Ok(asset) => Some(asset),
                Err(err) => {
                    emit_diagnostic_warn!(format!(
                        "failed to load tilemap from {} due to: {}",
                        path, err
                    ));
                    None
                }
            }
            .to_dynamic())
        });

        module.set_sub_module("Asset", sub_module);
    }
}
