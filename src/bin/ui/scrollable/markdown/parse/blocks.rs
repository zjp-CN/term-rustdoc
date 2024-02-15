use super::{element::LINK, Block, Line, LinkTag, MetaTag, Word};
use crate::ui::scrollable::markdown::{
    fallback::StyledLine,
    heading::Headings,
    region::{LinkedRegions, SelectedRegion},
};
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
                heading: Vec::new(),
                links: Vec::with_capacity(8),
                footnotes: hashmap(1),
            },
        }
    }

    pub fn shrink_to_fit(&mut self) {
        self.blocks.iter_mut().for_each(Block::shrink_to_fit);
        self.blocks.shrink_to_fit();
        self.links.heading.shrink_to_fit();
        self.links.links.shrink_to_fit();
        self.links.footnotes.shrink_to_fit();
    }

    pub fn write_styled_lines(&mut self, width: f64) -> Vec<StyledLine> {
        let mut writer = WriteLines::new(width);
        for block in &self.blocks {
            writer.write_lines(block.lines());
            if !block.links().is_empty() {
                writer.write_empty_line();
                for &idx in block.links() {
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
                            writer.write_line(&[anchor]);
                            while !link.is_empty() {
                                let end = width.min(link.len());
                                let line = Word {
                                    word: link[..end].into(),
                                    style: LINK,
                                    tag: MetaTag::Link(LinkTag::ReferenceLink(idx)),
                                    trailling_whitespace: false,
                                };
                                writer.write_line(&[line]);
                                link = &link[end..];
                            }
                        } else {
                            writer.write_line(&[anchor, word]);
                        }
                    }
                }
            }
            if !block.footnotes().is_empty() {
                writer.write_empty_line();
                for key in block.footnotes() {
                    if let Some(block) = self.links.get_footnote(key) {
                        writer.write_lines(block.lines());
                    }
                }
            }
            writer.write_empty_line();
        }
        writer.split(self.links())
    }
}

/// Append a line to vec of StyledLine which is from StyledLines.
/// This writer also generates LinkedRegions.
struct WriteLines {
    lines: Vec<StyledLine>,
    regions: LinkedRegions,
    width: f64,
    penalties: Penalties,
}

impl WriteLines {
    fn new(width: f64) -> WriteLines {
        WriteLines {
            lines: Vec::with_capacity(128),
            regions: LinkedRegions::new(),
            width,
            penalties: Penalties::default(),
        }
    }

    fn write_lines(&mut self, lines: &[Line]) {
        let width = self.width;
        for line in lines {
            match wrap_optimal_fit(line, &[width], &self.penalties) {
                Ok(lines) => lines.into_iter().for_each(|l| self.write_line(l)),
                Err(err) => error!("failed to wrap the line to width {width}:{err}\n{line:?} "),
            };
        }
    }

    fn write_line(&mut self, words: &[Word]) {
        self.lines.push(Word::words_to_line(
            words,
            self.lines.len(),
            &mut self.regions,
        ));
    }

    fn write_empty_line(&mut self) {
        self.lines.push(StyledLine::new());
    }

    fn split(mut self, links: &mut Links) -> Vec<StyledLine> {
        links.set_heading_regions(self.regions.take_headings());
        self.lines
    }
}

/// Referenced links/footnotes in the whole doc.
#[derive(Default, Debug)]
pub struct Links {
    heading: Vec<(u8, XString, SelectedRegion)>,
    links: Vec<XString>,
    // FIXME: replace this HashMap with Vec<(XString, Block)>,
    // and use the index as key/id like push_link returns.
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

    pub fn push_heading(&mut self, level: u8, raw: &str) -> usize {
        let id = self.heading.len();
        self.heading
            .push((level, raw.into(), SelectedRegion::default()));
        id
    }

    pub fn set_heading_regions(&mut self, mut regions: Vec<(usize, SelectedRegion)>) {
        regions.sort_unstable_by_key(|(idx, _)| *idx);
        if regions.len() != self.heading.len() {
            error!(
                "regions of heading has {} ids {:?}\nbut headings in blocks has {} ids {:?}",
                regions.len(),
                regions.iter().map(|r| r.0).collect::<Vec<_>>(),
                self.heading.len(),
                self.heading.iter().map(|r| r.0).collect::<Vec<_>>(),
            );
        }
        for (idx, region) in regions {
            if let Some((_, _, old)) = self.heading.get_mut(idx) {
                *old = region;
            } else {
                error!("the heading id {idx} from regions doesn't exist in Links");
            }
        }
    }

    pub fn to_heading(&self) -> Headings {
        let Some(top) = self.heading.iter().map(|h| h.0).min() else {
            return Headings::default();
        };
        let mut headings = Headings::with_capacity(self.heading.len());
        for (level, text, region) in &self.heading {
            let mut heading = XString::default();
            (0..level.saturating_sub(top)).for_each(|_| heading.push_str("  "));
            heading.push_str(text.trim_start().trim_start_matches('#').trim());
            headings.push(heading, region.clone());
        }
        headings
    }
}
