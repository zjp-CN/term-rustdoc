use self::{
    navi::{NaviAction, Navigation},
    panel::Panel,
};
use crate::{
    database::PkgKey,
    ui::{scrollable::ScrollTreeLines, Surround},
    Result,
};
use ratatui::prelude::{Buffer, Rect, Widget};
use term_rustdoc::tree::{CrateDoc, ID};

mod content;
mod layout;
mod navi;
mod outline;
/// fold/expand a tree view
mod page_fold;
/// scroll up/down behavior and with what offset
mod page_scroll;
mod panel;

#[derive(Default, Debug)]
pub struct Page {
    outline: Outline,
    content: Content,
    navi: Navigation,
    current: Option<Panel>,
    pkg_key: Option<PkgKey>,
    area: Rect,
}

impl Page {
    pub fn new(pkg_key: PkgKey, doc: CrateDoc, area: Rect) -> Result<Self> {
        let mut page = Page {
            outline: Outline::new(&doc),
            content: Content {
                inner: content::ContentInner::new(&doc),
                ..Default::default()
            },
            // page scrolling like HOME/END will check the current Panel
            current: Some(Panel::Outline),
            area,
            pkg_key: Some(pkg_key),
            navi: Default::default(),
        };
        page.update_area_inner(area);
        info!("Page ready");
        Ok(page)
    }

    #[allow(clippy::single_match)]
    pub fn double_click(&mut self) {
        match self.current {
            Some(Panel::Outline) => self.outline_fold_expand_toggle(),
            _ => {}
        }
    }

    pub fn is_empty(&self) -> bool {
        self.area.height == 0 || self.area.width == 0
    }

    /// Drop the data when PkgKey matches.
    pub fn drop(&mut self, pkg_key: &PkgKey) {
        if self
            .pkg_key
            .as_ref()
            .map(|key| key == pkg_key)
            .unwrap_or(false)
        {
            *self = Page::default();
        }
    }
}

impl Widget for &mut Page {
    fn render(self, area: Rect, buf: &mut Buffer) {
        debug!("Page rendering starts");
        self.update_area(area);
        self.outline.render(buf);
        self.content.border.render(buf);
        self.content.inner.render(buf);
        self.navi.render(buf, self.content.inner.md_ref());
        debug!("Page rendered");
    }
}

#[derive(Default, Debug)]
struct Outline {
    inner: outline::OutlineInner,
    border: Surround,
}

impl Outline {
    fn new(doc: &CrateDoc) -> Self {
        Outline {
            inner: outline::OutlineInner::new(doc),
            ..Default::default()
        }
    }

    fn render(&self, buf: &mut Buffer) {
        self.border.render(buf);
        self.inner.render(buf);
    }

    fn action(&mut self, action: NaviAction) {
        self.inner.action(action);
    }

    fn reset_to_module_tree(&mut self) {
        self.inner.reset_to_module_tree();
    }

    fn display(&mut self) -> &mut ScrollTreeLines {
        self.inner.display()
    }

    fn display_ref(&self) -> &ScrollTreeLines {
        self.inner.display_ref()
    }

    fn update_area(&mut self, border: Surround) {
        self.inner.update_area(border.inner());
        self.border = border;
    }

    fn set_setu_id(&mut self, id: ID) {
        self.inner.set_setu_id(id);
    }

    fn is_module_tree(&self) -> bool {
        self.inner.is_module_tree()
    }
}

#[derive(Default, Debug)]
struct Content {
    inner: content::ContentInner,
    border: Surround,
}

impl Content {
    fn update_area(&mut self, border: Surround, id: Option<&str>) {
        self.border = border;
        let outer = self.border.inner();
        if let Some(id) = id {
            self.inner.update_area(id, outer);
        }
    }

    fn update_doc(&mut self, id: &str) -> Option<crate::ui::scrollable::Headings> {
        self.inner.update_doc(id, self.border.inner())
    }
}
