mod registry;
mod search;

use self::{registry::Registry, search::Search};
use crate::{
    fuzzy::Fuzzy,
    local_registry::PkgInfo,
    ui::{ScrollOffset, Surround},
};
use ratatui::{
    layout::Flex,
    prelude::*,
    widgets::{Block, Borders},
};
use std::path::PathBuf;

#[derive(Default)]
pub struct UI {
    search: Search,
    // database: ScrollText,
    registry: Registry,
    area: Area,
}

impl UI {
    fn update_area(&mut self, full: Rect) {
        // skip rendering is the same area
        if let Some(registry) = self.area.update(full) {
            // update areas of search, database and registry
            self.search.area = self.area.search_border.inner();
            // self.database.area = self.area.database_border.inner();
            self.registry.set_area(registry);
        }
    }

    pub fn new(full: Rect, fuzzy: Fuzzy) -> Self {
        let mut ui = UI {
            registry: Registry::new_local(fuzzy),
            ..Default::default()
        };
        ui.update_area(full);
        ui
    }

    pub fn scroll_down(&mut self) {
        self.registry
            .scroll_text()
            .scrolldown(ScrollOffset::HalfScreen);
    }

    pub fn scroll_up(&mut self) {
        self.registry
            .scroll_text()
            .scrollup(ScrollOffset::HalfScreen);
    }

    pub fn scroll_home(&mut self) {
        self.registry.scroll_text().scroll_home();
    }

    pub fn scroll_end(&mut self) {
        self.registry.scroll_text().scroll_end();
    }

    pub fn move_backward_cursor(&mut self) {
        self.registry.scroll_text().move_backward_cursor();
    }

    pub fn move_forward_cursor(&mut self) {
        self.registry.scroll_text().move_forward_cursor();
    }

    pub fn get_local_pkg_info(&self) -> Option<(PathBuf, PkgInfo)> {
        self.registry.get_pkg_of_current_cursor()
    }
}

impl Widget for &mut UI {
    fn render(self, full: Rect, buf: &mut Buffer) {
        self.update_area(full);
        self.area.render(buf);
        self.search.render(buf);
        // self.database.render(buf);
        self.registry.render(buf);
    }
}

#[derive(Default)]
struct Area {
    full: Rect,
    center: Rect,
    search_border: Surround,
    database_border: Surround,
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let horizontal = Layout::horizontal([Constraint::Percentage(width)]).flex(Flex::Center);
    let vertical = Layout::vertical([Constraint::Percentage(height)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

impl Area {
    fn update(&mut self, full: Rect) -> Option<Surround> {
        if self.full == full {
            return None;
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
        Some(Surround::new(
            block.title("From Local Registry Src Dir"),
            reg,
        ))
    }

    fn render(&self, buf: &mut Buffer) {
        self.search_border.render(buf);
        self.database_border.render(buf);
    }
}
