/// fall back to syntect's highlights without text wrapping
mod fallback;
/// markdown headings
mod heading;
mod parse;
/// A continuous region that may be across lines.
mod region;
mod render;
/// cached and styled lines that are wrapped and incompletely highlighted
mod wrapped;

pub use self::{fallback::ScrollText, heading::ScrollHeading, wrapped::StyledText};
