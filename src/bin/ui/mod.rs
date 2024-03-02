use self::{
    panel::Panel,
    scrollable::{ScrollText, ScrollTreeLines},
};
use crate::{database::PkgKey, Result};
use ratatui::{
    prelude::{Buffer, Rect, Style, Widget},
    widgets::Block,
};
use term_rustdoc::tree::CrateDoc;
use unicode_width::UnicodeWidthStr;

mod layout;
mod panel;

/// fold/expand a tree view
mod page_fold;
/// scroll up/down behavior and with what offset
mod page_scroll;
/// Scrollable widget
mod scrollable;

pub use page_scroll::ScrollOffset;
pub use scrollable::{
    render_line, LineState, MarkdownAndHeading, Scroll, ScrollHeading, ScrollMarkdown, Scrollable,
};

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
                display: ScrollText::new_text(doc)?,
                ..Default::default()
            },
            // page scrolling like HOME/END will check the current Panel
            current: Some(Panel::Outline),
            area,
            pkg_key: Some(pkg_key),
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
        self.outline.border.render(buf);
        self.content.border.render(buf);
        self.outline.display.render(buf);
        self.content.display.render(buf);
        self.navi.border.render(buf);
        let content_start = self.content().start;
        let content_end = self.content().area.height as usize + content_start;
        self.navi.display.render(buf, content_start, content_end);
        debug!("Page rendered");
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

    pub fn inner(&self) -> Rect {
        self.block.inner(self.area)
    }

    pub fn render(&self, buf: &mut Buffer) {
        (&self.block).render(self.area, buf);
    }

    pub fn area(&self) -> Rect {
        self.area
    }

    /// Update the border area and then return inner area only when the outer areas differ.
    pub fn update_area(&mut self, area: Rect) -> Option<Rect> {
        if self.area == area {
            return None;
        }
        self.area = area;
        Some(self.inner())
    }

    pub fn render_only_bottom_right_text(&self, buf: &mut Buffer, text: &str) -> usize {
        let area = self.area;
        let text_width = text.width();
        if let Some(offset) = (area.width as usize).checked_sub(2 + text_width) {
            let x = area.x + offset as u16;
            let y = area.y + area.height - 1;
            render_line(Some((text, Style::new())), buf, x, y, text_width);
            return text_width + 2;
        }
        0
    }

    pub fn render_only_bottom_left_text(&self, buf: &mut Buffer, text: &str, used: usize) {
        let area = self.area;
        if let Some(rest) = (area.width as usize).checked_sub(2 + used) {
            if rest < text.width() {
                // not enought space to show
                return;
            }
            let x = area.x + 2;
            let y = area.y + area.height - 1;
            render_line(Some((text, Style::new())), buf, x, y, rest);
        }
    }

    pub fn render_only_top_left_text(&self, buf: &mut Buffer, text: &str, used: usize) {
        let area = self.area;
        if let Some(rest) = (area.width as usize).checked_sub(2 + used) {
            if rest < text.width() {
                // not enought space to show
                return;
            }
            let x = area.x + 2;
            let y = area.y;
            render_line(Some((text, Style::new())), buf, x, y, rest);
        }
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
