use super::{word::Word, MetaTag};
use ratatui::prelude::{Color, Modifier, Style};
use std::{fmt, ops::Deref};
use term_rustdoc::util::XString;

/// A line to be rendered on screen, containing multiple words.
///
/// For a line in a Paragraph block, texts are usually wrapped at fixed width.
#[derive(Default, Debug, Clone)]
pub struct Line {
    pub words: Vec<Word>,
}

impl FromIterator<Word> for Line {
    fn from_iter<T: IntoIterator<Item = Word>>(iter: T) -> Self {
        Line {
            words: Vec::from_iter(iter),
        }
    }
}

impl Extend<Word> for Line {
    fn extend<T: IntoIterator<Item = Word>>(&mut self, iter: T) {
        self.words.extend(iter);
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.words.len();
        self.words.iter().enumerate().try_for_each(|(idx, word)| {
            if word.trailling_whitespace && idx + 1 != len {
                write!(f, "{} ", word.word)
            } else {
                write!(f, "{}", word.word)
            }
        })
    }
}

impl Deref for Line {
    type Target = [Word];

    fn deref(&self) -> &Self::Target {
        &self.words
    }
}

impl Line {
    pub fn backtick(text: &str, fence: XString) -> [Line; 2] {
        let mut words = Vec::with_capacity(2);
        let mut start = 0;
        if let Some(split) = text.find('`') {
            words.push(Word {
                word: text[..split].into(),
                ..Default::default()
            });
            start = split;
        }
        words.push(Word {
            word: text[start..].into(),
            style: Style {
                fg: Some(Color::Red),
                add_modifier: Modifier::BOLD,
                ..Style::new()
            },
            tag: MetaTag::CodeBlock(fence.clone()),
            trailling_whitespace: false,
        });
        let pair2 = Line { words };
        let mut pair1 = pair2.words.clone();
        if let Some(tick) = pair1.last_mut() {
            let lang = if fence.is_empty() { "rust" } else { &fence };
            tick.word.push_str(lang);
        }
        [Line { words: pair1 }, pair2]
    }
}
