use super::region::SelectedRegion;
use crate::ui::scrollable::{generics::LineState, Scrollable};
use term_rustdoc::{tree::Text, util::XString};

pub type ScrollHeading = Scrollable<Headings>;

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
        self.max_windth = max_windth;
    }
}
