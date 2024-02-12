use super::{parse, StyledText};
use crate::{
    ui::{
        scrollable::{generics::LineState, Scrollable},
        Page,
    },
    Result,
};
use std::{fmt, ops::Deref};
use term_rustdoc::{tree::CrateDoc, util::XString};

/// Scrollable text area for displaying markdown.
pub type ScrollText = Scrollable<StyledLines>;

pub struct StyledLine {
    pub line: Vec<StyledText>,
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
        self.iter().map(|st| st.text()).collect()
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        self.iter()
            .zip(state)
            .all(|(st, s)| st.as_str() == s.as_str())
    }
}

#[derive(Default)]
pub struct StyledLines {
    /// use syntect's highlighting, but without text wrapped
    syntect: bool,
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
                self.lines = if self.syntect {
                    super::parse::md(doc)
                } else {
                    let mut lines = Vec::with_capacity(128);
                    parse::parse_doc(doc, 80.0, &mut lines);
                    lines.shrink_to_fit();
                    lines
                };
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

    pub fn toggle_sytect(&mut self) {
        self.syntect = !self.syntect;
    }
}

impl Page {
    pub fn toggle_sytect(&mut self) {
        self.content().lines.toggle_sytect();
        if let Some(id) = self.outline.display.get_id() {
            self.content.display.lines.update_doc(id);
        }
    }
}

// impl StyledLines {
//     pub fn append_new_line(&mut self) {
//         self.lines.push(StyledLine { line: Vec::new() });
//     }
//
//     pub fn append_line<L: Into<StyledLine>>(&mut self, line: L) {
//         self.lines.push(line.into());
//     }
// }

impl ScrollText {
    pub fn new_text(doc: Option<CrateDoc>) -> Result<Self> {
        // TODO:max_windth and text wrap for markdown
        Ok(Scrollable {
            lines: StyledLines::new(doc),
            ..Default::default()
        })
    }
}
