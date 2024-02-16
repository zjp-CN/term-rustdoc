use self::panel::{Panel, SET};
use self::scrollable::{ScrollHeading, ScrollText, ScrollTreeLines, Scrollable};
use crate::{app::App, Result};
use ratatui::{
    prelude::{Buffer, Constraint, Direction, Frame, Layout, Rect, Widget},
    widgets::{Block, BorderType, Borders},
};
use term_rustdoc::tree::{CrateDoc, TreeLines};

/// fold/expand a tree view
mod page_fold;
/// scroll up/down behavior and with what offset
mod page_scroll;
mod panel;
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
        Ok(page)
    }

    #[allow(clippy::single_match)]
    pub fn double_click(&mut self) {
        match self.current {
            Some(Panel::Outline) => self.outline_fold_expand_toggle(),
            _ => {}
        }
    }

    fn layout(&self) -> Layout {
        let outline_width = self.outline.display.max_width + 1;
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(outline_width),
                Constraint::Min(0),
                Constraint::Percentage(10),
            ])
    }

    /// This is called in Widget's render method because inner widgets don't implement
    /// Widget, since the areas they draw are updated only from here, not from Widget trait.
    fn update_area(&mut self, full: Rect) {
        // skip updating since the size is the same
        if self.area == full {
            return;
        }

        self.update_area_inner(full);
    }

    /// Force update Page inner layout.
    ///
    /// `full` usually should be the full screen area or Page area.
    fn update_area_inner(&mut self, full: Rect) {
        // layout
        self.area = full;
        let layout = self.layout().split(full);

        // border
        let outline_border = Block::new()
            .borders(Borders::RIGHT)
            .border_type(BorderType::Thick);
        self.outline.border = Surround {
            block: if matches!(self.current, None | Some(Panel::Outline)) {
                outline_border.style(SET)
            } else {
                outline_border
            },
            area: layout[0],
        };
        let outline_area = self.outline.border.inner();
        self.content.border = Surround {
            block: Block::new(),
            area: layout[1],
        };
        let content_area = self.content.border.inner();

        // display.area
        self.outline.display.area = outline_area;
        // self.outline.display.cursor = outline_area.y;
        let outline_max_width = self.outline.display.max_width;
        if outline_area.width < outline_max_width {
            warn!(
                outline_area.width,
                outline_max_width,
                "Outline width exceeds the area width, so lines may be truncated."
            );
        }

        self.content.display.area = content_area;
        self.content.display.max_width = content_area.width;

        if let Some(&navi_outer_area) = layout.get(2) {
            self.navi.border = Surround {
                block: Block::new()
                    .borders(Borders::LEFT)
                    .border_type(BorderType::Thick),
                area: navi_outer_area,
            };
            self.navi.display.area = self.navi.border.inner();
        }

        // auto update content when screen size changes
        self.update_content();
    }
}

impl Widget for &mut Page {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.update_area(area);
        self.outline.border.render(buf);
        self.content.border.render(buf);
        self.outline.display.render(buf);
        self.content.display.render(buf);
        self.navi.border.render(buf);
        let content_start = self.content().start;
        let content_end = self.content().area.height as usize + content_start;
        self.navi.display.render(buf, content_start, content_end);
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
