pub(crate) struct ViewOptions {
    pub(crate) show_tree: bool,
}

impl Default for ViewOptions {
    fn default() -> Self {
        Self { show_tree: false }
    }
}
