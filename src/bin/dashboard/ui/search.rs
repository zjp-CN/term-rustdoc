use crate::ui::Surround;
use ratatui::prelude::{Buffer, Color, Rect, Style};

#[derive(Default)]
pub(super) struct Search {
    pub input: String,
    area: Rect,
    source: Source,
    border: Surround,
}

#[derive(Clone, Copy, Default, Debug)]
enum Source {
    #[default]
    LocalRegistry,
    DataBase,
    Both,
}

impl Search {
    pub fn set_area(&mut self, border: Surround) {
        self.area = border.inner();
        self.border = border;
    }

    fn render_border(&self, buf: &mut Buffer) {
        self.border.render(buf);
        // render border title
        let text = match self.source {
            Source::LocalRegistry => " Search Package In Local Registry ",
            Source::DataBase => " Search Package In Database ",
            Source::Both => " Search Package In Both Local Registry And Database ",
        };
        self.border.render_only_top_left_text(buf, text, 0);
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.render_border(buf);

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
