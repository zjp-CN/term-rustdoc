use self::{
    navi::{NaviAction, Navigation},
    outline::OutlineKind,
    panel::Panel,
};
use crate::{
    database::PkgKey,
    ui::{
        scrollable::{Scroll, ScrollText, ScrollTreeLines},
        Surround,
    },
    Result,
};
use ratatui::prelude::{Buffer, Rect, Widget};
use term_rustdoc::tree::CrateDoc;

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
            outline: Outline {
                display: Scroll::new(doc.clone().into())?,
                ..Default::default()
            },
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
    display: ScrollTreeLines,
    inner_item: outline::InnerItem,
    render: OutlineKind,
    border: Surround,
}

impl Outline {
    fn render(&self, buf: &mut Buffer) {
        self.border.render(buf);
        match self.render {
            OutlineKind::Modules => self.display.render(buf),
            OutlineKind::InnerItem => {
                let doc = self.display.lines.doc_ref();
                self.inner_item.render(buf, doc);
            }
        };
    }

    fn action(&mut self, action: NaviAction) {
        match action {
            NaviAction::BackToHome => self.back_to_home(),
            _ => {
                self.inner_item.update_lines(&self.display);
                self.render = OutlineKind::InnerItem;
            }
        };
    }

    fn back_to_home(&mut self) {
        self.render = OutlineKind::Modules;
    }
}

#[derive(Default, Debug)]
struct Content {
    display: ScrollText,
    border: Surround,
}
