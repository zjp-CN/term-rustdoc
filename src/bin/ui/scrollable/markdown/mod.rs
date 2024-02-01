use super::Scrollable;
use std::fmt;
use term_rustdoc::tree::Text as StyledText;

/// Scrollable text area for displaying markdown.
pub type ScrollText = Scrollable<Vec<StyledLine>, StyledLine>;

#[derive(Default)]
pub struct StyledLine {
    line: Vec<StyledText>,
}

impl fmt::Debug for StyledLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StyledLine")
            .field("line-len", &self.line.len())
            .finish()
    }
}

impl AsRef<[StyledText]> for StyledLine {
    fn as_ref(&self) -> &[StyledText] {
        &self.line
    }
}
