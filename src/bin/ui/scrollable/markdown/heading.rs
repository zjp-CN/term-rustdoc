use super::region::SelectedRegion;
use crate::{
    color::HEAD,
    ui::scrollable::{
        generics::{render_line, render_line_fill_gap, LineState},
        Scroll,
    },
};
use ratatui::prelude::Buffer;
use term_rustdoc::{tree::Text, util::XString};

pub type ScrollHeading = Scroll<Headings>;

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

    pub fn jump_row_start(&self) -> usize {
        self.jump.row_start()
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
        self.lines = headings;
    }

    pub fn render(&self, buf: &mut Buffer, content_start: usize, content_end: usize) {
        let width = self.area.width;
        if width == 0 {
            return;
        }
        let (x, mut y, width) = (self.area.x, self.area.y, width as usize);
        let mut gap_str = XString::const_new("");
        let lines = &self.lines.lines;
        for (idx, line) in lines.iter().enumerate() {
            let text = &line.as_str();
            let text = text.get(..width.min(text.len())).unwrap_or("");
            let row_start = line.jump.row_start();
            // highlight the heading when the heading line is in visual range or
            // only the contents after the heading is in visual range
            if (content_start <= row_start && content_end > row_start)
                || (content_start > row_start
                    && lines
                        .get(idx + 1)
                        .map(|l| content_end < l.jump.row_start())
                        .unwrap_or(true))
            {
                render_line_fill_gap(Some(text), HEAD, buf, x, y, width, &mut gap_str);
            } else {
                let style = line.line.style;
                render_line(Some((text, style)), buf, x, y, width);
            }
            y += 1;
        }
    }
}
