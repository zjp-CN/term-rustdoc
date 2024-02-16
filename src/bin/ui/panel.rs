use ratatui::prelude::{Color, Rect, Style};

pub const SET: Style = Style::new().bg(Color::Rgb(20, 19, 18)); // #141312
pub const NEW: Style = Style::new();

#[derive(Debug)]
pub enum Panel {
    Outline,
    Content,
    Navigation,
}

impl super::Page {
    /// Responde to mouse click from left button.
    pub fn set_current_panel(&mut self, y: u16, x: u16) {
        fn contain(x: u16, y: u16, area: Rect) -> bool {
            (x >= area.x)
                && (x < area.x + area.width)
                && (y >= area.y)
                && (y < area.y + area.height)
        }
        macro_rules! set {
            (outline) => { set!(#Outline 0 1 2) };
            (content) => { set!(#Content 1 0 2) };
            (navi) => { set!(#Navigation 2 0 1) };
            (#$var:ident $a:tt $b:tt $c:tt) => {{
                let block = (
                    &mut self.outline.border.block,
                    &mut self.content.border.block,
                    &mut self.navi.border.block,
                );
                *block.$a = block.$a.clone().style(SET);
                *block.$b = block.$b.clone().style(NEW);
                *block.$c = block.$c.clone().style(NEW);
                Some(Panel::$var)
            }};
        }
        // Block area covers border and its inner
        self.current = if contain(x, y, self.outline.border.area) {
            self.outline().set_cursor(y);
            self.update_content();
            set!(outline)
        } else if contain(x, y, self.content.border.area) {
            set!(content)
        } else if contain(x, y, self.navi.border.area) {
            if self.heading_jump(y) {
                // succeed to jump to a heading, thus focus on content panel
                set!(content)
            } else {
                set!(navi)
            }
        } else {
            None
        };
        info!(?self.current);
    }
}
