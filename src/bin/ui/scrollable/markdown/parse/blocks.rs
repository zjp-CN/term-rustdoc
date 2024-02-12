use super::{block::Block, LinkTag, MetaTag, Word};
use crate::ui::scrollable::markdown::fallback::StyledLine;
use ratatui::style::{Color, Style};
use std::fmt;
use term_rustdoc::util::{hashmap, HashMap, XString};
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
        let penalties = Default::default();
        for block in &self.blocks {
            write_block_as_styled_lines(block, width, penalties, slines);
            if !block.links().is_empty() {
                write_empty_styled_line(slines);
                block.links().iter().copied().for_each(|idx| {
                    if let Some(word) = self.links.get_link(idx) {
                        write_styled_line(slines, word);
                    }
                });
            }
            if !block.footnotes().is_empty() {
                write_empty_styled_line(slines);
                block.footnotes().iter().for_each(|key| {
                    if let Some(block) = self.links.get_footnote(key) {
                        write_block_as_styled_lines(block, width, penalties, slines);
                    }
                });
            }
            write_empty_styled_line(slines);
        }
    }
}

fn write_block_as_styled_lines(
    block: &Block,
    width: f64,
    penalties: Penalties,
    slines: &mut Vec<StyledLine>,
) {
    for line in block.lines() {
        match wrap_optimal_fit(line, &[width], &penalties) {
            Ok(lines) => lines.into_iter().for_each(|l| write_styled_line(slines, l)),
            Err(err) => error!("failed to wrap the line to width {width}:{err}\n{line:?} "),
        };
    }
}

fn write_styled_line(slines: &mut Vec<StyledLine>, line: impl Into<StyledLine>) {
    slines.push(line.into());
}

fn write_empty_styled_line(slines: &mut Vec<StyledLine>) {
    slines.push(StyledLine { line: Vec::new() });
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
