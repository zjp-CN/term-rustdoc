use crate::{
    dashboard::local_registry::{LocalRegistry, PkgNameVersion},
    ui::{render_line, LineState, Scrollable},
};
use ratatui::prelude::*;

impl LineState for PkgNameVersion {
    type State = Self;

    fn state(&self) -> Self::State {
        self.clone()
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        self == state
    }
}

#[derive(Default)]
pub struct Registry {
    pub text: Scrollable<LocalRegistry>,
}

impl Registry {
    pub fn new_local() -> Self {
        let registry = match LocalRegistry::lastest_pkgs_in_latest_registry() {
            Ok(registry) => registry,
            Err(err) => {
                error!("{err}");
                return Registry::default();
            }
        };
        info!(
            "Found {} latest pkgs under {}",
            registry.len(),
            registry.registry_src_path().display()
        );
        Registry {
            text: Scrollable {
                lines: registry,
                ..Default::default()
            },
        }
    }

    pub fn set_area(&mut self, area: Rect) {
        self.text.area = area;
    }

    pub fn scroll_text(&mut self) -> &mut Scrollable<LocalRegistry> {
        &mut self.text
    }

    pub fn render(&self, buf: &mut Buffer) {
        let text = &self.text;
        let Rect {
            x, mut y, width, ..
        } = text.area;
        let width = width as usize;
        if let Some(lines) = text.visible_lines() {
            let style = Style::new();
            for line in lines {
                render_line(Some((line.name(), style)), buf, x, y, width);
                y += 1;
            }
        }
    }
}
