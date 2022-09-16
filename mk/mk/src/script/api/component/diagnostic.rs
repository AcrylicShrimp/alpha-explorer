use mlua::prelude::*;

pub type ComponentDiagnostic = super::Component<crate::component::Diagnostic>;

impl LuaUserData for ComponentDiagnostic {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentDiagnostic(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });
    }
}
