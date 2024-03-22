use crate::{
    color::{DECLARATION_BORDER, JUMP, NEW},
    ui::{
        render_line,
        scrollable::{Headings, ScrollText},
        LineState, Scroll, Surround,
    },
};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders},
};
use term_rustdoc::{
    tree::{CrateDoc, IDMap},
    type_name::{
        render::{DeclarationLine, DeclarationLines},
        style::item_styled,
    },
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

    pub fn update_decl(&mut self, id: &str, outer: Rect) {
        if let Some(map) = self.md.doc_ref() {
            // exclude border width
            let width = outer.width.saturating_sub(4);
            self.decl.update_decl(id, map, width);
        }
        self.update_area(outer);
    }

    pub fn update_area(&mut self, outer: Rect) {
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
        self.update_decl(id, outer);
        self.md.update_doc(id)
    }

    pub fn reset_doc(&mut self) {
        self.md.lines.reset_doc();
    }
}

struct Declaration {
    display: DeclarationInner,
    border: Surround,
}

impl Default for Declaration {
    fn default() -> Self {
        Declaration {
            display: Default::default(),
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

#[derive(Default)]
struct DeclarationInner {
    inner: Scroll<DeclarationLines>,
}

impl DeclarationInner {
    fn scroll_text(&mut self) -> &mut Scroll<DeclarationLines> {
        &mut self.inner
    }

    fn scroll_text_ref(&self) -> &Scroll<DeclarationLines> {
        &self.inner
    }

    fn render(&self, buf: &mut Buffer) {
        let Rect {
            x, mut y, width, ..
        } = self.inner.area;
        let width = width as usize;
        for line in self.inner.all_lines() {
            let line = line
                .iter()
                .map(|tt| (tt.text.as_str(), if tt.id.is_some() { JUMP } else { NEW }));
            render_line(line, buf, x, y, width);
            y += 1;
        }
    }

    fn update_decl(&mut self, lines: DeclarationLines) {
        self.inner.lines = lines;
    }
}

/// No need to query state for previous line.
impl LineState for DeclarationLine {
    type State = ();
    fn state(&self) -> Self::State {}
    fn is_identical(&self, _: &Self::State) -> bool {
        false
    }
}

impl Declaration {
    fn update_decl(&mut self, id: &str, map: &IDMap, width: u16) {
        let lines = item_styled(id, map).to_declaration_lines();
        if lines.is_empty() {
            self.display.scroll_text().lines = Default::default();
        } else {
            self.display.update_decl(lines);
            // self.display.rust_code(&code, width);
        }
    }

    /// Reserve space for item and returns the rest area for showing markdown content.
    fn update_area(&mut self, outer: Rect) -> Rect {
        let scroll_text = self.display.scroll_text();
        let total_len = scroll_text.total_len() as u16;
        if total_len == 0 {
            return outer;
        }
        let height = total_len + 2;
        let [decl, md] = Layout::vertical([Constraint::Length(height), Constraint::Min(0)])
            .spacing(1)
            .areas(outer);
        if let Some(inner) = self.border.update_area(decl) {
            scroll_text.area = inner;
            // scroll_text.max_width = decl.width;
        }
        md
    }
}
