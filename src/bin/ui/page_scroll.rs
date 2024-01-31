use std::usize;

use super::{Page, Scrollable};
use term_rustdoc::tree::TreeLines;

/// Scroll by fixed rows or half/full screen
pub enum ScrollOffset {
    Fixed(usize),
    HalfScreen,
    FullScreen,
}

/// Scrolling
impl Page {
    fn outline(&mut self) -> &mut Scrollable<TreeLines> {
        &mut self.outline.display
    }

    pub fn scrolldown_outline(&mut self, offset: ScrollOffset) {
        let outline = self.outline();
        let height = outline.area.height as usize;
        let len = outline.len();
        let nrows = match offset {
            ScrollOffset::Fixed(n) => n,
            ScrollOffset::HalfScreen => height / 2,
            ScrollOffset::FullScreen => height,
        };
        let maybe_outside = outline.start + nrows;
        // don't let the last row leave the bottom
        if maybe_outside > len {
            return;
        }
        // set new positions for first row to be displayed
        outline.start = (outline.start + nrows).min(len.saturating_sub(height));
    }

    pub fn scrollup_outline(&mut self, offset: ScrollOffset) {
        let outline = self.outline();
        let height = outline.area.height as usize;
        let nrows = match offset {
            ScrollOffset::Fixed(n) => n,
            ScrollOffset::HalfScreen => height / 2,
            ScrollOffset::FullScreen => height,
        };
        // set new positions for first row to be displayed
        outline.start = outline.start.saturating_sub(nrows);
    }

    pub fn scroll_home_outline(&mut self) {
        self.outline().start = 0;
        self.outline().cursor = 0;
    }

    pub fn scroll_end_outline(&mut self) {
        let outline = self.outline();
        let height = outline.area.height;
        let heightu = height as usize;
        let len = outline.len();
        outline.start = len.saturating_sub(heightu);
        let bot = if len < heightu { len as u16 } else { height };
        outline.cursor = bot.saturating_sub(1);
    }
}

/// Cursor movement
impl Page {
    pub fn move_forward_cursor_outline(&mut self) {
        let outline = self.outline();
        let height = outline.area.height;
        let reach_sceen_bottom = (outline.cursor + 1) == height;
        if reach_sceen_bottom {
            // scroll down for a new line
            self.scrolldown_outline(ScrollOffset::Fixed(1));
        }
        let outline = self.outline();
        // still be careful to cross the screen's bottom
        outline.cursor += if (outline.cursor + 1) == height { 0 } else { 1 };
    }

    pub fn move_backward_cursor_outline(&mut self) {
        let outline = self.outline();
        if outline.cursor == 0 {
            // scroll up for a new line
            self.scrollup_outline(ScrollOffset::Fixed(1));
        }
        let outline = self.outline();
        // still be careful to cross the screen's top
        outline.cursor = outline.cursor.saturating_sub(1);
    }
}
