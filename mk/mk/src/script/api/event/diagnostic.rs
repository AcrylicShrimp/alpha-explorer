use crate::script::api::LuaApiTable;
use mlua::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl DiagnosticLevel {
    pub fn to_str(self) -> &'static str {
        match self {
            DiagnosticLevel::Debug => "debug",
            DiagnosticLevel::Info => "info",
            DiagnosticLevel::Warn => "warn",
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Fatal => "fatal",
        }
    }
}

impl LuaApiTable for DiagnosticLevel {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Debug", DiagnosticLevel::Debug)?;
        table.set("Info", DiagnosticLevel::Info)?;
        table.set("Warn", DiagnosticLevel::Warn)?;
        table.set("Error", DiagnosticLevel::Error)?;
        table.set("Fatal", DiagnosticLevel::Fatal)?;

        Ok(table)
    }
}

impl LuaUserData for DiagnosticLevel {}

#[derive(Debug, Clone)]
pub struct SubDiagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl LuaUserData for SubDiagnostic {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("level", |_lua, this| Ok(this.level));
        fields.add_field_method_get("message", |_lua, this| Ok(this.message.clone()));
        fields.add_field_method_get("file", |_lua, this| Ok(this.file.clone()));
        fields.add_field_method_get("line", |_lua, this| Ok(this.line));
        fields.add_field_method_get("column", |_lua, this| Ok(this.column));
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub sub_diagnostics: Vec<SubDiagnostic>,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl LuaUserData for Diagnostic {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("level", |_lua, this| Ok(this.level));
        fields.add_field_method_get("message", |_lua, this| Ok(this.message.clone()));
        fields.add_field_method_get("sub_diagnostics", |_lua, this| {
            Ok(this.sub_diagnostics.clone())
        });
        fields.add_field_method_get("file", |_lua, this| Ok(this.file.clone()));
        fields.add_field_method_get("line", |_lua, this| Ok(this.line));
        fields.add_field_method_get("column", |_lua, this| Ok(this.column));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("sub_diagnostic", |_lua, this, index: usize| {
            Ok(this.sub_diagnostics.get(index + 1).cloned())
        });
    }
}

impl LuaApiTable for Diagnostic {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}
