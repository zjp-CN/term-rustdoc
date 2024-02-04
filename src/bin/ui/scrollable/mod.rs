use super::Selected;
use crate::{err, Result};
use ratatui::prelude::Rect;
use std::{fmt, ops::Deref};
use term_rustdoc::{
    tree::{TreeLine, TreeLines},
    util::XString,
};

mod interaction;
mod markdown;
mod render;

pub use self::markdown::ScrollText;

/// Scrollable tree view but stored in lines.
pub type ScrollTreeLines = Scrollable<TreeLines, TreeLine>;

pub struct Cursor<Line: LineState> {
    /// The row position because scrollable area only highlights row.
    ///
    /// This should be less than area's height at any time.
    pub y: u16,
    /// Remember the last cursor as much as possible with this state.
    ///
    /// This is mainly used to improve the cursor UX when scrolling
    /// or folding redraw the screen and the same line is available.
    pub state: Line::State,
}

impl<Line: LineState> Default for Cursor<Line> {
    fn default() -> Self {
        Self {
            y: 0,
            state: Default::default(),
        }
    }
}

/// A text panel that can be scrolled and select texts when the cursor is inside of it.
pub struct Scrollable<Lines, State: LineState> {
    /// Styled texts on each line
    pub lines: Lines,
    /// The start of row to be displayed
    pub start: usize,
    /// The row position where cursor was last time
    pub cursor: Cursor<State>,
    /// The maximum width among all lines
    pub max_windth: u16,
    /// The selected text across lines
    pub select: Option<Selected>,
    /// The widget area, usually not the full screen
    pub area: Rect,
}

pub trait LineState {
    type State: PartialEq + Default;
    fn state(&self) -> Self::State;
    fn is_identical(&self, state: &Self::State) -> bool;
}

impl LineState for TreeLine {
    type State = Option<XString>;

    fn state(&self) -> Self::State {
        self.id.clone()
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        self.id == *state
    }
}

impl<L: LineState, Lines: Deref<Target = [L]>> Scrollable<Lines, L> {
    /// The whole lines including non-visible lines.
    pub fn all_lines(&self) -> &[L] {
        &self.lines
    }

    /// The visible lines to be rendered.
    ///
    /// The slice length should be able to cast to u16 without surprising behavior.
    pub fn visible_lines(&self) -> Option<&[L]> {
        let total_len = self.lines.len();
        let end = (self.start + self.area.height as usize).min(total_len);
        self.lines.get(self.start..end)
    }

    /// The line that current cursor on screen points to.
    pub fn get_line_of_current_cursor(&self) -> Option<&L> {
        self.visible_lines().and_then(|lines| {
            let line = lines.get(self.cursor.y as usize);
            if line.is_none() && self.total_len() != 0 {
                error!("Cursor is beyond all lines length {}.", self.total_len());
            }
            line
        })
    }

    pub fn total_len(&self) -> usize {
        self.lines.len()
    }
}

impl<Lines: Default, L: LineState> Default for Scrollable<Lines, L> {
    fn default() -> Self {
        let (lines, start, cursor, max_windth, select, area) = Default::default();
        Scrollable {
            lines,
            start,
            cursor,
            max_windth,
            select,
            area,
        }
    }
}

impl<Lines: Default + Deref<Target = [TreeLine]>> Scrollable<Lines, TreeLine> {
    pub fn new(lines: Lines) -> Result<Self> {
        let w = lines.iter().map(TreeLine::width).max();
        let max_windth = w.ok_or_else(|| err!("The documentation is empty with no items."))?;

        Ok(Self {
            lines,
            max_windth,
            ..Default::default()
        })
    }

    /// Get the item id the current cursor points to.
    /// Non-item node doesn't have an id.
    pub fn get_id(&self) -> Option<&str> {
        self.all_lines()
            .get(self.cursor.y as usize + self.start)
            .and_then(|l| l.id.as_deref())
    }

    pub fn update_maxwidth(&mut self) {
        self.max_windth = self.lines.iter().map(TreeLine::width).max().unwrap();
    }
}

impl<L: LineState, Lines: Deref<Target = [L]>> fmt::Debug for Scrollable<Lines, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Scrollable");
        s.field("lines.len", &self.total_len())
            .field("start", &self.start)
            .field("cursor.y", &self.cursor.y)
            .field("max_windth", &self.max_windth)
            .field("area", &self.area);
        if let Some(select) = &self.select {
            s.field("select", select);
        }
        s.finish()
    }
}
