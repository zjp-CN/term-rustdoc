use crate::Result;
use std::path::PathBuf;
use term_rustdoc::tree::{CrateDoc, TreeLines};

mod local_registry;

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
        let path = PathBuf::from_iter([
            "target",
            "deps",
            "doc",
            #[cfg(target_os = "windows")]
            "integration.json",
            #[cfg(not(target_os = "windows"))]
            "tokio.json",
        ]);
        let doc = serde_json::from_reader(std::fs::File::open(path)?)?;
        let doc = CrateDoc::new(doc);
        let outline = TreeLines::new(doc.clone());
        self.doc = Some(doc);
        Ok(outline)
    }

    pub fn rustdoc(&self) -> Option<CrateDoc> {
        self.doc.clone()
    }
}
