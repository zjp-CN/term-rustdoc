#![allow(unused)] // TargetRegion is reserved for bidirection of referencd links
use super::{
    parse::{LinkTag, MetaTag},
    wrapped::ColumnSpan,
};
use smallvec::SmallVec;
use std::cmp::Ordering;
use term_rustdoc::util::{hashmap, HashMap, XString};

/// The selected texts will be rendered with original fg but grey bg.
///
/// NOTE: the region is continuous across lines.
#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
pub struct SelectedRegion {
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
}

impl SelectedRegion {
    fn new_same_line(row: usize, col: ColumnSpan) -> Self {
        let [start, end] = col.span();
        SelectedRegion {
            row_start: row,
            row_end: row,
            col_start: start,
            col_end: end,
        }
    }

    /// Merge two regions into one continuous region.
    ///
    /// NOTE: usually merge them into one larger continuous region
    /// * this means it's not used to merge usage regions of links or footnotes,
    ///   because they can scatter in discontinuous lines.
    fn merge_continuous(&mut self, new: SelectedRegion) {
        match self.row_start.cmp(&new.row_start) {
            Ordering::Greater => {
                self.row_start = new.row_start;
                self.col_start = new.col_start;
            }
            Ordering::Equal => self.col_start = self.col_start.min(new.col_start),
            Ordering::Less => {}
        }
        match self.row_end.cmp(&new.row_end) {
            Ordering::Less => {
                self.row_end = new.row_end;
                self.col_end = new.col_end;
            }
            Ordering::Equal => self.col_end = self.col_end.max(new.col_end),
            Ordering::Greater => {}
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum RegionTag {
    // /// A continuous region on screen.
    // OnScreen(SelectedRegion),
    /// A string key like impl or key of a footnote.
    FootNote(XString),
    FootNoteSrc(XString),
    /// A referencd link id
    Link(usize),
    LinkSrc(usize),
}

/// Multiple SelectedRegions, but in most cases still single SelectedRegion.
///
/// ReferenceLinks usually only have one linked region, but it's still common to
/// have multiple linked regions.
#[derive(Clone, Debug, Default)]
pub struct TargetRegion {
    targets: SmallVec<[SelectedRegion; 1]>,
}

impl From<SelectedRegion> for TargetRegion {
    fn from(region: SelectedRegion) -> Self {
        TargetRegion {
            targets: SmallVec::from([region]),
        }
    }
}

/// Regions that bidirect to each other.
/// When the cursor or selection falls into a region,
/// the regions in targets will be into the same background color.
#[derive(Debug, Default)]
pub struct LinkedRegions {
    tag: HashMap<RegionTag, TargetRegion>,
    heading: Vec<(usize, SelectedRegion)>,
}

impl LinkedRegions {
    pub fn new() -> LinkedRegions {
        LinkedRegions {
            tag: hashmap(8),
            heading: Vec::with_capacity(8),
        }
    }

    pub fn push_heading(&mut self, idx: usize, row: usize, col: ColumnSpan) {
        let region = SelectedRegion::new_same_line(row, col);
        // since the writer writes lines from top to bottom, headings will be in order
        if let Some((index, old)) = self.heading.last_mut() {
            if *index == idx {
                old.merge_continuous(region);
                return;
            }
        }
        self.heading.push((idx, region));
    }

    pub fn take_headings(&mut self) -> Vec<(usize, SelectedRegion)> {
        std::mem::take(&mut self.heading)
    }
}

pub fn region_tag(tag: MetaTag) -> Option<RegionTag> {
    match tag {
        MetaTag::Link(LinkTag::ReferenceLink(id)) => Some(RegionTag::Link(id)),
        MetaTag::Link(LinkTag::Footnote(key)) => Some(RegionTag::FootNote(key)),
        _ => None,
    }
}
