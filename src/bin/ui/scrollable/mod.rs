use crate::{err, Result};
use ratatui::prelude::Rect;
use std::fmt;
use term_rustdoc::tree::{TreeLine, TreeLines};

mod generics;
mod interaction;
mod markdown;
mod render;

pub use self::generics::{render_line, LineState, Lines};
pub use self::markdown::{MarkdownAndHeading, ScrollHeading, ScrollMarkdown, ScrollText};

/// Scrollable tree view but stored in lines.
pub type ScrollTreeLines = Scrollable<TreeLines>;

pub struct Cursor<State> {
    /// The row position because scrollable area only highlights row.
    ///
    /// This should be less than area's height at any time.
    pub y: u16,
    /// Remember the last cursor as much as possible with this state.
    ///
    /// This is mainly used to improve the cursor UX when scrolling
    /// or folding redraw the screen and the same line is available.
    pub state: State,
}

impl<State: Default> Default for Cursor<State> {
    fn default() -> Self {
        Self {
            y: 0,
            state: Default::default(),
        }
    }
}

/// A text panel that can be scrolled and select texts when the cursor is inside of it.
pub struct Scrollable<Ls: Lines> {
    /// Styled texts on each line
    pub lines: Ls,
    /// The start of row to be displayed
    pub start: usize,
    /// The row position where cursor was last time
    pub cursor: Cursor<<Ls::Line as LineState>::State>,
    /// The maximum width among all lines
    pub max_width: u16,
    /// The widget area, usually not the full screen
    pub area: Rect,
}

impl<Ls: Lines> Scrollable<Ls> {
    /// The whole lines including non-visible lines.
    pub fn all_lines(&self) -> &[Ls::Line] {
        &self.lines
    }

    /// The visible lines to be rendered.
    ///
    /// The slice length should be able to cast to u16 without surprising behavior.
    pub fn visible_lines(&self) -> Option<&[Ls::Line]> {
        let total_len = self.lines.len();
        if total_len == 0 {
            return None;
        }
        let end = (self.start + self.area.height as usize).min(total_len);
        self.lines.get(self.start..end)
    }

    /// The line that current cursor on screen points to.
    pub fn get_line_of_current_cursor(&self) -> Option<&Ls::Line> {
        self.visible_lines().and_then(|lines| {
            let cursor = self.cursor.y as usize;
            let line = lines.get(cursor);
            if lines.get(cursor).is_none() {
                error!(
                    "Cursor on row {cursor} is beyond all lines length {}.",
                    self.total_len()
                );
            }
            line
        })
    }

    pub fn total_len(&self) -> usize {
        self.lines.len()
    }
}

impl<Ls> Default for Scrollable<Ls>
where
    Ls: Default + Lines,
    <Ls::Line as LineState>::State: Default,
{
    fn default() -> Self where {
        let (lines, start, cursor, max_windth, area) = Default::default();
        Scrollable {
            lines,
            start,
            cursor,
            max_width: max_windth,
            area,
        }
    }
}

impl<Ls> Scrollable<Ls>
where
    Ls: Default + Lines<Line = TreeLine>,
{
    pub fn new(lines: Ls) -> Result<Self> {
        let w = lines.iter().map(TreeLine::width).max();
        let max_windth = w.ok_or_else(|| err!("The documentation is empty with no items."))?;

        Ok(Self {
            lines,
            max_width: max_windth,
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
        self.max_width = self.lines.iter().map(TreeLine::width).max().unwrap();
    }
}

impl<Ls: Lines> fmt::Debug for Scrollable<Ls> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Scrollable");
        s.field("lines.len", &self.total_len())
            .field("start", &self.start)
            .field("cursor.y", &self.cursor.y)
            .field("max_windth", &self.max_width)
            .field("area", &self.area);
        s.finish()
    }
}
