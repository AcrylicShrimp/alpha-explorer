use crate::{engine::use_context, handles::*};
use mlua::prelude::*;

pub type ComponentSpriteRenderer = super::Component<crate::component::SpriteRenderer>;

impl LuaUserData for ComponentSpriteRenderer {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("layer", |_lua, this| Ok(this.with_ref(|this| this.layer)));
        fields.add_field_method_get("order", |_lua, this| Ok(this.with_ref(|this| this.order)));
        fields.add_field_method_get("color", |_lua, this| Ok(this.with_ref(|this| this.color)));
        fields.add_field_method_get("shader", |_lua, this| {
            Ok(this.with_ref(|this| this.shader.clone()))
        });
        fields.add_field_method_get("sprite", |_lua, this| {
            Ok(this.with_ref(|this| this.sprite().clone()))
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
        fields.add_field_method_set("shader", |_lua, this, shader: ShaderHandle| {
            this.with_mut(|this| {
                this.shader = shader;
            });
            Ok(())
        });
        fields.add_field_method_set("sprite", |_lua, this, sprite: SpriteHandle| {
            this.with_mut(|this| {
                this.set_sprite(&mut use_context().render_mgr_mut(), sprite);
            });
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentSpriteRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });
    }
}
