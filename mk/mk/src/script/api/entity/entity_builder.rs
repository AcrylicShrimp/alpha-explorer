use super::entity_builder_params::*;
use crate::{component::*, engine::use_context, script::api::LuaApiTable, structure::Vec2};
use mlua::prelude::*;
use parking_lot::Mutex;
use specs::prelude::*;
use std::sync::Arc;

#[derive(Default)]
pub struct EntityBuilderImpl {
    name: Option<String>,
    transform_parent: Option<Transform>,
    transform_position: Option<Vec2>,
    transform_scale: Option<Vec2>,
    transform_angle: Option<f32>,
    size: Option<crate::structure::Size>,
    // alpha_tilemap_renderer_params: Option<AlphaTilemapRendererParams>,
    audio_source_params: Option<AudioSourceParams>,
    camera_params: Option<CameraParams>,
    is_diagnostic: bool,
    glyph_renderer_params: Option<GlyphRendererParams>,
    sprite_renderer_params: Option<SpriteRendererParams>,
    // tilemap_renderer_params: Option<TilemapRendererParams>,
    ui_element_params: Option<UIElementParams>,
    ui_mask_params: Option<UIMaskParams>,
    ui_scaler_params: Option<UIScalerParams>,
}

#[derive(Default, Clone)]
pub struct EntityBuilder(Arc<Mutex<EntityBuilderImpl>>);

impl EntityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut EntityBuilderImpl) -> R) -> R {
        f(&mut self.0.lock())
    }
}

impl LuaApiTable for EntityBuilder {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("new", lua.create_function(|_lua, ()| Ok(Self::new()))?)?;

        Ok(table)
    }
}

impl LuaUserData for EntityBuilder {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("name", |_lua, this, name| {
            this.with_mut(|this| {
                this.name = name;
            });
            Ok(this.clone())
        });
        methods.add_method("transform_parent", |_lua, this, transform_parent| {
            this.with_mut(|this| {
                this.transform_parent = transform_parent;
            });
            Ok(this.clone())
        });
        methods.add_method("transform_position", |_lua, this, transform_position| {
            this.with_mut(|this| {
                this.transform_position = transform_position;
            });
            Ok(this.clone())
        });
        methods.add_method("transform_scale", |_lua, this, transform_scale| {
            this.with_mut(|this| {
                this.transform_scale = transform_scale;
            });
            Ok(this.clone())
        });
        methods.add_method("transform_angle", |_lua, this, transform_angle| {
            this.with_mut(|this| {
                this.transform_angle = transform_angle;
            });
            Ok(this.clone())
        });
        methods.add_method("size", |_lua, this, size| {
            this.with_mut(|this| {
                this.size = size;
            });
            Ok(this.clone())
        });
        // methods.add_method(
        //     "alpha_tilemap_renderer",
        //     |_lua, this, params: Option<LuaTable>| {
        //         this.with_mut(|this| -> LuaResult<_> {
        //             this.alpha_tilemap_renderer_params =
        //                 params.map(|params| <_>::from_table(params)).transpose()?;
        //             Ok(())
        //         })?;
        //         Ok(this.clone())
        //     },
        // );
        methods.add_method("audio_source", |_lua, this, params: Option<LuaTable>| {
            this.with_mut(|this| -> LuaResult<_> {
                this.audio_source_params =
                    params.map(|params| <_>::from_table(params)).transpose()?;
                Ok(())
            })?;
            Ok(this.clone())
        });
        methods.add_method("camera", |_lua, this, params: Option<LuaTable>| {
            this.with_mut(|this| -> LuaResult<_> {
                this.camera_params = params.map(|params| <_>::from_table(params)).transpose()?;
                Ok(())
            })?;
            Ok(this.clone())
        });
        methods.add_method("diagnostic", |_lua, this, is_diagnostic| {
            this.with_mut(|this| -> LuaResult<_> {
                this.is_diagnostic = is_diagnostic;
                Ok(())
            })?;
            Ok(this.clone())
        });
        methods.add_method("glyph_renderer", |_lua, this, params: Option<LuaTable>| {
            this.with_mut(|this| -> LuaResult<_> {
                this.glyph_renderer_params =
                    params.map(|params| <_>::from_table(params)).transpose()?;
                Ok(())
            })?;
            Ok(this.clone())
        });
        methods.add_method("sprite_renderer", |_lua, this, params: Option<LuaTable>| {
            this.with_mut(|this| -> LuaResult<_> {
                this.sprite_renderer_params =
                    params.map(|params| <_>::from_table(params)).transpose()?;
                Ok(())
            })?;
            Ok(this.clone())
        });
        methods.add_method("ui_element", |_lua, this, params: Option<LuaTable>| {
            this.with_mut(|this| -> LuaResult<_> {
                this.ui_element_params =
                    params.map(|params| <_>::from_table(params)).transpose()?;
                Ok(())
            })?;
            Ok(this.clone())
        });
        methods.add_method("ui_mask", |_lua, this, params: Option<LuaTable>| {
            this.with_mut(|this| -> LuaResult<_> {
                this.ui_mask_params = params.map(|params| <_>::from_table(params)).transpose()?;
                Ok(())
            })?;
            Ok(this.clone())
        });
        methods.add_method("ui_scaler", |_lua, this, params: Option<LuaTable>| {
            this.with_mut(|this| -> LuaResult<_> {
                this.ui_scaler_params = params.map(|params| <_>::from_table(params)).transpose()?;
                Ok(())
            })?;
            Ok(this.clone())
        });

