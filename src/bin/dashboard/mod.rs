mod local_registry;
mod ui;

use crate::{fuzzy::Fuzzy, Result};
use ratatui::layout::Rect;

pub struct DashBoard {
    ui: ui::UI,
}

impl DashBoard {
    pub fn new(full: Rect, fuzzy: Fuzzy) -> Result<Self> {
        let ui = ui::UI::new(full, fuzzy);
        Ok(DashBoard { ui })
    }

    pub fn ui(&mut self) -> &mut ui::UI {
        &mut self.ui
    }
}
