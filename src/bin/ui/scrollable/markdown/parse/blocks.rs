use super::block::Block;
use rustc_hash::FxHashMap;
use std::{fmt, hash::BuildHasherDefault};
use term_rustdoc::util::XString;

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
