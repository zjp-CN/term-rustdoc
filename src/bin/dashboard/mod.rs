mod database;
mod ui;

use crate::{fuzzy::Fuzzy, Result};
use ratatui::layout::Rect;

use self::ui::UI;

pub struct DashBoard {
    ui: UI,
}

impl DashBoard {
    pub fn new(full: Rect, fuzzy: Fuzzy) -> Result<Self> {
        let ui = UI::new(full, fuzzy);
        Ok(DashBoard { ui })
    }

    pub fn ui(&mut self) -> &mut UI {
        &mut self.ui
    }
}
