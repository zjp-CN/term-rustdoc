use self::{
    panel::Panel,
    scrollable::{ScrollHeading, ScrollText, ScrollTreeLines, Scrollable},
};
use crate::{app::App, Result};
use ratatui::{
    prelude::{Buffer, Frame, Rect, Widget},
    widgets::Block,
};
use term_rustdoc::tree::{CrateDoc, TreeLines};

mod layout;
mod panel;

/// fold/expand a tree view
mod page_fold;
/// scroll up/down behavior and with what offset
mod page_scroll;
/// Scrollable widget
mod scrollable;

pub use page_scroll::ScrollOffset;

pub fn render(_app: &mut App, page: &mut Page, f: &mut Frame) {
    f.render_widget(page, f.size());
}

#[derive(Default, Debug)]
pub struct Page {
    outline: Outline,
    content: Content,
    navi: Navigation,
    current: Option<Panel>,
    area: Rect,
}

impl Page {
    pub fn new(outline: TreeLines, doc: Option<CrateDoc>, area: Rect) -> Result<Self> {
        let mut page = Page {
            outline: Outline {
                display: Scrollable::new(outline)?,
                ..Default::default()
            },
            content: Content {
                display: ScrollText::new_text(doc)?,
                ..Default::default()
            },
            // page scrolling like HOME/END will check the current Panel
            current: Some(Panel::Outline),
            area,
            ..Default::default()
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
}

impl Widget for &mut Page {
    fn render(self, area: Rect, buf: &mut Buffer) {
        info!("Page rendering starts");
        self.update_area(area);
        self.outline.border.render(buf);
        self.content.border.render(buf);
        self.outline.display.render(buf);
        self.content.display.render(buf);
        self.navi.border.render(buf);
        let content_start = self.content().start;
        let content_end = self.content().area.height as usize + content_start;
        self.navi.display.render(buf, content_start, content_end);
        info!("Page rendered");
    }
}

#[derive(Default, Debug)]
struct Surround {
    block: Block<'static>,
    area: Rect,
}

impl Surround {
    fn inner(&self) -> Rect {
        self.block.inner(self.area)
    }

    fn render(&self, buf: &mut Buffer) {
        (&self.block).render(self.area, buf);
    }
}

#[derive(Default, Debug)]
struct Outline {
    display: ScrollTreeLines,
    border: Surround,
}

#[derive(Default, Debug)]
struct Content {
    display: ScrollText,
    border: Surround,
}

#[derive(Default, Debug)]
struct Navigation {
    display: ScrollHeading,
    border: Surround,
}
