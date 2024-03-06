use ratatui::prelude::Buffer;
use term_rustdoc::tree::{CrateDoc, ID};

pub struct InnerItem {
    outer_item: ID,
    show: bool,
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
            show: false,
        }
    }

    pub fn render(&self, buf: &mut Buffer, doc: &CrateDoc) -> bool {
        if !self.show {
            return false;
        }
        true
    }
}
