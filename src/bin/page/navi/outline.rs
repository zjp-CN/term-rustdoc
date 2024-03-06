use crate::{
    color::NEW,
    ui::{render_line, Surround},
};
use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{Block, BorderType, Borders},
};
use rustdoc_types::ItemEnum;
use term_rustdoc::tree::{CrateDoc, ID};

#[derive(Default)]
pub struct NaviOutline {
    pub doc: Option<CrateDoc>,
    /// Selected item that has inner data of a kind like fields/variants/impls.
    pub selected: Option<Selected>,
    pub inner_area: Rect,
    pub border: Surround,
}

pub struct Selected {
    id: ID,
    kind: Kind,
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Struct,
    Enum,
    Trait,
    Union,
}

impl Kind {
    fn new(id: &str, doc: &CrateDoc) -> Option<Kind> {
        doc.get_item(id).and_then(|item| {
            Some(match &item.inner {
                ItemEnum::Struct(_) => Kind::Struct,
                ItemEnum::Enum(_) => Kind::Enum,
                ItemEnum::Trait(_) => Kind::Trait,
                ItemEnum::Union(_) => Kind::Union,
                _ => return None,
            })
        })
    }
}

impl NaviOutline {
    pub fn set_item_inner(&mut self, id: Option<&str>) {
        self.inner_area = self.border.inner();
        self.selected = id.zip(self.doc.as_ref()).and_then(|(id, doc)| {
            Kind::new(id, doc).map(|kind| Selected {
                id: id.into(),
                kind,
            })
        });
        *self.border.block_mut() = if self.selected.is_some() {
            block()
        } else {
            Default::default()
        };
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.border.render(buf);

        let width = self.inner_area.width as usize;
        let Rect { x, y, .. } = self.inner_area;
        match self.selected.as_ref().map(|v| v.kind) {
            Some(Kind::Struct | Kind::Union) => {
                render_line(Some(("👉 Fields", NEW)), buf, x, y, width);
            }
            Some(Kind::Enum) => {
                render_line(Some(("👉 Variants", NEW)), buf, x, y, width);
            }
            _ => (),
        }
    }
}

fn block() -> Block<'static> {
    Block::new()
        .borders(Borders::TOP)
        .border_type(BorderType::Thick)
}
