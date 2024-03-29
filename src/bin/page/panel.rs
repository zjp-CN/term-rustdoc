use crate::color::{NEW, SET};

#[derive(Debug)]
pub enum Panel {
    Outline,
    Content,
    Navigation,
}

impl super::Page {
    /// Responde to mouse click from left button.
    pub fn set_current_panel(&mut self, y: u16, x: u16) {
        macro_rules! set {
            (outline) => { set!(#Outline 0 1 2) };
            (content) => { set!(#Content 1 0 2) };
            (navi) => { set!(#Navigation 2 0 1) };
            (#$var:ident $a:tt $b:tt $c:tt) => {{
                let block = (
                    self.outline.border.block_mut(),
                    self.content.border.block_mut(),
                    self.navi.border().block_mut(),
                );
                *block.$a = block.$a.clone().style(SET);
                *block.$b = block.$b.clone().style(NEW);
                *block.$c = block.$c.clone().style(NEW);
                Some(Panel::$var)
            }};
        }
        let position = (x, y).into();
        // Block area covers border and its inner
        self.current = if self.outline.border.area().contains(position) {
            self.outline().set_cursor(y);
            self.update_content();
            set!(outline)
        } else if self.content.border.area().contains(position) {
            if let Some(id) = self.content.jumpable_id(x, y) {
                self.jump_to_id(&id);
            }
            set!(content)
        } else if self.navi.contains(position) {
            if self.heading_jump(y) {
                // succeed to jump to a heading, thus focus on content panel
                set!(content)
            } else if let Some(action) = self.navi.update_outline(y) {
                self.outline.action(action);
                self.update_area_inner(self.area);
                set!(outline)
            } else {
                set!(navi)
            }
        } else {
            None
        };
        info!(?self.current);
    }

    pub fn set_next_action(&mut self) {
        let next_action = self.navi.next_action();
        debug!(?next_action);
        if let Some(action) = next_action {
            self.outline.action(action);
            self.update_area_inner(self.area);
            // set!(outline)
        }
    }

    pub fn set_previous_action(&mut self) {
        let next_action = self.navi.previous_action();
        debug!(?next_action);
        if let Some(action) = next_action {
            self.outline.action(action);
            self.update_area_inner(self.area);
            // set!(outline)
        }
    }
}
