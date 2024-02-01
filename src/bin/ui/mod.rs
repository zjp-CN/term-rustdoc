use self::scrollable::{ScrollText, ScrollTreeLines, Scrollable};
use crate::{app::App, Result};
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
    pub fn new(outline: TreeLines, full: Rect) -> Result<Self> {
        let mut outline = Outline {
            display: Scrollable::new(outline, full)?,
        };
        let outline_width = outline.display.max_windth;

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(outline_width), Constraint::Min(0)])
            .split(full);
        outline.display.area = layout[0];

        let page = Page {
            outline,
            ..Default::default()
        };
        info!(?page);
        Ok(page)
    }
}

impl Widget for &mut Page {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.outline.display.render(area, buf);
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
