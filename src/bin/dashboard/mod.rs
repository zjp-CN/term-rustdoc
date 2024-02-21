mod ui;

use crate::{event::Sender, fuzzy::Fuzzy, Result};
use ratatui::layout::Rect;

use self::ui::UI;

pub struct DashBoard {
    ui: UI,
}

impl DashBoard {
    pub fn new(full: Rect, fuzzy: Fuzzy, sender: Sender) -> Result<Self> {
        let ui = UI::new(full, fuzzy, sender);
        Ok(DashBoard { ui })
    }

    pub fn ui(&mut self) -> &mut UI {
        &mut self.ui
    }
}
