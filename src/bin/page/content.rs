use crate::ui::scrollable::{Headings, MarkdownArea, ScrollText};
use ratatui::prelude::*;
use term_rustdoc::{
    decl::item_str,
    tree::{CrateDoc, IDMap},
};

#[derive(Default)]
pub(super) struct ContentInner {
    decl: Declaration,
    md: ScrollText,
}

impl std::fmt::Debug for ContentInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContentInner")
    }
}

impl ContentInner {
    pub fn new(doc: &CrateDoc) -> Self {
        let md = ScrollText::new_text(doc.clone()).unwrap_or_default();
        ContentInner {
            md,
            decl: Declaration::default(),
        }
    }

    pub fn update_area(&mut self, id: &str, outer: Rect) {
        if let Some(map) = self.md.doc_ref() {
            self.decl.update(id, map);
        }
        let md = self.decl.update_area(outer);
        self.md.area = md;
        self.md.start = 0;
        // self.md.max_width = md.width;
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.md.render(buf);
        self.decl.display.render(buf);
    }

    pub fn content(&mut self) -> &mut ScrollText {
        &mut self.md
    }

    pub fn md_ref(&self) -> &ScrollText {
        &self.md
    }

    pub fn update_doc(&mut self, id: &str, outer: Rect) -> Option<Headings> {
        self.update_area(id, outer);
        self.md.update_doc(id)
    }

    pub fn reset_doc(&mut self) {
        self.md.lines.reset_doc();
    }
}

#[derive(Default)]
struct Declaration {
    display: MarkdownArea,
}

impl Declaration {
    fn update(&mut self, id: &str, map: &IDMap) {
        let code = item_str(id, map);
        info!(code);
        self.display.rust_code(&code);
    }

    /// Reserve space for item and returns the rest area for showing markdown content.
    fn update_area(&mut self, outer: Rect) -> Rect {
        let scroll_text = self.display.scroll_text();
        let height = scroll_text.total_len() as u16 + 1;
        let [decl, md] =
            Layout::vertical([Constraint::Length(height), Constraint::Min(0)]).areas(outer);
        scroll_text.area = decl;
        // scroll_text.max_width = decl.width;
        md
    }
}
