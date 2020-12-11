pub trait ErrorReporter {
    fn report_error(&mut self, error: String);
    fn had_error(&self) -> bool;
    fn errors(&self) -> &[String];
}
