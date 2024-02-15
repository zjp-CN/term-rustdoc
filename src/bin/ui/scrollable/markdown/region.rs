use super::{
    parse::{LinkTag, MetaTag},
    wrapped::ColumnSpan,
};
use smallvec::SmallVec;
use std::{cmp::Ordering, collections::hash_map::Entry};
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

impl TargetRegion {
    /// Merge two regions into one continuous region.
    ///
    /// NOTE: this method means
    /// * the merge happens only when there is single region in self
    /// * usually merge them into one larger continuous region
    ///   * this means it's not used to merge usage regions of links or footnotes,
    ///     because they can scatter in discontinuous lines.
    pub fn merge_continuous(&mut self, new: SelectedRegion) {
        if self.targets.len() == 1 {
            let old = &mut self.targets[0];
            match old.row_start.cmp(&new.row_start) {
                Ordering::Greater => {
                    old.row_start = new.row_start;
                    old.col_start = new.col_start;
                }
                Ordering::Equal => old.col_start = old.col_start.min(new.col_start),
                Ordering::Less => {}
            }
            match old.row_end.cmp(&new.row_end) {
                Ordering::Less => {
                    old.row_end = new.row_end;
                    old.col_end = new.col_end;
                }
                Ordering::Equal => old.col_end = old.col_end.max(new.col_end),
                Ordering::Greater => {}
            }
        }
    }
}

/// Regions that bidirect to each other.
/// When the cursor or selection falls into a region,
/// the regions in targets will be into the same background color.
#[derive(Debug, Default)]
pub struct LinkedRegions {
    tag: HashMap<RegionTag, TargetRegion>,
    heading: Vec<(usize, TargetRegion)>,
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
        for (index, old) in &mut self.heading {
            if *index == idx {
                old.merge_continuous(region);
                return;
            }
        }
        self.heading.push((idx, region.into()));
    }

    pub fn take_headings(&mut self) -> Vec<(usize, TargetRegion)> {
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
