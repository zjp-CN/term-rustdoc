mod database;
mod registry;
mod search;

use self::{database::DataBaseUI, registry::Registry, search::Search};
use crate::{
    database::CachedDocInfo,
    event::Sender,
    fuzzy::Fuzzy,
    ui::{ScrollOffset, Surround},
};
use ratatui::{
    layout::Flex,
    prelude::*,
    widgets::{Block, Borders},
};

#[derive(Default)]
pub struct UI {
    search: Search,
    database: DataBaseUI,
    registry: Registry,
    area: Area,
}

impl UI {
    fn update_area(&mut self, full: Rect) {
        // skip rendering is the same area
        if let Some([db, registry]) = self.area.update(full) {
            // update areas of search, database and registry
            self.search.area = self.area.search_border.inner();
            self.database.set_area(db);
            self.registry.set_area(registry);
        }
    }

    pub fn new(full: Rect, fuzzy: Fuzzy, sender: Sender) -> Self {
        let mut ui = UI {
            database: DataBaseUI::init(sender),
            registry: Registry::new_local(fuzzy),
            search: Search::default(),
            area: Area::default(),
        };
        ui.switch_panel(); // switch to database if caches are not empty
        ui.update_area(full);
        info!("DashBoard UI initialized.");
        ui
    }

    pub fn scroll_down(&mut self) {
        match self.area.current {
            Panel::Database => self
                .database
                .scroll_text()
                .scrolldown(ScrollOffset::HalfScreen),
            Panel::LocalRegistry => self
                .registry
                .scroll_text()
                .scrolldown(ScrollOffset::HalfScreen),
        };
    }

    pub fn scroll_up(&mut self) {
        match self.area.current {
            Panel::Database => self
                .database
                .scroll_text()
                .scrollup(ScrollOffset::HalfScreen),
            Panel::LocalRegistry => self
                .registry
                .scroll_text()
                .scrollup(ScrollOffset::HalfScreen),
        };
    }

    pub fn scroll_home(&mut self) {
        match self.area.current {
            Panel::Database => self.database.scroll_text().scroll_home(),
            Panel::LocalRegistry => self.registry.scroll_text().scroll_home(),
        };
    }

    pub fn scroll_end(&mut self) {
        match self.area.current {
            Panel::Database => self.database.scroll_text().scroll_end(),
            Panel::LocalRegistry => self.registry.scroll_text().scroll_end(),
        };
    }

    pub fn move_backward_cursor(&mut self) {
        match self.area.current {
            Panel::Database => self.database.scroll_text().move_backward_cursor(),
            Panel::LocalRegistry => self.registry.scroll_text().move_backward_cursor(),
        };
    }

    pub fn move_forward_cursor(&mut self) {
        match self.area.current {
            Panel::Database => self.database.scroll_text().move_forward_cursor(),
            Panel::LocalRegistry => self.registry.scroll_text().move_forward_cursor(),
        };
    }

    pub fn compile_or_load_doc(&mut self) {
        match self.area.current {
            Panel::Database => self.database.load_doc(),
            Panel::LocalRegistry => {
                if let Some((pkg_dir, pkg_info)) = self.registry.get_pkg_of_current_cursor() {
                    self.database.compile_doc(pkg_dir, pkg_info);
                }
            }
        }
    }

    pub fn receive_compiled_doc(&mut self, info: CachedDocInfo) {
        self.database.receive_compiled_doc(info);
    }

    pub fn switch_panel(&mut self) {
        if self.database.is_empty() {
            self.area.current = Panel::LocalRegistry;
            return;
        }
        self.area.current = match self.area.current {
            Panel::Database => Panel::LocalRegistry,
            Panel::LocalRegistry => Panel::Database,
        };
    }
}

impl Widget for &mut UI {
    fn render(self, full: Rect, buf: &mut Buffer) {
        self.update_area(full);
        self.area.render(buf);
        self.search.render(buf);
        let [db, reg] = match self.area.current {
            Panel::Database => [true, false],
            Panel::LocalRegistry => [false, true],
        };
        self.database.render(buf, db);
        self.registry.render(buf, reg);
    }
}

#[derive(Default)]
struct Area {
    full: Rect,
    center: Rect,
    search_border: Surround,
    current: Panel,
}

#[derive(Default, Clone, Copy)]
enum Panel {
    Database,
    #[default]
    LocalRegistry,
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let horizontal = Layout::horizontal([Constraint::Percentage(width)]).flex(Flex::Center);
    let vertical = Layout::vertical([Constraint::Percentage(height)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

impl Area {
    fn update(&mut self, full: Rect) -> Option<[Surround; 2]> {
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
        let database = Surround::new(block.clone().title("From Database"), db);
        let registry = Surround::new(block.title("From Local Registry Src Dir"), reg);
        Some([database, registry])
    }

    fn render(&self, buf: &mut Buffer) {
        self.search_border.render(buf);
    }
}
