use crate::script::event::{Diagnostic, DiagnosticLevel};
use colored::{ColoredString, Colorize};

pub(crate) fn log_diagnostic_event(event: &Diagnostic) {
    let prefix = format!("{:>6}: ", event.level.to_str());
    let indent = prefix.len();
    let lines = event.message.split('\n').collect::<Vec<_>>();
    let (&first_line, context_lines) = lines.split_first().unwrap();
    let message = format!(
        "{}{} [{}:{}:{}]",
        set_color(event.level, prefix),
        first_line,
        event.file,
        event.line,
        event.column
    );
    let message = if context_lines.is_empty() {
        message
    } else {
        [
            message,
            context_lines
                .iter()
                .map(|&line| format!("{:indent$}{}", "", line, indent = indent))
                .collect::<Vec<_>>()
                .join("\n"),
        ]
        .join("\n")
    };

    println!("{}", message);

    for sub_diagnostics in &event.sub_diagnostics {
        let prefix = format!("> {:>6}: ", sub_diagnostics.level.to_str());
        let indent = prefix.len();
        let lines = sub_diagnostics.message.split('\n').collect::<Vec<_>>();
        let (&first_line, context_lines) = lines.split_first().unwrap();
        let message = format!(
            "        {}{} [{}:{}:{}]",
            set_color(sub_diagnostics.level, prefix),
            first_line,
            sub_diagnostics.file,
            sub_diagnostics.line,
            sub_diagnostics.column
        );
        let message = if context_lines.is_empty() {
            message
        } else {
            [
                message,
                context_lines
                    .iter()
                    .map(|&line| format!("        {:indent$}{}", "", line, indent = indent))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ]
            .join("\n")
        };

        println!("{}", message);
    }
}

fn set_color(level: DiagnosticLevel, str: String) -> ColoredString {
    match level {
        DiagnosticLevel::Debug => str.green(),
        DiagnosticLevel::Info => str.blue(),
        DiagnosticLevel::Warn => str.yellow(),
        DiagnosticLevel::Error => str.red(),
        DiagnosticLevel::Fatal => str.magenta(),
    }
}
