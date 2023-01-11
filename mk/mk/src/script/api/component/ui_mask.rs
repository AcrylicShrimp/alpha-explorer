use mlua::prelude::*;

pub type ComponentUIMask = super::Component<crate::component::UIMask>;

impl LuaUserData for ComponentUIMask {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("render_itself", |_lua, this| {
            Ok(this.with_ref(|this| this.render_itself))
        });

        fields.add_field_method_set("render_itself", |_lua, this, render_itself| {
            this.with_mut(|this| {
                this.render_itself = render_itself;
            });
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentUIMask(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });
    }
}
