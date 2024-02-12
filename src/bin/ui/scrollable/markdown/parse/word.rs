use crate::ui::scrollable::markdown::{fallback::StyledLine, StyledText};

use super::MetaTag;
use ratatui::style::Style;
use std::fmt::{self, Write};
use term_rustdoc::util::XString;
use textwrap::core::Fragment;
use unicode_width::UnicodeWidthStr;

/// A word that has styling and metadata.
///
/// For Paragraphs or QuoteBlocks, words are wrapped in a line.
/// For non-wrappable blocks, like in CodeBlocks, words in a line are truncated.
#[derive(Default, Clone)]
pub struct Word {
    /// NOTE: the word doesn't contain trailling whitespace,
    /// so when generating an owned text, we should use the
    /// `trailling_whitespace` to add it back.
    pub word: XString,
    pub style: Style,
    pub tag: MetaTag,
    /// serves as two purposes:
    /// * indicates the word has an trailling whitespace when the word is amid the line
    ///   as wrapping algorithm needs
    /// * since the style may extend to this potential whitespace, if the value is false,
    ///   we don't generate a whitespace in owned styled text; but if true, we should do.
    pub trailling_whitespace: bool,
}

impl Word {}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Word");
        s.field("word", &self.word);
        let style = Style::default();
        if self.style != style {
            // if self.style.fg != style.fg {
            //     s.field("style.fg", &self.style.fg);
            // }
            if self.style.add_modifier != style.add_modifier {
                s.field("style.add_modifier", &self.style.add_modifier);
            }
        }
        if !matches!(self.tag, MetaTag::Normal) {
            s.field("tag", &self.tag);
        }
        if self.trailling_whitespace {
            s.field("trailling_whitespace", &true);
        }
        s.finish()
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Display>::fmt(&self.word, f)?;
        if self.trailling_whitespace {
            f.write_char(' ')?;
        }
        Ok(())
    }
}

impl Fragment for Word {
    /// word width without whitespace before or after
    fn width(&self) -> f64 {
        self.word.width() as f64
    }

    /// occurence of trailing whitespace, like 0 for CJK or 1 for latin etc
    fn whitespace_width(&self) -> f64 {
        if self.trailling_whitespace {
            1.0
        } else {
            0.0
        }
    }

    /// imaginary extra width after the non-line-end word that the wrapping algorithm accepts
    fn penalty_width(&self) -> f64 {
        0.0
    }
}

impl From<Word> for StyledText {
    fn from(word: Word) -> Self {
        let mut text = word.word.clone();
        if word.trailling_whitespace {
            text.push(' ');
        }
        StyledText::new_plain(text, word.style)
    }
}

impl From<Word> for StyledLine {
    fn from(word: Word) -> Self {
        StyledLine {
            line: vec![word.into()],
        }
    }
}

impl From<&[Word]> for StyledLine {
    fn from(value: &[Word]) -> Self {
        StyledLine {
            line: value.iter().cloned().map(StyledText::from).collect(),
        }
    }
}
