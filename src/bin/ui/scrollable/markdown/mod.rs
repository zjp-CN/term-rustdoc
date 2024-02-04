use super::{LineState, Scrollable};
use crate::Result;
use std::{fmt, ops::Deref};
use term_rustdoc::{
    tree::{CrateDoc, Text as StyledText},
    util::XString,
};

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

impl LineState for StyledLine {
    type State = Vec<XString>;

    fn state(&self) -> Self::State {
        self.iter().map(|st| st.text.clone()).collect()
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        self.iter().zip(state).all(|(st, s)| st.text == s)
    }
}

#[derive(Default)]
pub struct StyledLines {
    lines: Vec<StyledLine>,
    doc: Option<CrateDoc>,
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

    /// only returns true if a new doc is fetched
    pub fn update_doc(&mut self, id: &str) -> bool {
        if let Some(doc) = &self.doc {
            if let Some(doc) = doc.get_doc(id) {
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
