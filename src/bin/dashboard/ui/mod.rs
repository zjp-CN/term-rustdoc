mod database;
mod registry;
mod search;

use self::{database::DataBaseUI, registry::Registry, search::Search};
use crate::{
    database::{CachedDocInfo, PkgKey},
    event::Sender,
    frame::centered_rect,
    fuzzy::Fuzzy,
    ui::{ScrollOffset, Surround},
};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};
use term_rustdoc::tree::CrateDoc;

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
        if let Some([search, db, registry]) = self.area.update(full) {
            // update areas of search, database and registry
            self.search.set_area(search);
            self.database.set_area(db);
            self.registry.set_area(registry);
        }
    }

    pub fn new(full: Rect, fuzzy: Fuzzy, sender: Sender) -> Self {
        let mut ui = UI {
            database: DataBaseUI::init(sender, fuzzy.clone()),
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

    pub fn switch_sort(&mut self) {
        match self.area.current {
            Panel::Database => self.database.switch_sort(),
            Panel::LocalRegistry => (),
        }
    }

    pub fn get_loaded_doc(&self, key: &PkgKey) -> Option<CrateDoc> {
        self.database.get_loaded_doc(key)
    }

    /// the full screen area
    pub fn get_full_area(&self) -> Rect {
        self.area.full
    }

    pub fn downgrade(&mut self) {
        self.database.downgrade();
    }

    /// Returns true for hinting Frame can switch to Page, because no mouse interaction in DashBoard.
    pub fn update_for_mouse(&mut self, event: MouseEvent) -> bool {
        match event.kind {
            MouseEventKind::ScrollDown => match self.area.current {
                Panel::Database => self
                    .database
                    .scroll_text()
                    .scrolldown(ScrollOffset::Fixed(5)),
                Panel::LocalRegistry => self
                    .registry
                    .scroll_text()
                    .scrolldown(ScrollOffset::Fixed(5)),
            },
            MouseEventKind::ScrollUp => match self.area.current {
                Panel::Database => self.database.scroll_text().scrollup(ScrollOffset::Fixed(5)),
                Panel::LocalRegistry => {
                    self.registry.scroll_text().scrollup(ScrollOffset::Fixed(5))
                }
            },
            MouseEventKind::Down(MouseButton::Left) => {
                let position = (event.column, event.row);

                if !self.area.center.contains(position.into()) {
                    return true;
                }

                let registry = self.registry.scroll_text();
                if registry.area.contains(position.into()) {
                    let y = registry.area.y;
                    registry.set_cursor(event.row.saturating_sub(y));
                    self.area.current = Panel::LocalRegistry;
                    return false;
                }

                let db = self.database.scroll_text();
                if db.area.contains(position.into()) {
                    let y = db.area.y;
                    db.set_cursor(event.row.saturating_sub(y));
                    self.area.current = Panel::Database;
                }
            }
            MouseEventKind::Down(MouseButton::Right) => {
                let position = (event.column, event.row);
                let db = self.database.scroll_text();
                if db.area.contains(position.into()) {
                    let y = db.area.y;
                    db.set_cursor(event.row.saturating_sub(y));
                    self.area.current = Panel::Database;
                    self.database.downgrade();
                    return false;
                }
                return true;
            }
            _ => (),
        };
        false
    }

    pub fn contains(&self, position: (u16, u16)) -> bool {
        self.area.center.contains(position.into())
    }
}

impl Widget for &mut UI {
    fn render(self, full: Rect, buf: &mut Buffer) {
        self.update_area(full);
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
    current: Panel,
}

#[derive(Default, Clone, Copy)]
enum Panel {
    Database,
    #[default]
    LocalRegistry,
}

impl Area {
    /// returns borders for search, database and registry
    fn update(&mut self, full: Rect) -> Option<[Surround; 3]> {
        if self.full == full {
            return None;
        }
        self.full = full;
        self.center = centered_rect(full, 80, 80);
        // database area: lined borders and one inner line
        let [search, db_reg] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(self.center);
        let block = Block::new().borders(Borders::all());
        let search = Surround::new(block.clone(), search);
        let half = Constraint::Percentage(50);
        let [db, reg] = Layout::horizontal([half, half]).areas(db_reg);
        let database = Surround::new(block.clone().title(" From Database "), db);
        let registry = Surround::new(block.title(" From Local Registry Src Dir "), reg);
        Some([search, database, registry])
    }
}
