use super::word::Word;
use std::{fmt, ops::Deref};

/// A line to be rendered on screen, containing multiple words.
///
/// For a line in a Paragraph block, texts are usually wrapped at fixed width.
#[derive(Default, Debug)]
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
