use crate::{
    component::*,
    engine::use_context,
    script::api::{component::*, ModuleType, OptionToDynamic},
};
use legion::{
    world::{EntityAccessError, EntryMut, EntryRef},
    EntityStore,
};
use rhai::{ImmutableString, Module};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub legion::Entity);

impl Entity {
    pub fn new(entity: legion::Entity) -> Self {
        Self(entity)
    }

    pub fn with_ref<R>(self, f: impl FnOnce(EntryRef) -> R) -> Option<R> {
        let world = use_context().world();
        let entry = match world.entry_ref(self.0) {
            Ok(entry) => entry,
            Err(err) => match err {
                EntityAccessError::AccessDenied => {
                    panic!("failed to access ref of entity {:?}", self.0)
                }
                EntityAccessError::EntityNotFound => return None,
            },
        };
        Some(f(entry))
    }

    pub fn with_mut<R>(self, f: impl FnOnce(EntryMut) -> R) -> Option<R> {
        let mut world = use_context().world_mut();
        let entry = match world.entry_mut(self.0) {
            Ok(entry) => entry,
            Err(err) => match err {
                EntityAccessError::AccessDenied => {
                    panic!("failed to access mut ref of entity {:?}", self.0)
                }
                EntityAccessError::EntityNotFound => return None,
            },
        };
        Some(f(entry))
    }
}

impl ModuleType for Entity {
    fn register(module: &mut Module) {
        module.set_custom_type::<Entity>("Entity");

        module.set_getter_fn("name", |this: &mut Self| {
            Ok(this
                .with_ref(|this| {
                    this.get_component::<Transform>()
                        .ok()
                        .map(|component| component.index())
                })
                .flatten()
                .map(|index| {
                    use_context()
                        .transform_mgr()
                        .name(index)
                        .map(|name| name.to_owned())
                })
                .flatten()
                .to_dynamic())
        });
        module.set_setter_fn("name", |this: &mut Self, _: ()| {
            this.with_ref(|this| {
                this.get_component::<Transform>()
                    .ok()
                    .map(|component| component.index())
            })
            .flatten()
            .map(|index| use_context().transform_mgr_mut().set_name(index, None));
            Ok(())
        });
        module.set_setter_fn("name", |this: &mut Self, name: ImmutableString| {
            this.with_ref(|this| {
                this.get_component::<Transform>()
                    .ok()
                    .map(|component| component.index())
            })
            .flatten()
            .map(|index| {
                use_context()
                    .transform_mgr_mut()
                    .set_name(index, Some(name.as_str().into()))
            });
            Ok(())
        });

        module.set_getter_fn("alpha_tilemap_renderer", |this: &mut Self| {
            Ok(ComponentAlphaTilemapRenderer::new(this.0))
        });
        module.set_getter_fn("audio_source", |this: &mut Self| {
            Ok(ComponentAudioSource::new(this.0))
        });
        module.set_getter_fn("camera", |this: &mut Self| Ok(ComponentCamera::new(this.0)));
        module.set_getter_fn("diagnostic", |this: &mut Self| {
            Ok(ComponentDiagnostic::new(this.0))
        });
        module.set_getter_fn("glyph_renderer", |this: &mut Self| {
            Ok(ComponentGlyphRenderer::new(this.0))
        });
        module.set_getter_fn("nine_patch_renderer", |this: &mut Self| {
            Ok(ComponentNinePatchRenderer::new(this.0))
        });
        module.set_getter_fn("size", |this: &mut Self| Ok(ComponentSize::new(this.0)));
        module.set_getter_fn("sprite_renderer", |this: &mut Self| {
            Ok(ComponentSpriteRenderer::new(this.0))
        });
        module.set_getter_fn("tilemap_renderer", |this: &mut Self| {
            Ok(ComponentTilemapRenderer::new(this.0))
        });
        module.set_getter_fn("transform", |this: &mut Self| {
            Ok(this
                .with_ref(|this| {
                    this.get_component::<Transform>()
                        .ok()
                        .map(|component| ComponentTransform::new(component.index()))
                })
                .flatten()
                .to_dynamic())
        });
        module.set_getter_fn("ui_element", |this: &mut Self| {
            Ok(ComponentUIElement::new(this.0))
        });
        module.set_getter_fn("ui_scaler", |this: &mut Self| {
            Ok(ComponentUIScaler::new(this.0))
        });

        // TODO: Implement below functions
        to_global!(
            module,
            module.set_native_fn("listen", |this: &mut Self| { Ok(()) })
        );
        to_global!(
            module,
            module.set_native_fn("unlisten", |this: &mut Self| { Ok(()) })
        );

        module.set_sub_module("Entity", {
            let mut sub_module = Module::new();

            sub_module.set_native_fn("find_by_name", |name: ImmutableString| {
                let transform_mgr = use_context().transform_mgr();
                Ok(transform_mgr
                    .find_by_name(name)
                    .map(|index| Self::new(transform_mgr.entity(index)))
                    .to_dynamic())
            });
            sub_module.set_native_fn("find_all_by_name", |name: ImmutableString| {
                let transform_mgr = use_context().transform_mgr();
                Ok(transform_mgr.find_all_by_name(name).map(|indices| {
                    indices
                        .iter()
                        .map(|index| Self::new(transform_mgr.entity(*index)))
                        .collect::<Vec<_>>()
                }))
            });

            sub_module
        });
    }
}
