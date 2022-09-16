use mlua::prelude::*;

pub type ComponentUIScaler = super::Component<crate::component::UIScaler>;

impl LuaUserData for ComponentUIScaler {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("mode", |_lua, this| Ok(this.with_ref(|this| this.mode)));
        fields.add_field_method_get("reference_size", |_lua, this| {
            Ok(this.with_ref(|this| this.reference_size))
        });

        fields.add_field_method_set("mode", |_lua, this, mode| {
            this.with_mut(|this| {
                this.mode = mode;
            });
            Ok(())
        });
        fields.add_field_method_set("reference_size", |_lua, this, reference_size| {
            this.with_mut(|this| {
                this.reference_size = reference_size;
            });
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentUIScaler(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });
    }
}
