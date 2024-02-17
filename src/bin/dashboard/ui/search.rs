use ratatui::prelude::{Buffer, Color, Rect, Style};

#[derive(Default)]
pub(super) struct Search {
    pub input: String,
    pub area: Rect,
}

impl Search {
    pub fn render(&self, buf: &mut Buffer) {
        let Rect { x, y, width, .. } = self.area;
        let width = width.saturating_sub(1) as usize;
        let mut text = self.input.as_str();
        // show end half if the input exceeds the width
        text = &text[text.len().saturating_sub(width)..];
        let (x, _) = buf.set_stringn(x, y, text, width, Style::new());

        // the last width is used as cursor
        let cursor = Style {
            bg: Some(Color::Green),
            ..Default::default()
        };
        buf.set_stringn(x, y, " ", 1, cursor);
    }
}

/// Search related
impl super::UI {
    pub fn push_char(&mut self, ch: char) {
        self.search.input.push(ch);
    }

    pub fn pop_char(&mut self) {
        self.search.input.pop();
    }
}
