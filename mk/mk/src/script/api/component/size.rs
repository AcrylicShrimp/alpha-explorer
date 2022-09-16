use mlua::prelude::*;

pub type ComponentSize = super::Component<crate::component::Size>;

impl LuaUserData for ComponentSize {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |_lua, this| Ok(this.with_ref(|this| this.size)));

        fields.add_field_method_set("size", |_lua, this, size| {
            this.with_mut(|this| {
                this.size = size;
            });
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentSize(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });
    }
}
