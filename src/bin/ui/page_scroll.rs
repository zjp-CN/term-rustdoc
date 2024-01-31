use super::Page;

/// Scroll by fixed rows or half/full screen
pub enum ScrollOffset {
    Fixed(usize),
    HalfScreen,
    FullScreen,
}

impl Page {
    pub fn scrolldown_outline(&mut self, offset: ScrollOffset) {
        let outline = &mut self.outline.display;
        let height = outline.area.height as usize;
        let len = outline.len();
        let nrows = match offset {
            ScrollOffset::Fixed(n) => n,
            ScrollOffset::HalfScreen => height / 2,
            ScrollOffset::FullScreen => height,
        };
        let maybe_outside = outline.start + nrows;
        // don't let the last row leave the bottom
        if maybe_outside + height > len {
            return;
        }
        let old_start = outline.start;
        let new_start = (outline.start + nrows).min(len);
        // set new positions for first row and cursor to be displayed
        outline.start = new_start;
        outline.cursor += new_start - old_start;
    }

    pub fn scrollup_outline(&mut self, offset: ScrollOffset) {
        let outline = &mut self.outline.display;
        let height = outline.area.height as usize;
        let nrows = match offset {
            ScrollOffset::Fixed(n) => n,
            ScrollOffset::HalfScreen => height / 2,
            ScrollOffset::FullScreen => height,
        };
        let old_start = outline.start;
        let new_start = outline.start.saturating_sub(nrows);
        // set new positions for first row and cursor to be displayed
        outline.start = new_start;
        outline.cursor -= old_start - new_start;
    }

    pub fn scroll_home_outline(&mut self) {
        self.outline.display.start = 0;
        self.outline.display.cursor = 0;
    }

    pub fn scroll_end_outline(&mut self) {
        let outline = &mut self.outline.display;
        let height = outline.area.height as usize;
        outline.start = outline.len().saturating_sub(height);
        outline.cursor = outline.len().saturating_sub(1);
    }
}
