mod local_registry;
mod ui;

use crate::Result;
use ratatui::layout::Rect;

pub struct DashBoard {
    ui: ui::UI,
}

impl DashBoard {
    pub fn new(full: Rect) -> Result<Self> {
        let ui = ui::UI::new(full);
        Ok(DashBoard { ui })
    }

    pub fn ui(&mut self) -> &mut ui::UI {
        &mut self.ui
    }
}
