use super::{scrollable::ScrollText, Component, Page, ScrollTreeLines};

/// Scroll by fixed rows or half/full screen
pub enum ScrollOffset {
    Fixed(usize),
    HalfScreen,
    FullScreen,
}

macro_rules! current {
    ($self:ident: $outline:block; $content:block $(;)?) => {
        match $self.current {
            Some(Component::Outline) => $outline,
            Some(Component::Content) => $content,
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
                self.outline().scrolldown(offset);
                self.update_content();
            };
            {
                self.content().scrolldown(offset);
            }
        }
    }

    pub fn scrollup(&mut self, offset: ScrollOffset) {
        current! { self :
            {
                self.outline().scrollup(offset);
                self.update_content();
            };
            {
                self.content().scrollup(offset)
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

    /// update content's StyledLines aftet setting the cursor
    pub fn update_content(&mut self) {
        if let Some(id) = self.outline.display.get_id() {
            if self.content.display.lines.update_doc(id) {
                // Only reset start after the update.
                // TODO: would it be better to remember the
                // view position if doc is able to be cached?
                self.content.display.start = 0;
            }
        } else {
            self.content.display.lines.reset_doc();
        }
    }
}
