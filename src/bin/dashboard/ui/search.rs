use ratatui::prelude::{Buffer, Color, Rect, Style};

#[derive(Default)]
pub(super) struct Search {
    pub input: String,
    pub area: Rect,
    source: Source,
}

#[derive(Clone, Copy, Default, Debug)]
enum Source {
    #[default]
    LocalRegistry,
    DataBase,
    Both,
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
        self.update_search();
    }

    // update fuzzy matcher
    fn update_search(&mut self) {
        match self.search.source {
            Source::LocalRegistry => self.registry.update_search(&self.search.input),
            Source::DataBase => self.database.update_search(&self.search.input),
            Source::Both => {
                self.registry.update_search(&self.search.input);
                self.database.update_search(&self.search.input);
            }
        };
    }

    pub fn pop_char(&mut self) {
        self.search.input.pop();
        // update fuzzy matcher
        self.update_search();
    }

    pub fn clear_input(&mut self) {
        self.search.input.clear();
        self.registry.clear_and_reset();

        match self.search.source {
            Source::LocalRegistry => self.registry.clear_and_reset(),
            Source::DataBase => self.database.clear_and_reset(),
            Source::Both => {
                self.registry.clear_and_reset();
                self.database.clear_and_reset();
            }
        };
    }

    pub fn switch_search_source(&mut self) {
        self.search.source = match self.search.source {
            Source::LocalRegistry => Source::DataBase,
            Source::DataBase => Source::Both,
            Source::Both => Source::LocalRegistry,
        };
    }
}
