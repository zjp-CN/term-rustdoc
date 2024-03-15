use super::{
    heading::Headings,
    parse::{self, Blocks},
    StyledText,
};
use crate::{
    ui::scrollable::{generics::LineState, Scroll},
    Result,
};
use ratatui::style::Style;
use std::{fmt, ops::Deref};
use term_rustdoc::{tree::CrateDoc, util::XString};
use unicode_width::UnicodeWidthStr;

/// Scrollable text area for displaying markdown.
pub type ScrollText = Scroll<StyledLines>;

pub struct StyledLine {
    line: Vec<StyledText>,
    /// the total width of a line
    width: usize,
}

impl StyledLine {
    pub fn new() -> Self {
        Self {
            line: Vec::new(),
            width: 0,
        }
    }

    pub fn push<T: Into<XString>>(&mut self, text: T, style: Style) {
        let start = self.width;
        let text = text.into();
        self.width += text.width();
        self.line.push(StyledText::new(text, style, start));
    }

    pub fn shrink_to_fit(&mut self) {
        self.line.shrink_to_fit();
    }

    pub fn iter_text_style(&self) -> impl Iterator<Item = (&'_ str, Style)> {
        self.line.iter().map(|l| (l.as_str(), l.style()))
    }

    pub fn remove_trailing_whitespace(&mut self) {
        if let Some(last_word) = self.line.last_mut() {
            if last_word.remove_trailing_whitespace() {
                self.width -= 1;
            }
        }
    }
}

/// NOTE: [`StyledText`]s must have correct [`ColumnSpan`]s here.
impl From<Vec<StyledText>> for StyledLine {
    fn from(line: Vec<StyledText>) -> Self {
        let width = line.last().map(StyledText::span_end).unwrap_or(0);
        StyledLine { line, width }
    }
}

impl fmt::Debug for StyledLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for word in &self.line {
            word.fmt(f)?;
        }
        Ok(())
    }
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
    /// Use syntect's highlighting, but with no text wrapped, which means contents won't be shown
    /// if they exceed the area width.
    ///
    /// To switch between non-wrapping and wrapping behavior, press `d` key.
    syntect: bool,
    lines: Vec<StyledLine>,
    blocks: Blocks,
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
    pub fn new(doc: CrateDoc) -> Self {
        StyledLines {
            doc: Some(doc),
            ..Default::default()
        }
    }

    /// Only returns Some if a new doc is fetched.
    ///
    /// The Headings can still be empty because heading jumping isn't supported in syntect case.
    pub fn update_doc(&mut self, id: &str, width: Option<f64>) -> Option<Headings> {
        if let Some(doc) = &self.doc {
            if let Some(doc) = doc.get_doc(id) {
                return if let Some(width) = width {
                    let (lines, blocks, headings) = parse::parse_doc(doc, width);
                    self.lines = lines;
                    self.blocks = blocks;
                    Some(headings)
                } else {
                    warn!("no wrapping for markdown content");
                    self.lines = parse::md(doc);
                    Some(Headings::default())
                };
            }
        }
        self.reset_doc();
        None
    }

    /// FIXME: cache queried doc to save parsing
    pub fn reset_doc(&mut self) {
        self.lines = Vec::new();
    }

    pub fn toggle_sytect(&mut self) {
        self.syntect = !self.syntect;
    }
}

impl ScrollText {
    pub fn new_text(doc: CrateDoc) -> Result<Self> {
        Ok(Scroll {
            lines: StyledLines::new(doc),
            ..Default::default()
        })
    }

    // Wrapping width is the exclusive maximum of a line.
    // It's like the area width, all texts should be strictly less than the width.
    //
    // When syntect is used, no need to get wrapping width, because we won't support
    // wrapping in case of syntect.
    fn wrapping_width(&self) -> Option<f64> {
        // we use this method to aviod duplicating a width field in StyledLines.
        (!self.lines.syntect && self.area.width > 1).then_some(self.area.width as f64)
    }

    pub fn update_doc(&mut self, id: &str) -> Option<Headings> {
        let width = self.wrapping_width();
        self.lines.update_doc(id, width)
    }

    pub fn doc_ref(&self) -> Option<&CrateDoc> {
        self.lines.doc.as_ref()
    }
}
