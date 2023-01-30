#[macro_export]
macro_rules! emit_diagnostic {
    ($level:expr, $message:expr) => {
        let (file, line, column) = (file!(), line!(), column!());
        let context = crate::engine::use_context();
        context.event_mgr().emit(
            &crate::script::event::Diagnostic {
                level: $level,
                message: $message,
                sub_diagnostics: vec![],
                file: file.to_owned(),
                line,
                column,
            },
            context.script_mgr().lua(),
        );
    };
    ($level:expr, $message:expr, $sub_diagnostics:expr) => {
        let (file, line, column) = (file!(), line!(), column!());
        let context = crate::engine::use_context();
        context.event_mgr().emit(
            &crate::script::event::Diagnostic {
                level: $level,
                message: $message,
                sub_diagnostics: $sub_diagnostics,
                file: file.to_owned(),
                line,
                column,
            },
            context.script_mgr().lua(),
        );
    };
}

#[macro_export]
macro_rules! emit_diagnostic_debug {
    ($message:expr) => {
        crate::emit_diagnostic!(crate::script::event::DiagnosticLevel::Debug, $message);
    };
    ($message:expr, $sub_diagnostics:expr) => {
        crate::emit_diagnostic!(
            crate::script::event::DiagnosticLevel::Debug,
            $message,
            $sub_diagnostics
        );
    };
}

#[macro_export]
macro_rules! emit_diagnostic_info {
    ($message:expr) => {
        crate::emit_diagnostic!(crate::script::event::DiagnosticLevel::Info, $message);
    };
    ($message:expr, $sub_diagnostics:expr) => {
        crate::emit_diagnostic!(
            crate::script::event::DiagnosticLevel::Info,
            $message,
            $sub_diagnostics
        );
    };
}

#[macro_export]
macro_rules! emit_diagnostic_warn {
    ($message:expr) => {
        crate::emit_diagnostic!(crate::script::event::DiagnosticLevel::Warn, $message);
    };
    ($message:expr, $sub_diagnostics:expr) => {
        crate::emit_diagnostic!(
            crate::script::event::DiagnosticLevel::Warn,
            $message,
            $sub_diagnostics
        );
    };
}

#[macro_export]
macro_rules! emit_diagnostic_error {
    ($message:expr) => {
        crate::emit_diagnostic!(crate::script::event::DiagnosticLevel::Error, $message);
    };
    ($message:expr, $sub_diagnostics:expr) => {
        crate::emit_diagnostic!(
            crate::script::event::DiagnosticLevel::Error,
            $message,
            $sub_diagnostics
        );
    };
}

#[macro_export]
macro_rules! emit_diagnostic_fatal {
    ($message:expr) => {
        crate::emit_diagnostic!(crate::script::event::DiagnosticLevel::Fatal, $message);
    };
    ($message:expr, $sub_diagnostics:expr) => {
        crate::emit_diagnostic!(
            crate::script::event::DiagnosticLevel::Fatal,
            $message,
            $sub_diagnostics
        );
    };
}

#[macro_export]
macro_rules! subdiag {
    ($level:expr, $message:expr) => {{
        let (file, line, column) = (file!(), line!(), column!());
        crate::script::event::SubDiagnostic {
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
        crate::subdiag!(crate::script::event::DiagnosticLevel::Debug, $message)
    };
}

#[macro_export]
macro_rules! subdiag_info {
    ($message:expr) => {
        crate::subdiag!(crate::script::event::DiagnosticLevel::Info, $message)
    };
}

#[macro_export]
macro_rules! subdiag_warn {
    ($message:expr) => {
        crate::subdiag!(crate::script::event::DiagnosticLevel::Warn, $message)
    };
}

#[macro_export]
macro_rules! subdiag_error {
    ($message:expr) => {
        crate::subdiag!(crate::script::event::DiagnosticLevel::Error, $message)
    };
}

#[macro_export]
macro_rules! subdiag_fatal {
    ($message:expr) => {
        crate::subdiag!(crate::script::event::DiagnosticLevel::Fatal, $message)
    };
}
