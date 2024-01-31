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
        outline.start = (outline.start + nrows).min(len);
    }

    pub fn scrollup_outline(&mut self, offset: ScrollOffset) {
        let outline = &mut self.outline.display;
        let height = outline.area.height as usize;
        let nrows = match offset {
            ScrollOffset::Fixed(n) => n,
            ScrollOffset::HalfScreen => height / 2,
            ScrollOffset::FullScreen => height,
        };
        outline.start = outline.start.saturating_sub(nrows);
    }

    pub fn scroll_home_outline(&mut self) {
        self.outline.display.start = 0;
    }

    pub fn scroll_end_outline(&mut self) {
        let outline = &mut self.outline.display;
        let height = outline.area.height as usize;
        outline.start = outline.len().saturating_sub(height);
    }
}
