use super::navi::NaviAction;
use crate::ui::scrollable::ScrollTreeLines;
use ratatui::prelude::{Buffer, Rect};
use term_rustdoc::tree::{CrateDoc, TreeLines, ID};

#[derive(Default)]
pub struct OutlineInner {
    kind: OutlineKind,
    modules: ScrollTreeLines,
    inner_item: InnerItem,
}

impl std::fmt::Debug for OutlineInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutlineInner")
            .field("kind", &self.kind)
            .finish()
    }
}

impl OutlineInner {
    pub fn new(doc: &CrateDoc) -> Self {
        let modules = match ScrollTreeLines::new_tree_lines(doc.clone().into()) {
            Ok(lines) => lines,
            Err(err) => {
                error!("Failed to init module Outline:\n{err}");
                return OutlineInner::default();
            }
        };
        OutlineInner {
            modules,
            ..Default::default()
        }
    }

    // pub fn kind(&self) -> OutlineKind {
    //     self.kind
    // }

    pub fn display(&mut self) -> &mut ScrollTreeLines {
        match self.kind {
            OutlineKind::Modules => &mut self.modules,
            OutlineKind::InnerItem => &mut self.inner_item.display,
        }
    }

    pub fn display_ref(&self) -> &ScrollTreeLines {
        match self.kind {
            OutlineKind::Modules => &self.modules,
            OutlineKind::InnerItem => &self.inner_item.display,
        }
    }

    pub fn update_area(&mut self, area: Rect) {
        self.modules.area = area;
        self.inner_item.update_area(area);
    }

    pub fn render(&self, buf: &mut Buffer) {
        match self.kind {
            OutlineKind::Modules => self.modules.render(buf),
            OutlineKind::InnerItem => {
                let doc = self.inner_item.display.lines.doc_ref();
                self.inner_item.render(buf, doc);
            }
        };
    }
}

/// Action from Navi
impl OutlineInner {
    pub fn set_inner_item_id(&mut self, id: ID) {
        self.inner_item.outer_item = id;
    }

    pub fn action(&mut self, action: NaviAction) {
        match action {
            NaviAction::BackToHome => self.back_to_home(),
            x => {
                self.inner_item.update_lines(&self.modules, x);
                self.kind = OutlineKind::InnerItem;
            }
        };
    }

    fn back_to_home(&mut self) {
        self.kind = OutlineKind::Modules;
    }
}

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

impl InnerItem {
    pub fn update_area(&mut self, area: Rect) {
        self.display.area = area;
    }

    pub fn update_lines(&mut self, modules: &ScrollTreeLines, action: NaviAction) {
        let doc = modules.lines.doc();
        self.display.lines = TreeLines::new_with(doc, |doc| {
            let id = &self.outer_item;
            let dmod = doc.dmodule();
            match action {
                NaviAction::ITABImpls => dmod.impl_tree(id, doc),
                _ => dmod.item_inner_tree(id, doc),
            }
            .unwrap_or_default()
        })
        .0;
        if self.display.total_len() == 0 {
            let path = modules.lines.doc_ref().path(&self.outer_item);
            error!("{path} generated unexpected empty TreeLines");
        }
        self.display.update_maxwidth();
    }

    pub fn render(&self, buf: &mut Buffer, _doc: &CrateDoc) {
        if self.display.lines.is_empty() {
            return;
        }
        self.display.render(buf);
    }
}
