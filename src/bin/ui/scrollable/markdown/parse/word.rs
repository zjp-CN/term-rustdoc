use super::{segment_words, MetaTag};
use ratatui::style::{Color, Style};
use rustc_hash::FxHashMap;
use std::{
    fmt::{self, Write},
    hash::BuildHasherDefault,
};
use term_rustdoc::util::XString;
use textwrap::core::Fragment;
use unicode_width::UnicodeWidthStr;

/// A block that represents a region like Paragraph, CodeBlock, QuoteBlock, Rule etc.
#[derive(Default, Debug)]
pub struct Block {
    lines: Vec<Line>,
    links: Vec<usize>,
    footnotes: Vec<XString>,
}

impl FromIterator<Line> for Block {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        Block {
            lines: Vec::from_iter(iter),
            links: Vec::new(),
            footnotes: Vec::new(),
        }
    }
}

impl FromIterator<Word> for Block {
    fn from_iter<T: IntoIterator<Item = Word>>(iter: T) -> Self {
        let mut lines = Vec::with_capacity(8);
        lines.push(Line::from_iter(iter));
        Block {
            lines,
            links: Vec::new(),
            footnotes: Vec::new(),
        }
    }
}

impl Extend<Line> for Block {
    fn extend<T: IntoIterator<Item = Line>>(&mut self, iter: T) {
        self.lines.extend(iter);
    }
}

impl Extend<Word> for Block {
    fn extend<T: IntoIterator<Item = Word>>(&mut self, iter: T) {
        self.last_line().extend(iter);
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

impl Block {
    /// Always get the last line: if it's not present, create a empty one before returning it.
    fn last_line(&mut self) -> &mut Line {
        if self.lines.is_empty() {
            self.lines.push(Line {
                words: Vec::with_capacity(16),
            });
        }
        self.lines.last_mut().unwrap()
    }

    /// Append a normal **and** the exact word to the last line.
    pub fn push_a_normal_and_exact_word(&mut self, word: &str) {
        let word = Word {
            word: word.into(),
            ..Default::default()
        };
        let last_line = self.last_line();
        last_line.words.push(word);
    }

    /// Append normal words segmented from the input to the last line.
    pub fn push_normal_words(&mut self, words: &str) {
        let last_line = self.last_line();
        segment_words(words, |word, trailling_whitespace| {
            last_line.words.push(Word {
                word: word.into(),
                trailling_whitespace,
                ..Default::default()
            })
        });
    }

    /// Append specified style and tag words segmented from the input to the last line.
    pub fn push_words(&mut self, words: &str, style: Style, tag: MetaTag) {
        let last_line = self.last_line();
        segment_words(words, |word, trailling_whitespace| {
            last_line.words.push(Word {
                word: word.into(),
                style,
                tag: tag.clone(),
                trailling_whitespace,
            })
        });
    }

    /// Append a constructed word to the last line.
    pub fn push_a_word(&mut self, word: Word) {
        self.last_line().words.push(word);
    }

    /// Shrink the vec of words, lines and links.
    ///
    /// NOTE:this also remove the last empty line. Empty line means there is no words in the line.
    pub fn shrink_to_fit(&mut self) {
        if let Some(true) = self.lines.last().map(|line| line.words.is_empty()) {
            // remove the last line with zero words
            self.lines.pop();
        }
        self.lines.shrink_to_fit();
        for line in &mut self.lines {
            line.words.shrink_to_fit();
        }
        self.links.shrink_to_fit();
    }

    pub fn heading(level: u8, text: &str) -> Block {
        Block {
            lines: Vec::from([Line {
                words: Vec::from([Word {
                    word: text.into(),
                    style: Style {
                        fg: Some(Color::LightCyan),
                        ..Default::default()
                    },
                    tag: MetaTag::Heading(level),
                    trailling_whitespace: false,
                }]),
            }]),
            links: Vec::new(),
            footnotes: Vec::new(),
        }
    }

    /// QuoteBlocks are Paragraphs tagged with QuoteBlock and slightly different rendering style.
    /// We firstly generate a QuoteBlock from a Paragraph, and now modify the tag and style.
    pub fn set_quote_block(&mut self) {
        for line in &mut self.lines {
            for word in &mut line.words {
                word.tag = MetaTag::QuoteBlock;
                word.style.fg = Some(Color::Rgb(186, 85, 211)); // #BA55D3
            }
        }
    }

    /// We firstly generate a Footnote from a Paragraph, and now modify the tag and style.
    pub fn set_foot_note(&mut self) {
        for line in &mut self.lines {
            for word in &mut line.words {
                word.tag = MetaTag::FootnoteSource;
                word.style.fg = Some(Color::LightMagenta);
            }
        }
    }
}

/// The whole documentation for an item.
#[derive(Debug, Default)]
pub struct Blocks {
    pub blocks: Vec<Block>,
    pub links: Links,
}

impl fmt::Display for Blocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.blocks {
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

impl Blocks {
    /// Append a block.
    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// Get the `&mut Links`.
    pub fn links(&mut self) -> &mut Links {
        &mut self.links
    }
}

impl Blocks {
    pub fn new() -> Blocks {
        Blocks {
            blocks: Vec::with_capacity(16),
            links: Links {
                links: Vec::with_capacity(8),
                footnotes: FxHashMap::with_capacity_and_hasher(1, BuildHasherDefault::default()),
            },
        }
    }

    pub fn shrink_to_fit(&mut self) {
        self.blocks.iter_mut().for_each(Block::shrink_to_fit);
        self.blocks.shrink_to_fit();
        self.links.links.shrink_to_fit();
        self.links.footnotes.shrink_to_fit();
    }
}

/// Referenced links/footnotes in the whole doc.
#[derive(Default, Debug)]
pub struct Links {
    links: Vec<XString>,
    footnotes: FxHashMap<XString, Block>,
}

impl Links {
    pub fn push_link(&mut self, link: XString) -> usize {
        self.links
            .iter()
            .position(|l| *l == link)
            .unwrap_or_else(|| {
                let len = self.links.len();
                self.links.push(link);
                len
            })
    }

    fn get_link(&self, idx: usize) -> Option<&str> {
        self.links.get(idx).map(XString::as_str)
    }

    pub fn push_footnote(&mut self, key: &str, value: Block) {
        if let Some(old) = self.footnotes.insert(key.into(), value) {
            error!("Footnote definition `{key}` existed with the value {old:?}, but now covered.");
        }
    }

    pub fn get_footnote(&self, key: &str) -> Option<&Block> {
        self.footnotes.get(key)
    }
}

/// A line to be rendered on screen, containing multiple words.
///
/// For a line in a Paragraph block, texts are usually wrapped at fixed width.
#[derive(Default, Debug)]
pub struct Line {
    words: Vec<Word>,
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
