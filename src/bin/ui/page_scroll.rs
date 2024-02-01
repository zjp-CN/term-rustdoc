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
        self.update_content();
    }

    pub fn scrollup(&mut self, offset: ScrollOffset) {
        self.outline().scrollup(offset);
        self.update_content();
    }

    pub fn scroll_home(&mut self) {
        self.outline().scroll_home();
        self.update_content();
    }

    pub fn scroll_end(&mut self) {
        self.outline().scroll_end();
        self.update_content();
    }
}

/// Cursor movemen
impl Page {
    pub fn move_forward_cursor(&mut self) {
        self.outline().move_forward_cursor();
        self.update_content();
    }

    pub fn move_backward_cursor(&mut self) {
        self.outline().move_backward_cursor();
        self.update_content();
    }

    pub fn move_top_cursor(&mut self) {
        self.outline().move_top_cursor();
        self.update_content();
    }

    pub fn move_bottom_cursor(&mut self) {
        self.outline().move_bottom_cursor();
        self.update_content();
    }

    pub fn move_middle_cursor(&mut self) {
        self.outline().move_middle_cursor();
        self.update_content();
    }

    /// update content's StyledLines aftet setting the cursor
    pub fn update_content(&mut self) {
        if let Some(id) = self.outline.display.get_id() {
            self.content.display.lines.update_doc(id);
        } else {
            self.content.display.lines.reset_doc();
        }
    }
}
