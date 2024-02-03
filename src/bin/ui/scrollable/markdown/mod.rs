use super::Scrollable;
use crate::Result;
use rustdoc_types::Id;
use std::{cell::RefCell, fmt, ops::Deref};
use term_rustdoc::tree::{CrateDoc, Text as StyledText};

mod parse;
mod render;

/// Scrollable text area for displaying markdown.
pub type ScrollText = Scrollable<StyledLines>;

pub struct StyledLine {
    line: Vec<StyledText>,
}

impl Deref for StyledLine {
    type Target = [StyledText];

    fn deref(&self) -> &Self::Target {
        &self.line
    }
}

#[derive(Default)]
pub struct StyledLines {
    lines: Vec<StyledLine>,
    doc: Option<CrateDoc>,
    idbuf: RefCell<String>,
}

impl fmt::Debug for StyledLines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StyledLines")
            .field("lines-len", &self.lines.len())
            .finish()
    }
}

impl Deref for StyledLines {
    type Target = [StyledLine];

    fn deref(&self) -> &Self::Target {
        &self.lines
    }
}

impl StyledLines {
    pub fn new(doc: Option<CrateDoc>) -> Self {
        StyledLines {
            doc,
            ..Default::default()
        }
    }

    fn get<T>(&self, id: &str, f: impl FnOnce(&Id) -> T) -> T {
        let mut idbuf = self.idbuf.take();
        idbuf.clear();
        idbuf.push_str(id);
        let id = Id(idbuf);
        let res = f(&id);
        self.idbuf.replace(id.0);
        res
    }

    /// only returns true if a new doc is fetched
    pub fn update_doc(&mut self, id: &str) -> bool {
        if let Some(doc) = &self.doc {
            if let Some(doc) = self.get(id, |id| {
                doc.doc()
                    .index
                    .get(id)
                    .and_then(|item| item.docs.as_deref())
            }) {
                self.lines = parse::md(doc);
                return true;
            }
        }
        self.reset_doc();
        false
    }

    /// FIXME: cache queried doc to save parsing
    pub fn reset_doc(&mut self) {
        self.lines = Vec::new();
    }
}

impl ScrollText {
    pub fn new_text(doc: Option<CrateDoc>) -> Result<Self> {
        // TODO:max_windth and text wrap for markdown
        Ok(Scrollable {
            lines: StyledLines::new(doc),
            ..Default::default()
        })
    }
}
