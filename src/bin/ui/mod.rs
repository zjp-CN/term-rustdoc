use self::scrollable::{ScrollText, ScrollTreeLines, Scrollable};
use crate::{
    app::{App, CrateDoc},
    Result,
};
use ratatui::prelude::*;
use term_rustdoc::tree::TreeLines;

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
}

impl Page {
    pub fn new(outline: TreeLines, doc: Option<CrateDoc>) -> Result<Self> {
        let mut page = Page {
            outline: Outline {
                display: Scrollable::new(outline)?,
            },
            content: Content {
                display: ScrollText::new_text(doc)?,
            },
            ..Default::default()
        };
        info!(?page);
        page.update_content();
        Ok(page)
    }

    fn update_area(&mut self, full: Rect) {
        let outline_width = self.outline.display.max_windth;
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(outline_width), Constraint::Min(0)])
            .split(full);
        let (outline_area, content_area) = (layout[0], layout[1]);

        self.outline.display.area = outline_area;
        let outline_max_width = self.outline.display.max_windth;
        if outline_area.width < outline_max_width {
            warn!(
                outline_area.width,
                outline_max_width,
                "Outline width exceeds the area width, so lines may be truncated."
            );
        }

        self.content.display.area = content_area;
    }
}

impl Widget for &mut Page {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.update_area(area);
        self.outline.display.render(buf);
        self.content.display.render(buf);
    }
}

/// The selected texts will be rendered with original fg but grey bg.
#[derive(Default, Debug)]
pub struct Selected {
    row_start: u16,
    row_end: u16,
    col_start: u16,
    col_end: u16,
}

#[derive(Default, Debug)]
struct Outline {
    display: ScrollTreeLines,
}

#[derive(Default, Debug)]
struct Content {
    display: ScrollText,
}

#[derive(Default, Debug)]
struct Navigation {
    display: ScrollTreeLines,
}
