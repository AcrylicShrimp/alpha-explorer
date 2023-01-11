use crate::script::{
    api::IntoShared,
    gfx::{Shader, SpriteNinePatch},
};
use mlua::prelude::*;

pub type ComponentNinePatchRenderer = super::Component<crate::component::NinePatchRenderer>;

impl LuaUserData for ComponentNinePatchRenderer {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("layer", |_lua, this| Ok(this.with_ref(|this| this.layer)));
        fields.add_field_method_get("order", |_lua, this| Ok(this.with_ref(|this| this.order)));
        fields.add_field_method_get("color", |_lua, this| Ok(this.with_ref(|this| this.color)));
        fields.add_field_method_get("shader", |_lua, this| {
            Ok(this.with_ref(|this| this.shader.clone().into_shared()))
        });
        fields.add_field_method_get("nine_patch", |_lua, this| {
            Ok(this.with_ref(|this| this.nine_patch.clone().into_shared()))
        });

        fields.add_field_method_set("layer", |_lua, this, layer| {
            this.with_mut(|this| {
                this.layer = layer;
            });
            Ok(())
        });
        fields.add_field_method_set("order", |_lua, this, order| {
            this.with_mut(|this| {
                this.order = order;
            });
            Ok(())
        });
        fields.add_field_method_set("color", |_lua, this, color| {
            this.with_mut(|this| {
                this.color = color;
            });
            Ok(())
        });
        fields.add_field_method_set("shader", |_lua, this, shader: Shader| {
            this.with_mut(|this| {
                this.shader = shader.into_inner();
            });
            Ok(())
        });
        fields.add_field_method_set("nine_patch", |_lua, this, nine_patch: SpriteNinePatch| {
            this.with_mut(|this| {
                this.nine_patch = nine_patch.into_inner();
            });
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentNinePatchRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });
    }
}
