mod local_registry;
mod ui;

use crate::Result;
use local_registry::LocalRegistry;
use ratatui::layout::Rect;

pub struct DashBoard {
    local_registry: LocalRegistry,
    ui: ui::UI,
}

impl DashBoard {
    pub fn new(full: Rect) -> Result<Self> {
        let local_registry = LocalRegistry::lastest_pkgs_in_latest_registry()?;
        info!(
            "Found {} latest pkgs under {}",
            local_registry.len(),
            local_registry.registry_src_path().display()
        );
        let ui = ui::UI::new(full);
        Ok(DashBoard { local_registry, ui })
    }

    pub fn ui(&mut self) -> &mut ui::UI {
        &mut self.ui
    }
}
