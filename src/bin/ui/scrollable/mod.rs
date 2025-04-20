use crate::Result;
use ratatui::buffer::Cell;
use ratatui::prelude::{Buffer, Rect};
use rustdoc_types::Id;
use std::fmt;
use term_rustdoc::tree::{TreeLine, TreeLines};

mod generics;
mod interaction;
mod markdown;
mod render;

pub use self::generics::{render_line, LineState, Lines};
pub use self::interaction::{ScrollOffset, Scrollable};
pub use self::markdown::{Headings, MarkdownAndHeading, ScrollHeading, ScrollMarkdown, ScrollText};

/// Scrollable tree view but stored in lines.
pub type ScrollTreeLines = Scroll<TreeLines>;

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
pub struct Scroll<Ls: Lines> {
    /// Styled texts on each line
    pub lines: Ls,
    /// The start of row to be displayed
    pub start: usize,
    /// The row position where cursor was last time
    pub cursor: Cursor<<Ls::Line as LineState>::State>,
    /// The widget area, usually not the full screen
    pub area: Rect,
}

impl<Ls: Lines> Scroll<Ls> {
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

    /// NOTE: y is the row position in screen, not an index of elements.
    pub fn get_line_on_screen(&self, y: u16) -> Option<&Ls::Line> {
        y.checked_sub(self.area.y)
            .and_then(|offset| self.visible_lines()?.get(self.start + offset as usize))
    }

    /// Try to force the current cursor and return the newly current line.
    /// Set and return Some only if the operation is successful.
    /// Force means we won't check_if_can_return_to_previous_cursor.
    ///
    /// NOTE: y is the row position in screen, not an index of elements.
    pub fn force_line_on_screen(&mut self, y: u16) -> Option<&Ls::Line> {
        if !self.is_empty() && y >= self.area.y && y < self.area.y + self.area.height {
            let y = y - self.area.y;
            if let Some(current) = self.lines.get(self.start + y as usize) {
                self.cursor.y = y;
                return Some(current);
            }
        }
        None
    }

    pub fn highlight_current_line(&self, buf: &mut Buffer, mut f: impl FnMut(&mut Cell)) {
        let current = self.start + self.cursor.y as usize;
        // y is always made sure to be in area!
        if self.lines.get(current).is_some() {
            let area = self.area;
            let y = self.cursor.y + area.y;
            for x in area.x..(area.x + area.width) {
                f(buf.get_mut(x, y));
            }
        }
    }

    pub fn total_len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.total_len() == 0
    }
}

impl<Ls> Default for Scroll<Ls>
where
    Ls: Default + Lines,
    <Ls::Line as LineState>::State: Default,
{
    fn default() -> Self where {
        let (lines, start, cursor, area) = Default::default();
        Scroll {
            lines,
            start,
            cursor,
            area,
        }
    }
}

impl<Ls> Scroll<Ls>
where
    Ls: Default + Lines<Line = TreeLine>,
{
    // TODO: rm me
    pub fn new_tree_lines(lines: Ls, height: u16) -> Result<Self> {
        Ok(Self {
            lines,
            area: Rect {
                height, // for getting correct visible_lines and basic layout
                ..Default::default()
            },
            ..Default::default()
        })
    }

    /// Get the item id the current cursor points to.
    /// Non-item node doesn't have an id.
    pub fn get_id(&self) -> Option<Id> {
        self.all_lines()
            .get(self.cursor.y as usize + self.start)
            .and_then(|l| l.id)
    }
}

impl<Ls: Lines> fmt::Debug for Scroll<Ls> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Scrollable");
        s.field("lines.len", &self.total_len())
            .field("start", &self.start)
            .field("cursor.y", &self.cursor.y)
            .field("area", &self.area);
        s.finish()
    }
}
