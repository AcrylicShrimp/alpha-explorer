use crate::script::{
    api::IntoShared,
    gfx::{Shader, Sprite},
};
use mlua::prelude::*;

pub type ComponentUIImageRenderer = super::Component<crate::component::UIImageRenderer>;

impl LuaUserData for ComponentUIImageRenderer {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("layer", |_lua, this| Ok(this.with_ref(|this| this.layer)));
        fields.add_field_method_get("color", |_lua, this| Ok(this.with_ref(|this| this.color)));
        fields.add_field_method_get("shader", |_lua, this| {
            Ok(this.with_ref(|this| this.shader.clone().into_shared()))
        });
        fields.add_field_method_get("image", |_lua, this| {
            Ok(this.with_ref(|this| this.image.clone().into_shared()))
        });
        fields.add_field_method_get("mode", |_lua, this| {
            Ok(this.with_ref(|this| this.image.clone().into_shared()))
        });
        fields.add_field_method_get("ignore_mask", |_lua, this| {
            Ok(this.with_ref(|this| this.ignore_mask))
        });

        fields.add_field_method_set("layer", |_lua, this, layer| {
            this.with_mut(|this| {
                this.layer = layer;
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
        fields.add_field_method_set("image", |_lua, this, image: Sprite| {
            this.with_mut(|this| {
                this.image = image.into_inner();
            });
            Ok(())
        });
        fields.add_field_method_set("ignore_mask", |_lua, this, ignore_mask| {
            this.with_mut(|this| {
                this.ignore_mask = ignore_mask;
            });
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentUIImageRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });
    }
}
