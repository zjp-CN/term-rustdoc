use super::{Page, Panel};
use crate::ui::scrollable::{ScrollOffset, ScrollText, ScrollTreeLines};

macro_rules! current {
    ($self:ident: $outline:block; $content:block $(;)?) => {
        match $self.current {
            Some(Panel::Outline) => $outline,
            Some(Panel::Content) => $content,
            _ => (),
        };
    };
}

/// Scrolling
impl Page {
    pub(super) fn outline(&mut self) -> &mut ScrollTreeLines {
        &mut self.outline.display
    }

    pub(super) fn content(&mut self) -> &mut ScrollText {
        &mut self.content.display
    }

    pub fn scrolldown(&mut self, offset: ScrollOffset) {
        current! { self :
            {
                self.outline().scroll_down(offset);
                self.update_content();
            };
            {
                self.content().scroll_down(offset);
            }
        }
    }

    pub fn scrollup(&mut self, offset: ScrollOffset) {
        current! { self :
            {
                self.outline().scroll_up(offset);
                self.update_content();
            };
            {
                self.content().scroll_up(offset)
            }
        }
    }

    pub fn scroll_home(&mut self) {
        current! { self :
            {
                self.outline().scroll_home();
                self.update_content();
            };
            {
                self.content().scroll_home();
            }
        }
    }

    pub fn scroll_end(&mut self) {
        current! { self :
            {
                self.outline().scroll_end();
                self.update_content();
            };
            {
                self.content().scroll_end();
            }
        }
    }
}

/// Cursor movement
/// TODO:at present, only outline cursor is moveable,
/// we need to wait for cursor rendering in content
/// to implement content cursor movement.
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

    /// update content's StyledLines and Headings aftet setting the cursor
    pub fn update_content(&mut self) {
        if let Some(id) = self.outline.display.get_id() {
            if let Some(headings) = self.content.display.update_doc(id) {
                // Only reset start after the update.
                // TODO: would it be better to remember the
                // view position if doc is able to be cached?
                self.content.display.start = 0;
                self.navi.heading().update_headings(headings);
            }
        } else {
            self.content.display.lines.reset_doc();
            self.navi.heading().lines = Default::default();
        }
        self.update_navi();
    }

    fn update_navi(&mut self) {
        self.navi.set_item_inner(self.outline.display.get_id());
    }
}

impl Page {
    pub fn heading_jump(&mut self, y: u16) -> bool {
        const MARGIN: usize = 3;
        if let Some(heading) = self.navi.heading().get_line_on_screen(y) {
            // set the upper bound: usually no need to use this, but who knows if y points
            // to a line out of the doc range.
            let limit = self.content.display.total_len().saturating_sub(MARGIN);
            self.content().start = heading.jump_row_start().saturating_sub(MARGIN).min(limit);
            return true;
        }
        false
    }

    pub fn toggle_sytect(&mut self) {
        self.content().lines.toggle_sytect();
        self.update_content();
    }
}