        methods.add_method("build", |_lua, this, ()| {
            let mut this = this.0.lock();

            let context = use_context();
            let mut world = context.world_mut();
            let mut builder = world.create_entity();

            let mut transform_mgr = context.transform_mgr_mut();
            let transform = transform_mgr.alloc();
            builder = builder.with(Transform::new(transform));

            let mut render_mgr = context.render_mgr_mut();
            let mut glyph_mgr = context.glyph_mgr_mut();

            transform_mgr
                .name_manager_mut()
                .set_name(transform, this.name.take().map(|name| name.as_str().into()));

            if let Some(parent) = this.transform_parent {
                transform_mgr
                    .hierarchy_mut()
                    .set_parent(transform, Some(parent.index()));
            }

            {
                let transform = transform_mgr.allocator_mut().transform_mut(transform);

                if let Some(position) = this.transform_position {
                    transform.position = position;
                }

                if let Some(scale) = this.transform_scale {
                    transform.scale = scale;
                }

                if let Some(angle) = this.transform_angle {
                    transform.angle = angle;
                }
            }

            let mut size = Size::new(transform);

            if let Some(param) = this.size.take() {
                size.size = param;
            }

            builder = builder.with(size);

            // if let Some(param) = this.alpha_tilemap_renderer_params.take() {
            //     let alpha_tilemap_renderer = AlphaTilemapRenderer::new(
            //         param.layer,
            //         param.order,
            //         param.color,
            //         param.fore_shader,
            //         param.back_shader,
            //         param.font,
            //         param.font_size,
            //         param.thickness,
            //         param.smoothness,
            //         param.tilemap,
            //     );
            //     builder = builder.with(alpha_tilemap_renderer);
            // }

            if let Some(param) = this.audio_source_params.take() {
                let mut audio_source = AudioSource::new();

                if let Some(volume) = param.volume {
                    audio_source.set_volume(volume);
                }

                audio_source.set_clip(param.clip);

                builder = builder.with(audio_source);
            }

            if let Some(param) = this.camera_params.take() {
                let camera = Camera::new(
                    &render_mgr,
                    param.layer,
                    param.order,
                    param.clear_mode,
                    param.clear_color,
                );
                builder = builder.with(camera);
            }

            if this.is_diagnostic {
                builder = builder.with(Diagnostic);
            }

            if let Some(param) = this.glyph_renderer_params.take() {
                let mut glyph_renderer = GlyphRenderer::new(
                    param.layer,
                    param.order,
                    param.color,
                    param.shader,
                    param.thickness,
                    param.smoothness,
                    param.font,
                    param.font_size,
                );

                if let Some(text) = param.text {
                    glyph_renderer.set_text(
                        &mut glyph_mgr,
                        &mut render_mgr,
                        text.as_str().to_owned(),
                    );
                }

                if let Some(config) = param.config {
                    glyph_renderer.set_config(config);
                }

                builder = builder.with(glyph_renderer);
            }

            if let Some(param) = this.sprite_renderer_params.take() {
                let sprite_renderer = SpriteRenderer::new(
                    &mut render_mgr,
                    param.layer,
                    param.order,
                    param.color,
                    param.shader,
                    param.sprite,
                );

                builder = builder.with(sprite_renderer);
            }

            // if let Some(param) = this.tilemap_renderer_params.take() {
            //     let tilemap_renderer = TilemapRenderer::new(
            //         param.layer,
            //         param.order,
            //         param.color,
            //         param.shader,
            //         param.tilemap,
            //     );

            //     builder = builder.with(tilemap_renderer);
            // }

            let mut ui_element_index = None;

            if let Some(param) = this.ui_element_params.take() {
                let mut ui_mgr = context.ui_mgr_mut();
                let index = ui_mgr.begin_alloc();
                let ui_element = ui_mgr.element_mut(index);
                ui_element.mark_as_dirty();

                ui_element.anchor = param.anchor;
                ui_element.margin = param.margin;
                ui_element.set_interactible(param.is_interactible.unwrap_or(true));
                ui_element.set_order_index(param.order_index);

                builder = builder.with(UIElement::new(index));

                ui_element_index = Some(index);
            }

            if let Some(param) = this.ui_mask_params.take() {
                let ui_mask = UIMask {
                    render_itself: param.render_itself,
                };

                builder = builder.with(ui_mask);
            }

            if let Some(param) = this.ui_scaler_params.take() {
                let ui_scaler = UIScaler {
                    mode: param.mode,
                    reference_size: param.reference_size,
                };

                builder = builder.with(ui_scaler);
            }

            let entity = builder.build();
            transform_mgr.alloc_entity(transform, entity);

            if let Some(index) = ui_element_index {
                context.ui_mgr_mut().fin_alloc(index, entity);
            }

            Ok(super::entity::Entity::new(entity))
        });
    }
}
