use self::{
    panel::Panel,
    scrollable::{ScrollHeading, ScrollText, ScrollTreeLines},
};
use crate::Result;
use ratatui::{
    prelude::{Buffer, Rect, Widget},
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
pub use scrollable::{render_line, render_line_fill_gap, LineState, Scrollable};

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
pub struct Surround {
    block: Block<'static>,
    area: Rect,
}

impl Surround {
    pub fn new(block: Block<'static>, area: Rect) -> Self {
        Surround { block, area }
    }

    // pub fn block(&mut self) -> &mut Block<'static> {
    //     &mut self.block
    // }
    //
    // pub fn area(&mut self) -> &mut Rect {
    //     &mut self.area
    // }

    pub fn inner(&self) -> Rect {
        self.block.inner(self.area)
    }

    pub fn render(&self, buf: &mut Buffer) {
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
