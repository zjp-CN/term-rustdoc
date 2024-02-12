use ratatui::style::Style;
use smallvec::SmallVec;
use std::{cell::RefCell, fmt, rc::Rc};
use term_rustdoc::{
    tree::Text,
    util::{hashmap, HashMap, XString},
};

pub struct StyledText {
    text: Text,
    interaction: Option<Interaction>,
}

impl fmt::Debug for StyledText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Debug>::fmt(self.as_str(), f)
    }
}

impl StyledText {
    pub fn new_plain<T: Into<XString>>(text: T, style: Style) -> Self {
        StyledText {
            text: Text {
                text: text.into(),
                style,
            },
            interaction: None,
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
}

pub struct Interaction {
    map: Rc<RefCell<HashMap<ColumnRange, SmallVec<[BiDirection; 1]>>>>,
    range: ColumnRange,
}

/// column range on screen
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct ColumnRange {
    start: u16,
    end: u16,
}

/// When the cursor is on the text range in pos or bidirections, the background color is added.
struct BiDirection {
    row: u16,
    col_start: u16,
    col_end: u16,
}
