use ratatui::prelude::Buffer;
use term_rustdoc::tree::{CrateDoc, ID};

#[derive(Default, Debug, Clone, Copy)]
pub enum OutlineKind {
    #[default]
    Modules,
    InnerItem,
}

#[derive(Default)]
pub struct InnerItem {
    outer_item: ID,
}

impl std::fmt::Debug for InnerItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InnerItem")
    }
}

impl InnerItem {
    pub fn new(outer_item: ID) -> Self {
        Self { outer_item }
    }

    pub fn render(&self, buf: &mut Buffer, doc: &CrateDoc) {}
}
