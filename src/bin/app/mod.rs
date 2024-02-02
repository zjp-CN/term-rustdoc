use crate::Result;
use term_rustdoc::tree::{CrateDoc, TreeLines};

#[derive(Default)]
pub struct App {
    doc: Option<CrateDoc>,
    pub should_quit: bool,
}

impl App {
    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn set_doc(&mut self) -> Result<TreeLines> {
        let doc = serde_json::from_reader(std::fs::File::open("target/deps/doc/tokio.json")?)?;
        let doc = CrateDoc::new(doc);
        let outline = TreeLines::new(doc.clone()).0;
        self.doc = Some(doc);
        Ok(outline)
    }

    pub fn rustdoc(&self) -> Option<CrateDoc> {
        self.doc.clone()
    }
}
