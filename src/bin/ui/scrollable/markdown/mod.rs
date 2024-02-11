/// fall back to syntect's highlights without text wrapping
mod fallback;
mod parse;
mod render;

pub use fallback::ScrollText;
