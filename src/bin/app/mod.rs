use crate::Result;
use rustdoc_types::Crate;
use std::rc::Rc;
use term_rustdoc::tree::{DModule, IDMap, Show, TreeLines};

#[derive(Default)]
pub struct App {
    doc: Option<CrateDoc>,
    pub should_quit: bool,
}

pub type CrateDoc = Rc<RustDoc>;

#[derive(Clone)]
pub struct RustDoc {
    pub doc: Crate,
}

impl RustDoc {
    fn dmodule_idmap(&self) -> (DModule, IDMap<'_>) {
        (DModule::new(&self.doc), IDMap::from_crate(&self.doc))
    }
}

impl App {
    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn set_doc(&mut self) -> Result<TreeLines> {
        let doc = serde_json::from_reader(std::fs::File::open("target/deps/doc/tokio.json")?)?;
        let doc = RustDoc { doc };
        let (dmod, map) = doc.dmodule_idmap();
        let outline = dmod.show_prettier(&map).into_treelines();
        self.doc = Some(Rc::new(doc));
        Ok(outline)
    }

    pub fn rustdoc(&self) -> Option<CrateDoc> {
        self.doc.clone()
    }
}
