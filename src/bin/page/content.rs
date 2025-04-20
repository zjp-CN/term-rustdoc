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
use rustdoc_types::Id;
use std::ops::Range;
use term_rustdoc::{
    tree::{CrateDoc, IDMap},
    type_name::{DeclarationLine, DeclarationLines},
};
use unicode_width::UnicodeWidthStr;

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

    pub fn update_decl(&mut self, id: &Id, outer: Rect) {
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

    pub fn update_doc(&mut self, id: &Id, outer: Rect) -> Option<Headings> {
        self.update_decl(id, outer);
        self.md.update_doc(id)
    }

    pub fn reset_doc(&mut self) {
        self.md.lines.reset_doc();
    }

    pub fn jumpable_id(&self, x: u16, y: u16) -> Option<Id> {
        self.decl.display.jumpable_id(x, y)
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
    jumpable_ids: Vec<JumpableId>,
}

#[derive(Debug)]
struct JumpableId {
    y: u16,
    x: Range<u16>,
    id: Id,
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
        // self.jumpable_ids = lines.it;
        let mut jumpable_ids = Vec::new();
        let mut col = 0;
        for (y, line) in lines.iter().enumerate() {
            for tt in &**line {
                let width = tt.text.width();
                if let Some(id) = tt.id {
                    let end = col + width;
                    jumpable_ids.push(JumpableId {
                        y: y as u16,
                        x: col as u16..end as u16,
                        id,
                    });
                }
                col += width;
            }
            col = 0;
        }
        info!(?jumpable_ids);
        self.jumpable_ids = jumpable_ids;
        self.inner.lines = lines;
    }

    fn jumpable_id(&self, x: u16, y: u16) -> Option<Id> {
        let area = self.inner.area;
        let x = x.checked_sub(area.x)?;
        let y = y.checked_sub(area.y)?;
        info!(y, x);
        self.jumpable_ids
            .iter()
            .find_map(|jump| (jump.y == y && jump.x.contains(&x)).then_some(jump.id))
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
    fn update_decl(&mut self, id: &Id, map: &IDMap, width: u16) {
        let lines = DeclarationLines::new(id, map);
        if lines.is_empty() {
            self.display.scroll_text().lines = Default::default();
            self.display.jumpable_ids = Vec::new();
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
