use super::region::SelectedRegion;
use crate::ui::{
    scrollable::{
        generics::{render_line, LineState},
        Scrollable,
    },
    Page,
};
use ratatui::buffer::Buffer;
use term_rustdoc::{tree::Text, util::XString};

pub type ScrollHeading = Scrollable<Headings>;

#[derive(Debug)]
pub struct Heading {
    line: Text,
    jump: SelectedRegion,
}

impl Heading {
    fn new(text: XString, jump: SelectedRegion) -> Self {
        Self {
            line: Text {
                text,
                style: Default::default(),
            },
            jump,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.line.text
    }
}

impl LineState for Heading {
    type State = XString;

    fn state(&self) -> Self::State {
        self.line.text.clone()
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        self.as_str() == *state
    }
}

#[derive(Default)]
pub struct Headings {
    lines: Vec<Heading>,
}

impl std::ops::Deref for Headings {
    type Target = [Heading];

    fn deref(&self) -> &Self::Target {
        &self.lines
    }
}

impl Headings {
    pub fn with_capacity(cap: usize) -> Self {
        Headings {
            lines: Vec::with_capacity(cap),
        }
    }

    pub fn push(&mut self, text: XString, region: SelectedRegion) {
        self.lines.push(Heading::new(text, region));
    }
}

impl ScrollHeading {
    pub fn update_headings(&mut self, headings: Headings) {
        // NOTE: max_width is the real maximum width of all lines,
        // and the display width can be smaller then max_width to truncate heading lines.
        let max_windth = self
            .lines
            .iter()
            .map(|h| h.as_str().len() as _)
            .max()
            .unwrap_or(0);
        self.lines = headings;
        self.max_width = max_windth;
    }

    pub fn render(&self, buf: &mut Buffer) {
        let width = self.area.width;
        if width == 0 {
            return;
        }
        let (x, mut y, width) = (self.area.x, self.area.y, width as usize);
        for line in &self.lines.lines {
            let text = &line.as_str();
            let text = text.get(..width.min(text.len())).unwrap_or("");
            let style = line.line.style;
            render_line(Some((text, style)), buf, x, y, width);
            y += 1;
        }
    }
}

impl Page {
    pub fn heading_jump(&mut self, y: u16) -> bool {
        const MARGIN: usize = 3;
        if let Some(heading) = self.navi.display.lines.get(y as usize) {
            // set the upper bound: usually no need to use this, but who knows if y points
            // to a line out of the doc range.
            let limit = self.content.display.total_len().saturating_sub(MARGIN);
            self.content().start = heading.jump.row_start().saturating_sub(MARGIN).min(limit);
            return true;
        }
        false
    }
}
