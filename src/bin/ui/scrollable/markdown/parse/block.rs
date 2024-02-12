use super::{line::Line, word::Word};
use super::{segment_words, MetaTag};
use ratatui::style::{Color, Modifier, Style};
use std::fmt;
use term_rustdoc::util::XString;

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

    pub fn set_heading(&mut self, level: u8) {
        for line in &mut self.lines {
            for word in &mut line.words {
                word.tag = MetaTag::Heading(level);
                word.style.fg = Some(Color::LightCyan);
                word.style.add_modifier = Modifier::BOLD;
            }
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

impl Block {
    pub fn lines(&self) -> &[Line] {
        &self.lines
    }

    pub fn links(&self) -> &[usize] {
        &self.links
    }

    pub fn footnotes(&self) -> &[XString] {
        &self.footnotes
    }
}
