use super::{fallback::StyledLine, parse::Blocks, render::write_lines, ScrollHeading};
use crate::ui::{scrollable::markdown::parse::parse_doc, Scrollable};
use ratatui::prelude::{Buffer, Constraint, Layout, Rect};

pub struct MarkdownAndHeading {
    md: MarkdownArea,
    heading: ScrollHeading,
}

pub type ScrollMarkdown = Scrollable<MarkdownInner>;

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

    /// row is for full screen
    pub fn heading_jump(&mut self, row: u16) -> bool {
        const MARGIN: usize = 1;
        let y = row.saturating_sub(self.heading.area.y);
        if let Some(heading) = self.heading.lines.get(y as usize) {
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
    inner: Scrollable<MarkdownInner>,
    #[allow(dead_code)]
    blocks: Blocks,
}

impl MarkdownArea {
    fn new(area: Rect, lines: Vec<StyledLine>, blocks: Blocks) -> Self {
        let md = Scrollable::<MarkdownInner> {
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
