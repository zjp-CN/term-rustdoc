use crate::{
    color::NEW,
    ui::{render_line, Surround},
};
use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{Block, BorderType, Borders},
};
use term_rustdoc::tree::ItemInnerKind;

#[derive(Default)]
pub struct NaviOutline {
    /// Fields/Variants and impls.
    pub item_inner: Option<ItemInnerKind>,
    pub inner_area: Rect,
    pub border: Surround,
}

impl NaviOutline {
    pub fn set_item_inner(&mut self, item_inner: Option<ItemInnerKind>) {
        *self.border.block_mut() = if item_inner.is_some() {
            block()
        } else {
            Default::default()
        };
        self.inner_area = self.border.inner();
        self.item_inner = item_inner;
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.border.render(buf);

        let width = self.inner_area.width as usize;
        let Rect { x, y, .. } = self.inner_area;
        match &self.item_inner {
            Some(ItemInnerKind::Struct(_) | ItemInnerKind::Union(_)) => {
                render_line(Some(("👉 Fields", NEW)), buf, x, y, width);
            }
            Some(ItemInnerKind::Enum(_)) => {
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
