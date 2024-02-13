use super::{block::Block, element::LINK, LinkTag, MetaTag, Word};
use crate::ui::scrollable::markdown::{fallback::StyledLine, region::LinkedRegions};
use ratatui::style::{Color, Style};
use std::fmt;
use term_rustdoc::util::{hashmap, xformat, HashMap, XString};
use textwrap::wrap_algorithms::{wrap_optimal_fit, Penalties};

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
                footnotes: hashmap(1),
            },
        }
    }

    pub fn shrink_to_fit(&mut self) {
        self.blocks.iter_mut().for_each(Block::shrink_to_fit);
        self.blocks.shrink_to_fit();
        self.links.links.shrink_to_fit();
        self.links.footnotes.shrink_to_fit();
    }

    pub fn write_styled_lines(&self, width: f64, slines: &mut Vec<StyledLine>) {
        let mut writer = WriteLines::new(slines);
        let penalties = Default::default();
        for block in &self.blocks {
            writer.write_block_as_styled_lines(block, width, penalties);
            if !block.links().is_empty() {
                writer.write_empty_styled_line();
                block.links().iter().copied().for_each(|idx| {
                    if let Some(word) = self.links.get_link(idx) {
                        let anchor = Word {
                            word: xformat!("[{idx}]:"),
                            style: LINK,
                            tag: MetaTag::Link(LinkTag::ReferenceLink(idx)),
                            trailling_whitespace: true,
                        };
                        let width = width as usize;
                        let mut link = &word.word[..];
                        if link.len() + anchor.word.len() + 1 > width {
                            // when the link is too long, split it to multiple lines
                            writer.write_styled_line(anchor);
                            while !link.is_empty() {
                                let end = width.min(link.len());
                                let line = Word {
                                    word: link[..end].into(),
                                    style: LINK,
                                    tag: MetaTag::Link(LinkTag::ReferenceLink(idx)),
                                    trailling_whitespace: false,
                                };
                                writer.write_styled_line(line);
                                link = &link[end..];
                            }
                        } else {
                            writer.write_styled_line(&[anchor, word][..]);
                        }
                    }
                });
            }
            if !block.footnotes().is_empty() {
                writer.write_empty_styled_line();
                block.footnotes().iter().for_each(|key| {
                    if let Some(block) = self.links.get_footnote(key) {
                        writer.write_block_as_styled_lines(block, width, penalties);
                    }
                });
            }
            writer.write_empty_styled_line();
        }
    }
}

/// Append a line to vec of StyledLine which is from StyledLines.
/// This writer also generates LinkedRegions.
struct WriteLines<'lines> {
    lines: &'lines mut Vec<StyledLine>,
    regions: LinkedRegions,
}

impl<'lines> WriteLines<'lines> {
    fn new(lines: &'lines mut Vec<StyledLine>) -> WriteLines<'lines> {
        WriteLines {
            lines,
            regions: LinkedRegions::new(),
        }
    }

    fn write_block_as_styled_lines(&mut self, block: &Block, width: f64, penalties: Penalties) {
        for line in block.lines() {
            match wrap_optimal_fit(line, &[width], &penalties) {
                Ok(lines) => lines.into_iter().for_each(|l| self.write_styled_line(l)),
                Err(err) => error!("failed to wrap the line to width {width}:{err}\n{line:?} "),
            };
        }
    }

    fn write_styled_line(&mut self, line: impl Into<StyledLine>) {
        self.lines.push(line.into());
    }

    fn write_empty_styled_line(&mut self) {
        self.lines.push(StyledLine::new());
    }
}

/// Referenced links/footnotes in the whole doc.
#[derive(Default, Debug)]
pub struct Links {
    links: Vec<XString>,
    footnotes: HashMap<XString, Block>,
}

impl Links {
    pub fn push_link(&mut self, link: XString) -> usize {
        // check if the same link exists; if exists, use that idx
        self.links
            .iter()
            .position(|l| *l == link)
            .unwrap_or_else(|| {
                let len = self.links.len();
                self.links.push(link);
                len
            })
    }

    pub fn get_link(&self, idx: usize) -> Option<Word> {
        self.links.get(idx).map(|link| Word {
            word: link.clone(),
            style: Style {
                fg: Some(Color::Rgb(30, 144, 255)), // #1E90FF
                ..Default::default()
            },
            tag: MetaTag::Link(LinkTag::ReferenceLink(idx)),
            trailling_whitespace: false,
        })
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
