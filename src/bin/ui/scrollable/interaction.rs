use super::Scrollable;
use crate::ui::ScrollOffset;
use term_rustdoc::tree::TreeLine;

/// Scrolling
impl<Line: AsRef<[TreeLine]>> Scrollable<Line> {
    pub fn scrolldown(&mut self, offset: ScrollOffset) {
        let height = self.area.height as usize;
        let len = self.len();
        let nrows = match offset {
            ScrollOffset::Fixed(n) => n,
            ScrollOffset::HalfScreen => height / 2,
            ScrollOffset::FullScreen => height,
        };
        let maybe_outside = self.start + nrows;
        // don't let the last row leave the bottom
        if maybe_outside > len {
            return;
        }
        // set new positions for first row to be displayed
        self.start = (self.start + nrows).min(len.saturating_sub(height));
    }

    pub fn scrollup(&mut self, offset: ScrollOffset) {
        let height = self.area.height as usize;
        let nrows = match offset {
            ScrollOffset::Fixed(n) => n,
            ScrollOffset::HalfScreen => height / 2,
            ScrollOffset::FullScreen => height,
        };
        // set new positions for first row to be displayed
        self.start = self.start.saturating_sub(nrows);
    }

    pub fn scroll_home(&mut self) {
        self.start = 0;
        self.cursor = 0;
    }

    pub fn scroll_end(&mut self) {
        let height = self.area.height;
        let heightu = height as usize;
        let len = self.len();
        self.start = len.saturating_sub(heightu);
        let bot = if len < heightu { len as u16 } else { height };
        self.cursor = bot.saturating_sub(1);
    }
}

/// Cursor movement
impl<Line: AsRef<[TreeLine]>> Scrollable<Line> {
    pub fn move_forward_cursor(&mut self) {
        let height = self.area.height;
        let reach_sceen_bottom = (self.cursor + 1) == height;
        if reach_sceen_bottom {
            // scroll down for a new line
            self.scrolldown(ScrollOffset::Fixed(1));
        }
        // still be careful to cross the screen's bottom
        self.cursor += if (self.cursor + 1) == height { 0 } else { 1 };
    }

    pub fn move_backward_cursor(&mut self) {
        if self.cursor == 0 {
            // scroll up for a new line
            self.scrollup(ScrollOffset::Fixed(1));
        }
        // still be careful to cross the screen's top
        self.cursor = self.cursor.saturating_sub(1);
    }
}
