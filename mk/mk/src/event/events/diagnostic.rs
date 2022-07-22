use mlua::prelude::*;
use std::any::type_name;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl DiagnosticLevel {
    pub fn to_str(&self) -> &'static str {
        match self {
            DiagnosticLevel::Debug => "debug",
            DiagnosticLevel::Info => "info",
            DiagnosticLevel::Warn => "warn",
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Fatal => "fatal",
        }
    }

    pub fn from_str(s: &str) -> Option<DiagnosticLevel> {
        Some(match s {
            "debug" => DiagnosticLevel::Debug,
            "info" => DiagnosticLevel::Info,
            "warn" => DiagnosticLevel::Warn,
            "error" => DiagnosticLevel::Error,
            "fatal" => DiagnosticLevel::Fatal,
            _ => return None,
        })
    }
}

impl<'lua> FromLua<'lua> for DiagnosticLevel {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        Ok(match value {
            LuaValue::String(str) => match Self::from_str(str.to_str()?) {
                Some(level) => level,
                None => return Err(format!("invalid {}", type_name::<Self>()).to_lua_err()),
            },
            _ => return Err(format!("invalid {}", type_name::<Self>()).to_lua_err()),
        })
    }
}

impl<'lua> ToLua<'lua> for DiagnosticLevel {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.to_str().to_lua(lua)
    }
}

#[derive(Debug, Clone, Hash)]
pub struct SubDiagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl<'lua> FromLua<'lua> for SubDiagnostic {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        Ok(match value {
            LuaValue::Table(table) => Self {
                level: match DiagnosticLevel::from_str(table.get::<_, String>("level")?.as_str()) {
                    Some(level) => level,
                    None => return Err(format!("invalid {}", type_name::<Self>()).to_lua_err()),
                },
                message: table.get("message")?,
                file: table.get("file")?,
                line: table.get("line")?,
                column: table.get("column")?,
            },
            _ => return Err(format!("invalid {}", type_name::<Self>()).to_lua_err()),
        })
    }
}

impl<'lua> ToLua<'lua> for SubDiagnostic {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("level", self.level.to_str())?;
        table.set("message", self.message)?;
        table.set("file", self.file)?;
        table.set("line", self.line)?;
        table.set("column", self.column)?;
        Ok(LuaValue::Table(table))
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub sub_diagnostics: Vec<SubDiagnostic>,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl_event_type_lua_api!(Diagnostic);

impl<'lua> FromLua<'lua> for Diagnostic {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        Ok(match value {
            LuaValue::Table(table) => Self {
                level: match DiagnosticLevel::from_str(table.get::<_, String>("level")?.as_str()) {
                    Some(level) => level,
                    None => return Err(format!("invalid {}", type_name::<Self>()).to_lua_err()),
                },
                message: table.get("message")?,
                sub_diagnostics: table.get("sub_diagnostics")?,
                file: table.get("file")?,
                line: table.get("line")?,
                column: table.get("column")?,
            },
            _ => return Err(format!("invalid {}", type_name::<Self>()).to_lua_err()),
        })
    }
}

impl<'lua> ToLua<'lua> for Diagnostic {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("level", self.level.to_str())?;
        table.set("message", self.message)?;
        table.set("sub_diagnostics", self.sub_diagnostics)?;
        table.set("file", self.file)?;
        table.set("line", self.line)?;
        table.set("column", self.column)?;
        Ok(LuaValue::Table(table))
    }
}

#[macro_export]
macro_rules! emit_diagnostic {
    ($level:expr, $message:expr) => {
        let (file, line, column) = (file!(), line!(), column!());
        crate::api::use_context().event_mgr().dispatcher().emit(
            crate::api::use_context().lua_mgr().lua(),
            &crate::event::events::Diagnostic {
                level: $level,
                message: $message,
                sub_diagnostics: vec![],
                file: file.to_owned(),
                line,
                column,
            },
        );
    };
    ($level:expr, $message:expr, $sub_diagnostics:expr) => {
        let (file, line, column) = (file!(), line!(), column!());
        crate::api::use_context().event_mgr().dispatcher().emit(
            crate::api::use_context().lua_mgr().lua(),
            &crate::event::events::Diagnostic {
                level: $level,
                message: $message,
                sub_diagnostics: $sub_diagnostics,
                file: file.to_owned(),
                line,
                column,
            },
        );
    };
}

#[macro_export]
macro_rules! emit_diagnostic_debug {
    ($message:expr) => {
        crate::emit_diagnostic!(crate::event::events::DiagnosticLevel::Debug, $message);
    };
    ($message:expr, $sub_diagnostics:expr) => {
        crate::emit_diagnostic!(
            crate::event::events::DiagnosticLevel::Debug,
            $message,
            $sub_diagnostics
        );
    };
}

#[macro_export]
macro_rules! emit_diagnostic_info {
    ($message:expr) => {
        crate::emit_diagnostic!(crate::event::events::DiagnosticLevel::Info, $message);
    };
    ($message:expr, $sub_diagnostics:expr) => {
        crate::emit_diagnostic!(
            crate::event::events::DiagnosticLevel::Info,
            $message,
            $sub_diagnostics
        );
    };
}

#[macro_export]
macro_rules! emit_diagnostic_warn {
    ($message:expr) => {
        crate::emit_diagnostic!(crate::event::events::DiagnosticLevel::Warn, $message);
    };
    ($message:expr, $sub_diagnostics:expr) => {
        crate::emit_diagnostic!(
            crate::event::events::DiagnosticLevel::Warn,
            $message,
            $sub_diagnostics
        );
    };
}

#[macro_export]
macro_rules! emit_diagnostic_error {
    ($message:expr) => {
        crate::emit_diagnostic!(crate::event::events::DiagnosticLevel::Error, $message);
    };
    ($message:expr, $sub_diagnostics:expr) => {
        crate::emit_diagnostic!(
            crate::event::events::DiagnosticLevel::Error,
            $message,
            $sub_diagnostics
        );
    };
}

#[macro_export]
macro_rules! emit_diagnostic_fatal {
    ($message:expr) => {
        crate::emit_diagnostic!(crate::event::events::DiagnosticLevel::Fatal, $message);
    };
    ($message:expr, $sub_diagnostics:expr) => {
        crate::emit_diagnostic!(
            crate::event::events::DiagnosticLevel::Fatal,
            $message,
            $sub_diagnostics
        );
    };
}

#[macro_export]
macro_rules! subdiag {
    ($level:expr, $message:expr) => {{
        let (file, line, column) = (file!(), line!(), column!());
        crate::event::events::SubDiagnostic {
            level: $level,
            message: $message,
            file: file.to_owned(),
            line,
            column,
        }
    }};
}

#[macro_export]
macro_rules! subdiag_debug {
    ($message:expr) => {
        crate::subdiag!(crate::event::events::DiagnosticLevel::Debug, $message)
    };
}

#[macro_export]
macro_rules! subdiag_info {
    ($message:expr) => {
        crate::subdiag!(crate::event::events::DiagnosticLevel::Info, $message)
    };
}

#[macro_export]
macro_rules! subdiag_warn {
    ($message:expr) => {
        crate::subdiag!(crate::event::events::DiagnosticLevel::Warn, $message)
    };
}

#[macro_export]
macro_rules! subdiag_error {
    ($message:expr) => {
        crate::subdiag!(crate::event::events::DiagnosticLevel::Error, $message)
    };
}

#[macro_export]
macro_rules! subdiag_fatal {
    ($message:expr) => {
        crate::subdiag!(crate::event::events::DiagnosticLevel::Fatal, $message)
    };
}
