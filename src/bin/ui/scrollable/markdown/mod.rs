/// fall back to syntect's highlights without text wrapping
mod fallback;
/// A continuous region that may be across lines.
mod region;
/// cached and styled lines that are wrapped and incompletely highlighted
mod wrapped;

mod parse;
mod render;

pub use fallback::ScrollText;
pub use wrapped::StyledText;
