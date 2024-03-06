use crate::{
    color::NEW,
    ui::{render_line, Surround},
};
use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{Block, BorderType, Borders},
};
use rustdoc_types::ItemEnum;
use term_rustdoc::tree::{CrateDoc, ItemInnerKind, ID};

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

impl Selected {
    fn inner_item(&self, doc: &CrateDoc) -> Option<ItemInnerKind> {
        doc.dmodule().get_item_inner(&self.id)
    }
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
    pub fn set_item_inner(&mut self, id: Option<&str>) -> Option<ItemInnerKind> {
        self.inner_area = self.border.inner();
        if let Some(doc) = &self.doc {
            self.selected = id.and_then(|id| {
                Kind::new(id, doc).map(|kind| Selected {
                    id: id.into(),
                    kind,
                })
            });
            if let Some(selected) = &self.selected {
                *self.border.block_mut() = block();
                return selected.inner_item(doc);
            }
            *self.border.block_mut() = Default::default();
        }
        None
    }

    fn kind(&self) -> Option<Kind> {
        self.selected.as_ref().map(|v| v.kind)
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.border.render(buf);

        let width = self.inner_area.width as usize;
        let Rect { x, y, .. } = self.inner_area;
        match self.kind() {
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
