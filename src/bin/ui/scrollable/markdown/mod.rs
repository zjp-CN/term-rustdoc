/// Use the custom markdown highlighting based on parsing contents to wrap texts.
/// But still can fall back to syntect's highlights without text wrapping.
mod fallback;
/// markdown headings
mod heading;
mod parse;
/// A continuous region that may be across lines.
mod region;
mod render;
/// cached and styled lines that are wrapped and incompletely highlighted
mod wrapped;

/// A rendering widget that contains
/// * a scrollable markdown area with texts wrapped
/// * a scrollable, auto-updated and clickable heading area
mod ingerated;

pub use self::{
    fallback::ScrollText,
    heading::{Headings, ScrollHeading},
    ingerated::{MarkdownAndHeading, MarkdownArea, ScrollMarkdown},
    wrapped::StyledText,
};
