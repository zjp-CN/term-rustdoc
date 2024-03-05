#![allow(dead_code)]
use crate::ui::{scrollable::ScrollHeading, Surround};
use ratatui::layout::Position;
use term_rustdoc::tree::ItemInnerKind;

#[derive(Default, Debug)]
pub struct Navigation {
    display: Navi,
    border: Surround,
}

impl Navigation {
    pub fn heading(&mut self) -> &mut ScrollHeading {
        &mut self.display.heading
    }

    // position in (x, y)
    pub fn contains(&self, position: Position) -> bool {
        self.border.area().contains(position)
    }

    pub fn border(&mut self) -> &mut Surround {
        &mut self.border
    }

    pub fn set_item_inner(&mut self, item_inner: ItemInnerKind) {
        self.display.outline.item_inner = Some(item_inner);
    }
}

#[derive(Default)]
struct Navi {
    heading: ScrollHeading,
    outline: NaviOutline,
}

impl std::fmt::Debug for Navi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Navi {{ ... }}")
    }
}

#[derive(Default)]
struct NaviOutline {
    /// Fields/Variants and impls.
    item_inner: Option<ItemInnerKind>,
}
