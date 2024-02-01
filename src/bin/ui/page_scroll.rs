use super::{Page, ScrollTreeLines};

/// Scroll by fixed rows or half/full screen
pub enum ScrollOffset {
    Fixed(usize),
    HalfScreen,
    FullScreen,
}

/// Scrolling
impl Page {
    fn outline(&mut self) -> &mut ScrollTreeLines {
        &mut self.outline.display
    }

    pub fn scrolldown(&mut self, offset: ScrollOffset) {
        self.outline().scrolldown(offset);
    }

    pub fn scrollup(&mut self, offset: ScrollOffset) {
        self.outline().scrollup(offset);
    }

    pub fn scroll_home(&mut self) {
        self.outline().scroll_home();
    }

    pub fn scroll_end(&mut self) {
        self.outline().scroll_end();
    }
}

/// Cursor movement
impl Page {
    pub fn move_forward_cursor(&mut self) {
        self.outline().move_forward_cursor();
    }

    pub fn move_backward_cursor(&mut self) {
        self.outline().move_backward_cursor();
    }

    pub fn move_top_cursor(&mut self) {
        self.outline().move_top_cursor();
    }

    pub fn move_bottom_cursor(&mut self) {
        self.outline().move_bottom_cursor();
    }

    pub fn move_middle_cursor(&mut self) {
        self.outline().move_middle_cursor();
    }
}