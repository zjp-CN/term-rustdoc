use crate::ui::{ScrollText, Surround};
use ratatui::{
    layout::Flex,
    prelude::*,
    widgets::{Block, Borders},
};

#[derive(Default)]
pub struct UI {
    search: Search,
    database: ScrollText,
    registry: ScrollText,
    area: Area,
}

impl UI {
    fn update_area(&mut self, full: Rect) {
        // skip rendering is the same area
        if !self.area.update(full) {
            return;
        }
        // update areas of search, database and registry
        self.search.area = self.area.search_border.inner();
        self.database.area = self.area.database_border.inner();
        self.registry.area = self.area.registry_border.inner();
    }

    pub fn new(full: Rect) -> Self {
        let mut ui = UI::default();
        ui.update_area(full);
        ui
    }
}

impl Widget for &mut UI {
    fn render(self, full: Rect, buf: &mut Buffer) {
        self.update_area(full);
        self.area.render(buf);
        self.search.render(buf);
        self.database.render(buf);
        self.registry.render(buf);
    }
}

#[derive(Default)]
struct Area {
    full: Rect,
    center: Rect,
    search_border: Surround,
    database_border: Surround,
    registry_border: Surround,
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let horizontal = Layout::horizontal([Constraint::Percentage(width)]).flex(Flex::Center);
    let vertical = Layout::vertical([Constraint::Percentage(height)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

impl Area {
    fn update(&mut self, full: Rect) -> bool {
        if self.full == full {
            return false;
        }
        self.full = full;
        self.center = centered_rect(full, 80, 80);
        // database area: lined borders and one inner line
        let [search, db_reg] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(self.center);
        let block = Block::new().borders(Borders::all());
        self.search_border = Surround::new(block.clone().title("Search Package"), search);
        let half = Constraint::Percentage(50);
        let [db, reg] = Layout::horizontal([half, half]).areas(db_reg);
        self.database_border = Surround::new(block.clone().title("From Database"), db);
        self.registry_border = Surround::new(block.title("From Local Registry Src Dir"), reg);
        true
    }

    fn render(&self, buf: &mut Buffer) {
        self.search_border.render(buf);
        self.database_border.render(buf);
        self.registry_border.render(buf);
    }
}

#[derive(Default)]
struct Search {
    input: String,
    area: Rect,
}

impl Search {
    fn render(&self, buf: &mut Buffer) {
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
impl UI {
    pub fn push_char(&mut self, ch: char) {
        self.search.input.push(ch);
    }

    pub fn pop_char(&mut self) {
        self.search.input.pop();
    }
}
