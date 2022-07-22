pub struct DiagnosticManager {
    pub console_enabled: bool,
}

impl DiagnosticManager {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DiagnosticManager {
    fn default() -> Self {
        Self {
            console_enabled: false,
        }
    }
}
