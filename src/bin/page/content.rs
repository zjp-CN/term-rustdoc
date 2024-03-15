use crate::{
    color::DECLARATION_BORDER,
    ui::{
        scrollable::{Headings, MarkdownArea, ScrollText},
        Surround,
    },
};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders},
};
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
            // exclude border width
            let width = outer.width.saturating_sub(4);
            self.decl.update_decl(id, map, width);
        }
        let md = self.decl.update_area(outer);
        self.md.area = md;
        self.md.start = 0;
        // self.md.max_width = md.width;
    }

    pub fn render(&self, buf: &mut Buffer) {
        if !self.decl.display.scroll_text_ref().is_empty() {
            self.decl.border.render(buf);
            self.decl.display.render(buf);
        }
        self.md.render(buf);
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

struct Declaration {
    display: MarkdownArea,
    border: Surround,
}

impl Default for Declaration {
    fn default() -> Self {
        Declaration {
            display: MarkdownArea::default(),
            border: Surround::new(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::QuadrantOutside)
                    .border_style(DECLARATION_BORDER),
                Rect::default(),
            ),
        }
    }
}

impl Declaration {
    fn update_decl(&mut self, id: &str, map: &IDMap, width: u16) {
        let code = item_str(id, map);
        if code.is_empty() {
            self.display.scroll_text().lines = Default::default();
        } else {
            self.display.rust_code(&code, width);
        }
    }

    /// Reserve space for item and returns the rest area for showing markdown content.
    fn update_area(&mut self, outer: Rect) -> Rect {
        let scroll_text = self.display.scroll_text();
        let total_len = scroll_text.total_len() as u16;
        if total_len == 0 {
            return outer;
        }
        let height = total_len + 1;
        let [decl, md] =
            Layout::vertical([Constraint::Length(height), Constraint::Min(0)]).areas(outer);
        if let Some(inner) = self.border.update_area(decl) {
            scroll_text.area = inner;
            // scroll_text.max_width = decl.width;
        }
        md
    }
}
