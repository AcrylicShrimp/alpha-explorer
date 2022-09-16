use mlua::prelude::*;

pub type ComponentCamera = super::Component<crate::component::Camera>;

impl LuaUserData for ComponentCamera {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("layer", |_lua, this| Ok(this.with_ref(|this| this.layer)));
        fields.add_field_method_get("order", |_lua, this| Ok(this.with_ref(|this| this.order)));

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
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentCamera(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });
    }
}
