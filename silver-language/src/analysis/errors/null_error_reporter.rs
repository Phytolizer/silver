use super::error_reporter::ErrorReporter;

pub struct NullErrorReporter {
    had_error: bool,
}

impl NullErrorReporter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for NullErrorReporter {
    fn default() -> Self {
        Self { had_error: false }
    }
}

impl ErrorReporter for NullErrorReporter {
    fn report_error(&mut self, _error: crate::analysis::diagnostic::Diagnostic) {
        self.had_error = true;
    }

    fn had_error(&self) -> bool {
        self.had_error
    }

    fn errors(&self) -> &[crate::analysis::diagnostic::Diagnostic] {
        &[]
    }

    fn clear(&mut self) {
        self.had_error = false;
    }
}
