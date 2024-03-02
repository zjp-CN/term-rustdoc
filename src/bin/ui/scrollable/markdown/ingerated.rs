use super::{fallback::StyledLine, parse::Blocks, render::write_lines, ScrollHeading};
use crate::ui::{scrollable::markdown::parse::parse_doc, Scroll};
use ratatui::prelude::{Buffer, Constraint, Layout, Rect};

pub struct MarkdownAndHeading {
    md: MarkdownArea,
    heading: ScrollHeading,
}

pub type ScrollMarkdown = Scroll<MarkdownInner>;

const TOC_WIDTH: u16 = 12;

impl MarkdownAndHeading {
    pub fn new(mut md: &str, area: Rect) -> Self {
        let width = area.width.saturating_sub(1);
        if width < TOC_WIDTH {
            md = "too narrow to show anything";
        }
        let (lines, blocks, headings) = parse_doc(md, (width - TOC_WIDTH) as f64);
        let mut heading = ScrollHeading::default();
        heading.update_headings(headings);
        let [md, head] = split_area(area);
        heading.area = head;
        MarkdownAndHeading {
            md: MarkdownArea::new(md, lines, blocks),
            heading,
        }
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.md.render(buf);
        let content_start = self.md.inner.start;
        let content_end = self.md.inner.area.height as usize + content_start;
        self.heading.render(buf, content_start, content_end);
    }

    pub fn scroll_text(&mut self) -> &mut ScrollMarkdown {
        &mut self.md.inner
    }

    pub fn heading(&mut self) -> &mut ScrollHeading {
        &mut self.heading
    }

    /// y is the row in full screen
    pub fn heading_jump(&mut self, y: u16) -> bool {
        const MARGIN: usize = 1;
        if let Some(heading) = self.heading.get_line_on_screen(y) {
            // set the upper bound: usually no need to use this, but who knows if y points
            // to a line out of the doc range.
            let limit = self.md.inner.total_len().saturating_sub(MARGIN);
            let old = self.md.inner.start;
            self.md.inner.start = heading.jump_row_start().saturating_sub(MARGIN).min(limit);
            let new = self.md.inner.start;
            info!(old, new);
            return true;
        }
        false
    }
}

fn split_area(area: Rect) -> [Rect; 2] {
    // in case heading is too wide
    Layout::horizontal([Constraint::Min(0), Constraint::Length(TOC_WIDTH)]).areas(area)
}

pub struct MarkdownArea {
    inner: Scroll<MarkdownInner>,
    #[allow(dead_code)]
    blocks: Blocks,
}

impl MarkdownArea {
    fn new(area: Rect, lines: Vec<StyledLine>, blocks: Blocks) -> Self {
        let md = Scroll::<MarkdownInner> {
            area,
            lines: MarkdownInner { lines },
            ..Default::default()
        };
        MarkdownArea { inner: md, blocks }
    }

    fn render(&self, buf: &mut Buffer) {
        write_lines(&self.inner.lines, self.inner.start, self.inner.area, buf);
    }
}

#[derive(Default)]
pub struct MarkdownInner {
    lines: Vec<StyledLine>,
}

impl std::ops::Deref for MarkdownInner {
    type Target = [StyledLine];

    fn deref(&self) -> &Self::Target {
        &self.lines
    }
}
