use ratatui::style::Style;
use std::fmt;
use term_rustdoc::{tree::Text, util::XString};
use unicode_width::UnicodeWidthStr;

pub struct StyledText {
    text: Text,
    span: ColumnSpan,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColumnSpan {
    start: usize,
    end: usize,
}

impl ColumnSpan {
    pub fn span(self) -> [usize; 2] {
        [self.start, self.end]
    }
}

impl fmt::Debug for StyledText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Debug>::fmt(self.as_str(), f)
    }
}

impl StyledText {
    pub fn new<T: Into<XString>>(text: T, style: Style, start: usize) -> Self {
        let text = text.into();
        let end = start + text.width();
        StyledText {
            text: Text { text, style },
            span: ColumnSpan { start, end },
        }
    }

    pub fn text(&self) -> XString {
        self.text.text.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.text.text
    }

    pub fn style(&self) -> Style {
        self.text.style
    }

    pub fn span_end(&self) -> usize {
        self.span.end
    }

    pub fn span(&self) -> ColumnSpan {
        self.span.clone()
    }
}
