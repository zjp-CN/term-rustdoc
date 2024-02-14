use super::{parse, region::LinkedRegions, StyledText};
use crate::{
    ui::{
        scrollable::{generics::LineState, Scrollable},
        Page,
    },
    Result,
};
use ratatui::style::Style;
use std::{fmt, ops::Deref};
use term_rustdoc::{tree::CrateDoc, util::XString};
use unicode_width::UnicodeWidthStr;

/// Scrollable text area for displaying markdown.
pub type ScrollText = Scrollable<StyledLines>;

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
    /// To switch between non-wrapping and wrapping behavior, press `<space>` shortcut.
    syntect: bool,
    lines: Vec<StyledLine>,
    regions: LinkedRegions,
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
    pub fn update_doc(&mut self, id: &str, width: Option<f64>) -> bool {
        if let Some(doc) = &self.doc {
            if let Some(doc) = doc.get_doc(id) {
                if let Some(width) = (!self.syntect).then_some(width).flatten() {
                    let (lines, regions) = parse::parse_doc(doc, width);
                    self.lines = lines;
                    self.regions = regions;
                } else {
                    self.lines = parse::md(doc)
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
        self.update_content();
    }
}

impl ScrollText {
    pub fn new_text(doc: Option<CrateDoc>) -> Result<Self> {
        Ok(Scrollable {
            lines: StyledLines::new(doc),
            ..Default::default()
        })
    }

    fn wrapping_width(&self) -> Option<f64> {
        // wrapping width should less than area width
        (self.max_windth > 1).then(|| self.max_windth.saturating_sub(1) as f64)
    }

    pub fn update_doc(&mut self, id: &str) -> bool {
        let width = self.wrapping_width();
        self.lines.update_doc(id, width)
    }
}
