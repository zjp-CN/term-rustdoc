use crate::ui::scrollable::MarkdownArea;
use ratatui::prelude::*;
use term_rustdoc::{decl::item_str, tree::IDMap};

#[derive(Default)]
pub struct Declaration {
    display: MarkdownArea,
}

impl std::fmt::Debug for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Declaration")
    }
}

impl Declaration {
    pub fn new(id: &str, map: &IDMap, width: u16) -> Self {
        let code = item_str(id, map);
        Declaration {
            display: MarkdownArea::rust_code(&code, width),
        }
    }

    /// Reserve space for item and returns the rest area for showing markdown content.
    pub fn update_area(&mut self, outer: Rect) -> Rect {
        let scroll_text = self.display.scroll_text();
        let height = scroll_text.total_len() as u16;
        let [decl, md] =
            Layout::vertical([Constraint::Length(height), Constraint::Min(0)]).areas(outer);
        scroll_text.area = decl;
        md
    }
}
