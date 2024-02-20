mod database;
mod ui;

use crate::{fuzzy::Fuzzy, Result};
use ratatui::layout::Rect;

use self::{database::DataBase, ui::UI};

pub struct DashBoard {
    ui: UI,
    db: DataBase,
}

impl DashBoard {
    pub fn new(full: Rect, fuzzy: Fuzzy) -> Result<Self> {
        let ui = UI::new(full, fuzzy);
        let db = DataBase::init()?;
        Ok(DashBoard { ui, db })
    }

    pub fn ui_db(&mut self) -> (&mut UI, &mut DataBase) {
        (&mut self.ui, &mut self.db)
    }
}
