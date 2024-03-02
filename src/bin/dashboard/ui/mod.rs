mod database;
mod registry;
mod search;
mod version_features;

use self::{
    database::DataBaseUI, registry::Registry, search::Search, version_features::VersionFeatures,
};
use crate::{
    database::{CachedDocInfo, PkgKey},
    event::Sender,
    frame::centered_rect,
    fuzzy::Fuzzy,
    ui::{Scroll, ScrollOffset, Surround},
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
    ver_feat: VersionFeatures,
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
        self.ver_feat.update_area(self.area.center);
    }

    pub fn new(full: Rect, fuzzy: Fuzzy, sender: Sender) -> Self {
        let mut ui = UI {
            database: DataBaseUI::init(sender, fuzzy.clone()),
            registry: Registry::new_local(fuzzy),
            ..Default::default()
        };
        ui.switch_panel(); // switch to database if caches are not empty
        ui.update_area(full);
        info!("DashBoard UI initialized.");
        ui
    }

    pub fn scroll_text(&mut self) -> &mut dyn Scroll {
        match self.area.current {
            Panel::Database => self.database.scroll_text() as &mut dyn Scroll,
            Panel::LocalRegistry => self.registry.scroll_text(),
            Panel::VersionFeatures => self.ver_feat.scroll_text(),
        }
    }

    pub fn scroll_down(&mut self) {
        self.scroll_text().scroll_down(ScrollOffset::HalfScreen);
    }

    pub fn scroll_up(&mut self) {
        self.scroll_text().scroll_up(ScrollOffset::HalfScreen);
    }

    pub fn scroll_home(&mut self) {
        self.scroll_text().scroll_home();
    }

    pub fn scroll_end(&mut self) {
        self.scroll_text().scroll_end();
    }

    pub fn move_backward_cursor(&mut self) {
        self.scroll_text().move_backward_cursor();
    }

    pub fn move_forward_cursor(&mut self) {
        self.scroll_text().move_forward_cursor();
    }

    pub fn compile_or_load_doc(&mut self, y: Option<u16>) {
        match self.area.current {
            Panel::Database => self.database.load_doc(y),
            Panel::LocalRegistry => {
                if let Some(pkg_info) = self.registry.get_pkg(y) {
                    if !self.ver_feat.features().is_same_pkg(&pkg_info) {
                        let all = self
                            .registry
                            .scroll_text()
                            .lines
                            .get_all_version(pkg_info.name());
                        self.ver_feat = VersionFeatures::new(pkg_info, all, self.area.center);
                    }
                    if self.ver_feat.features().skip_selection() {
                        // no feature to select, thus compile the doc directly
                        if let Some(pkg) = self.ver_feat.features().pkg_with_features() {
                            self.database.compile_doc(pkg)
                        }
                    } else {
                        self.area.current = Panel::VersionFeatures;
                    }
                }
            }
            Panel::VersionFeatures => {
                self.ver_feat.features().toggle();
            }
        }
    }

    fn comfirm_features_and_compile_doc(&mut self) {
        if let Some(pkg) = self.ver_feat.features().pkg_with_features() {
            self.database.compile_doc(pkg);
            self.area.current = Panel::Database;
        }
    }

    pub fn respond_to_char(&mut self, ch: char) {
        match self.area.current {
            Panel::VersionFeatures => {
                if ch == ' ' {
                    self.comfirm_features_and_compile_doc();
                }
            }
            _ => self.push_char(ch),
        };
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
            Panel::VersionFeatures => Panel::LocalRegistry,
        };
    }

    pub fn switch_sort(&mut self) {
        if let Panel::Database = self.area.current {
            self.database.switch_sort()
        }
    }

    pub fn get_loaded_doc(&self, key: &PkgKey) -> Option<CrateDoc> {
        self.database.get_loaded_doc(key)
    }

    /// the full screen area
    pub fn get_full_area(&self) -> Rect {
        self.area.full
    }

    pub fn downgrade(&mut self, y: Option<u16>) {
        self.database.downgrade(y);
    }

    /// Returns true for hinting Frame can switch to Page, because no mouse interaction in DashBoard.
    pub fn update_for_mouse(&mut self, event: MouseEvent) -> bool {
        match event.kind {
            MouseEventKind::ScrollDown => self.scroll_text().scroll_down(ScrollOffset::Fixed(5)),
            MouseEventKind::ScrollUp => self.scroll_text().scroll_up(ScrollOffset::Fixed(5)),
            MouseEventKind::Down(MouseButton::Left) => {
                let position = (event.column, event.row);

                if !self.area.center.contains(position.into()) {
                    return true;
                }

                if matches!(self.area.current, Panel::VersionFeatures) {
                    let features = &mut self.ver_feat.features().scroll_text();
                    let y = features.area.y;
                    features.set_cursor(event.row.saturating_sub(y));
                    return false;
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
                    self.database.downgrade(Some(event.row));
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

        let [db, reg] = match self.area.current {
            Panel::Database => [true, false],
            Panel::LocalRegistry => [false, true],
            Panel::VersionFeatures => {
                self.ver_feat.render(buf);
                return;
            }
        };
        self.search.render(buf);
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
    VersionFeatures,
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
