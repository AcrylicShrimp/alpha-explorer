use super::{entity::Entity, entity_builder_params::*};
use crate::{component::*, engine::use_context, script::api::ModuleType, structure::Vec2};
use parking_lot::Mutex;
use rhai::{ImmutableString, Map, Module};
use std::sync::Arc;

#[derive(Default)]
pub struct EntityBuilderImpl {
    name: Option<ImmutableString>,
    transform_parent: Option<Transform>,
    transform_position: Option<Vec2>,
    transform_scale: Option<Vec2>,
    transform_angle: Option<f32>,
    size: Option<crate::structure::Size>,
    alpha_tilemap_renderer_params: Option<AlphaTilemapRendererParams>,
    audio_source_params: Option<AudioSourceParams>,
    camera_params: Option<CameraParams>,
    is_diagnostic: bool,
    glyph_renderer_params: Option<GlyphRendererParams>,
    nine_patch_renderer_params: Option<NinePatchRendererParams>,
    sprite_renderer_params: Option<SpriteRendererParams>,
    tilemap_renderer_params: Option<TilemapRendererParams>,
    ui_element_params: Option<UIElementParams>,
    ui_scaler_params: Option<UIScalerParams>,
}

#[derive(Default, Clone)]
pub struct EntityBuilder(Arc<Mutex<EntityBuilderImpl>>);

impl EntityBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ModuleType for EntityBuilder {
    fn register(module: &mut Module) {
        module.set_custom_type::<EntityBuilder>("EntityBuilder");

        to_global!(
            module,
            module.set_native_fn("name", |this: Self, name| {
                this.0.lock().name = Some(name);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("transform_parent", |this: Self, transform_parent| {
                this.0.lock().transform_parent = Some(transform_parent);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("transform_position", |this: Self, transform_position| {
                this.0.lock().transform_position = Some(transform_position);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("transform_scale", |this: Self, transform_scale| {
                this.0.lock().transform_scale = Some(transform_scale);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("transform_angle", |this: Self, transform_angle| {
                this.0.lock().transform_angle = Some(transform_angle);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("size", |this: Self, size| {
                this.0.lock().size = Some(size);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("alpha_tilemap_renderer", |this: Self, params: Map| {
                this.0.lock().alpha_tilemap_renderer_params = Some(<_>::from_table(params)?);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("audio_source", |this: Self| {
                this.0.lock().audio_source_params = Some(<_>::default());
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("audio_source", |this: Self, params: Map| {
                this.0.lock().audio_source_params = Some(<_>::from_table(params)?);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("camera", |this: Self, params: Map| {
                this.0.lock().camera_params = Some(<_>::from_table(params)?);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("diagnostic", |this: Self| {
                this.0.lock().is_diagnostic = true;
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("glyph_renderer", |this: Self, params: Map| {
                this.0.lock().glyph_renderer_params = Some(<_>::from_table(params)?);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("nine_patch_renderer", |this: Self, params: Map| {
                this.0.lock().nine_patch_renderer_params = Some(<_>::from_table(params)?);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("sprite_renderer", |this: Self, params: Map| {
                this.0.lock().sprite_renderer_params = Some(<_>::from_table(params)?);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("ui_element", |this: Self, params: Map| {
                this.0.lock().ui_element_params = Some(<_>::from_table(params)?);
                Ok(this)
            })
        );
        to_global!(
            module,
            module.set_native_fn("ui_scaler", |this: Self, params: Map| {
                this.0.lock().ui_scaler_params = Some(<_>::from_table(params)?);
                Ok(this)
            })
        );

        to_global!(
            module,
            module.set_native_fn("build", |this: &mut Self| {
                let mut this = this.0.lock();

                let context = use_context();
                let mut world = context.world_mut();
                let entity = world.push(());
                let mut entry = world.entry(entity).unwrap();

                let mut transform_mgr = context.transform_mgr_mut();
                let transform = transform_mgr.alloc(entity);
                transform_mgr
                    .set_name(transform, this.name.take().map(|name| name.as_str().into()));

                entry.add_component(Transform::new(transform));

                transform_mgr.set_parent(
                    transform,
                    this.transform_parent.map(|parent| parent.index()),
                );

                {
                    let transform = transform_mgr.transform_mut(transform);
                    transform.mark_as_dirty();

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
                    size.width = param.width;
                    size.height = param.height;
                }

                entry.add_component(size);

                if let Some(param) = this.alpha_tilemap_renderer_params.take() {
                    let alpha_tilemap_renderer = AlphaTilemapRenderer::new(
                        param.layer,
                        param.order,
                        param.color,
                        param.fore_shader,
                        param.back_shader,
                        param.font,
                        param.font_size,
                        param.thickness,
                        param.smoothness,
                        param.tilemap,
                    );
                    entry.add_component(alpha_tilemap_renderer);
                }

                if let Some(param) = this.audio_source_params.take() {
                    let mut audio_source = AudioSource::new();

                    if let Some(volume) = param.volume {
                        audio_source.set_volume(volume);
                    }

                    audio_source.set_clip(param.clip);

                    entry.add_component(audio_source);
                }

                if let Some(param) = this.camera_params.take() {
                    let camera = Camera::new(param.layer, param.order);
                    entry.add_component(camera);
                }

                if this.is_diagnostic {
                    entry.add_component(Diagnostic);
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
                        glyph_renderer.set_text(text.as_str().to_owned());
                    }

                    if let Some(config) = param.config {
                        glyph_renderer.set_config(config);
                    }

                    entry.add_component(glyph_renderer);
                }

                if let Some(param) = this.nine_patch_renderer_params.take() {
                    let nine_patch_renderer = NinePatchRenderer::new(
                        param.layer,
                        param.order,
                        param.color,
                        param.shader,
                        param.nine_patch,
                    );

                    entry.add_component(nine_patch_renderer);
                }

                if let Some(param) = this.sprite_renderer_params.take() {
                    let sprite_renderer = SpriteRenderer::new(
                        param.layer,
                        param.order,
                        param.color,
                        param.shader,
                        param.sprite,
                    );

                    entry.add_component(sprite_renderer);
                }

                if let Some(param) = this.tilemap_renderer_params.take() {
                    let tilemap_renderer = TilemapRenderer::new(
                        param.layer,
                        param.order,
                        param.color,
                        param.shader,
                        param.tilemap,
                    );

                    entry.add_component(tilemap_renderer);
                }

                if let Some(param) = this.ui_element_params.take() {
                    let mut ui_mgr = context.ui_mgr_mut();
                    let index = ui_mgr.alloc(entity);
                    let ui_element = ui_mgr.element_mut(index);
                    ui_element.mark_as_dirty();

                    ui_element.anchor = param.anchor;
                    ui_element.margin = param.margin;
                    ui_element.set_interactible(param.is_interactible.unwrap_or(true));
                    ui_element.set_order_index(param.order_index);

                    entry.add_component(UIElement::new(index));
                }

                Ok(Entity::new(entity))
            })
        );

        module.set_sub_module("EntityBuilder", {
            let mut sub_module = Module::new();

            sub_module.set_native_fn("create", || Ok(Self::new()));

            sub_module
        });
    }
}
