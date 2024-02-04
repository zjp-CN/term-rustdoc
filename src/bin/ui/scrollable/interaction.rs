use super::{LineState, Scrollable};
use crate::ui::ScrollOffset;
use std::ops::Deref;

/// Scrolling
impl<L: LineState, Lines: Deref<Target = [L]>> Scrollable<Lines, L> {
    pub fn scrolldown(&mut self, offset: ScrollOffset) {
        let height = self.area.height as usize;
        let len = self.total_len();
        let nrows = match offset {
            ScrollOffset::Fixed(n) => n,
            ScrollOffset::HalfScreen => height.saturating_sub(1) / 2,
            ScrollOffset::FullScreen => height,
        };
        let maybe_outside = self.start + nrows;
        // don't let the last row leave the bottom
        if maybe_outside > len {
            return;
        }
        // set new positions for first row to be displayed
        self.set_cursor_state();
        // let previous = self.start;
        self.start = (self.start + nrows).min(len.saturating_sub(height));
        self.check_if_can_return_to_previous_cursor();
    }

    pub fn scrollup(&mut self, offset: ScrollOffset) {
        let height = self.area.height as usize;
        let nrows = match offset {
            ScrollOffset::Fixed(n) => n,
            ScrollOffset::HalfScreen => height.saturating_sub(1) / 2,
            ScrollOffset::FullScreen => height,
        };
        // set new positions for first row to be displayed
        self.set_cursor_state();
        // let previous = self.start;
        self.start = self.start.saturating_sub(nrows);
        self.check_if_can_return_to_previous_cursor();
    }

    pub fn scroll_home(&mut self) {
        self.start = 0;
        self.cursor.y = 0;
        self.set_cursor_state();
    }

    pub fn scroll_end(&mut self) {
        let height = self.area.height;
        let heightu = height as usize;
        let len = self.total_len();
        self.start = len.saturating_sub(heightu);
        let bot = if len < heightu { len as u16 } else { height };
        self.cursor.y = bot.saturating_sub(1);
        self.set_cursor_state();
    }
}

/// Cursor state
impl<L: LineState, Lines: Deref<Target = [L]>> Scrollable<Lines, L> {
    /// Check the cursor position and its state after redrawing.
    ///
    /// This also moves the cursor in two ways:
    /// * coerce the cursor to the bottom line if it lies outside the height
    /// * coerce the cursor to last position if the screen contains the previous state
    pub fn check_if_can_return_to_previous_cursor(&mut self) {
        if self.cursor.y >= self.area.height {
            error!(
                "This is a bug because cursor is beyond height of drawing area. \
                 Let's coerce it to the bottom line."
            );
            self.cursor.y = self.area.height.saturating_sub(1);
        }
        if let Some(lines) = self.visible_lines() {
            if let Some(pos) = lines
                .iter()
                .enumerate()
                .find_map(|(pos, line)| line.is_identical(&self.cursor.state).then_some(pos))
            {
                self.cursor.y = pos as u16;
            }
        }
    }

    fn set_cursor_state(&mut self) {
        if let Some(l) = self.get_line_of_current_cursor() {
            self.cursor.state = l.state();
        }
    }
}

/// Cursor movement
impl<L: LineState, Lines: Deref<Target = [L]>> Scrollable<Lines, L> {
    pub fn move_forward_cursor(&mut self) {
        let height = self.area.height;
        let reach_sceen_bottom = (self.cursor.y + 1) == height;
        if reach_sceen_bottom {
            // scroll down for a new line
            self.scrolldown(ScrollOffset::Fixed(1));
        }
        // still be careful to cross the screen's bottom
        self.cursor.y += if (self.cursor.y + 1) == height { 0 } else { 1 };
        self.set_cursor_state();
    }

    pub fn move_backward_cursor(&mut self) {
        if self.cursor.y == 0 {
            // scroll up for a new line
            self.scrollup(ScrollOffset::Fixed(1));
        }
        // still be careful to cross the screen's top
        self.cursor.y = self.cursor.y.saturating_sub(1);
        self.set_cursor_state();
    }

    pub fn move_top_cursor(&mut self) {
        self.cursor.y = 0;
        self.set_cursor_state();
    }

    pub fn move_bottom_cursor(&mut self) {
        self.cursor.y = (self.area.height as usize)
            .min(self.total_len())
            .saturating_sub(1) as u16;
        self.set_cursor_state();
    }

    pub fn move_middle_cursor(&mut self) {
        self.move_bottom_cursor();
        self.cursor.y /= 2;
        self.set_cursor_state();
    }
}
