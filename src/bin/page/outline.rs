use crate::ui::scrollable::ScrollTreeLines;
use ratatui::prelude::{Buffer, Rect};
use term_rustdoc::tree::{CrateDoc, TreeLines, ID};

#[derive(Default, Debug, Clone, Copy)]
pub enum OutlineKind {
    #[default]
    Modules,
    InnerItem,
}

#[derive(Default)]
pub struct InnerItem {
    outer_item: ID,
    display: ScrollTreeLines,
}

impl std::fmt::Debug for InnerItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InnerItem")
    }
}

impl InnerItem {
    pub fn new(outer_item: ID) -> Self {
        Self {
            outer_item,
            ..Default::default()
        }
    }

    pub fn update_area(&mut self, area: Rect) {
        self.display.area = area;
    }

    pub fn update_lines(&mut self, doc: CrateDoc) {
        self.display.lines = TreeLines::new_with(doc, |doc| {
            doc.dmodule()
                .item_inner_tree(&self.outer_item, doc)
                .unwrap()
        })
        .0;
    }

    pub fn render(&self, buf: &mut Buffer, _doc: &CrateDoc) {
        if self.display.lines.is_empty() {
            return;
        }
        self.display.render(buf);
    }
}
