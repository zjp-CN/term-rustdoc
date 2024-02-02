use self::scrollable::{ScrollText, ScrollTreeLines, Scrollable};
use crate::{
    app::{App, CrateDoc},
    Result,
};
use ratatui::{
    prelude::{Buffer, Constraint, Direction, Frame, Layout, Rect, Widget},
    widgets::{Block, Borders},
};
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
                ..Default::default()
            },
            content: Content {
                display: ScrollText::new_text(doc)?,
                ..Default::default()
            },
            ..Default::default()
        };
        info!(?page);
        page.update_content();
        Ok(page)
    }

    /// This is called in Widget's render method because inner widgets don't implement
    /// Widget, since the areas they draw are updated only from here, not from Widget trait.
    fn update_area(&mut self, full: Rect) {
        // layout
        let outline_width = self.outline.display.max_windth;
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(outline_width), Constraint::Min(0)])
            .split(full);

        // border
        self.outline.border = Surround {
            block: Block::new(),
            area: layout[0],
        };
        let outline_area = self.outline.border.inner();
        self.content.border = Surround {
            block: Block::new().borders(Borders::LEFT),
            area: layout[1],
        };
        let content_area = self.content.border.inner();

        // display.area
        self.outline.display.area = outline_area;
        // self.outline.display.cursor = outline_area.y;
        let outline_max_width = self.outline.display.max_windth;
        if outline_area.width < outline_max_width {
            warn!(
                outline_area.width,
                outline_max_width,
                "Outline width exceeds the area width, so lines may be truncated."
            );
        }

        self.content.display.area = content_area;
        // self.content.display.cursor = content_area.y;
    }
}

impl Widget for &mut Page {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.update_area(area);
        self.outline.border.render(buf);
        self.content.border.render(buf);
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
struct Surround {
    block: Block<'static>,
    area: Rect,
}

impl Surround {
    fn inner(&self) -> Rect {
        self.block.inner(self.area)
    }

    fn render(&self, buf: &mut Buffer) {
        self.block.clone().render(self.area, buf);
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
    display: ScrollTreeLines,
    border: Surround,
}
