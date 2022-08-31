use crate::script::api::ModuleType;

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

impl ModuleType for DiagnosticLevel {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<DiagnosticLevel>("DiagnosticLevel");

        module.set_native_fn("enum_type", |this: &mut Self| {
            Ok(match this {
                DiagnosticLevel::Debug => "Debug",
                DiagnosticLevel::Info => "Info",
                DiagnosticLevel::Warn => "Warn",
                DiagnosticLevel::Error => "Error",
                DiagnosticLevel::Fatal => "Fatal",
            })
        });
    }
}

#[derive(Debug, Clone)]
pub struct SubDiagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl ModuleType for SubDiagnostic {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<SubDiagnostic>("SubDiagnostic");

        module.set_getter_fn("level", |this: &mut Self| Ok(this.level));
        module.set_getter_fn("message", |this: &mut Self| Ok(this.message.clone()));
        module.set_getter_fn("file", |this: &mut Self| Ok(this.file.clone()));
        module.set_getter_fn("line", |this: &mut Self| Ok(this.line));
        module.set_getter_fn("column", |this: &mut Self| Ok(this.column));
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

impl ModuleType for Diagnostic {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Diagnostic>("Diagnostic");

        module.set_getter_fn("level", |this: &mut Self| Ok(this.level));
        module.set_getter_fn("message", |this: &mut Self| Ok(this.message.clone()));
        module.set_getter_fn("sub_diagnostics", |this: &mut Self| {
            Ok(this.sub_diagnostics.clone())
        });
        module.set_getter_fn("file", |this: &mut Self| Ok(this.file.clone()));
        module.set_getter_fn("line", |this: &mut Self| Ok(this.line));
        module.set_getter_fn("column", |this: &mut Self| Ok(this.column));

        to_global!(
            module,
            module.set_native_fn("get_sub_diagnostic_at", |this: &mut Self, index: usize| {
                Ok(this.sub_diagnostics.get(index).cloned())
            })
        );
    }
}
