use rustdoc_types::Crate;

#[derive(Default)]
pub struct App {
    crate_doc: Option<Crate>,
    pub should_quit: bool,
}

impl App {
    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
