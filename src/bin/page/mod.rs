use self::{
    navi::{NaviAction, Navigation},
    panel::Panel,
};
use crate::{
    database::PkgKey,
    ui::{
        scrollable::{ScrollText, ScrollTreeLines},
        Surround,
    },
    Result,
};
use ratatui::prelude::{Buffer, Rect, Widget};
use term_rustdoc::tree::{CrateDoc, ID};

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
                display: ScrollText::new_text(doc.clone())?,
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
        self.content.display.render(buf);
        self.navi.render(buf, &self.content.display);
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

    fn set_inner_item_id(&mut self, id: ID) {
        self.inner.set_inner_item_id(id);
    }
}

#[derive(Default, Debug)]
struct Content {
    display: ScrollText,
    border: Surround,
}
