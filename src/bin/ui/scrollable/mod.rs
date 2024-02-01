use super::Selected;
use crate::{err, Result};
use ratatui::prelude::Rect;
use std::fmt;
use term_rustdoc::tree::TreeLine;

mod render;

/// A text panel that can be scrolled and select texts when the cursor is inside of it.
#[derive(Default)]
pub struct Scrollable<Lines> {
    /// Styled texts on each line
    pub lines: Lines,
    /// The start of row to be displayed
    pub start: usize,
    /// The row position where cursor was last time
    pub cursor: u16,
    /// The maximum width among all lines
    pub max_windth: u16,
    /// The selected text across lines
    pub select: Option<Selected>,
    /// The widget area, usually not the full screen
    pub area: Rect,
}

impl<Line: AsRef<[TreeLine]>> Scrollable<Line> {
    pub fn lines(&self) -> &[TreeLine] {
        self.lines.as_ref()
    }

    pub fn len(&self) -> usize {
        self.lines().len()
    }
}

impl<Lines: Default + AsRef<[TreeLine]>> Scrollable<Lines> {
    pub fn new(lines: Lines, full: Rect) -> Result<Self> {
        let w = lines.as_ref().iter().map(TreeLine::width).max();
        let max_windth = w.ok_or_else(|| err!("The documentation is empty with no items."))?;
        if full.width < max_windth {
            warn!(
                full.width,
                max_windth, "Outline width exceeds the area width, so lines may be truncated."
            );
        }
        Ok(Self {
            lines,
            max_windth,
            area: full,
            ..Default::default()
        })
    }
}

impl<Lines> Scrollable<Lines> {
    /// The index the current cursor on screen points to.
    pub fn idx_of_current_cursor(&self) -> usize {
        self.cursor as usize + self.start
    }
}

impl<Lines: AsRef<[TreeLine]>> fmt::Debug for Scrollable<Lines> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Scrollable");
        s.field("lines.len", &self.len())
            .field("start", &self.start)
            .field("cursor", &self.cursor)
            .field("max_windth", &self.max_windth)
            .field("area", &self.area);
        if let Some(select) = &self.select {
            s.field("select", select);
        }
        s.finish()
    }
}
