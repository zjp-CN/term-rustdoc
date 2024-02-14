use smallvec::SmallVec;
use term_rustdoc::util::{hashmap, HashMap, XString};

/// The selected texts will be rendered with original fg but grey bg.
///
/// NOTE: the region is continuous across lines.
#[derive(Default, Debug, Hash, PartialEq, Eq)]
pub struct SelectedRegion {
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
}

#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum RegionTag {
    // /// A continuous region on screen.
    // OnScreen(SelectedRegion),
    /// A string key like headings or impl or key of a footnote.
    Heading(u8, XString),
    FootNote(XString),
    FootNoteSrc(XString),
    /// A referencd link id
    Link(usize),
    LinkSrc(usize),
}

pub struct TargetRegion {
    targets: SmallVec<[SelectedRegion; 1]>,
}

/// Regions that bidirect to each other.
/// When the cursor or selection falls into a region,
/// the regions in targets will be into the same background color.
pub struct LinkedRegions {
    tag: HashMap<RegionTag, TargetRegion>,
    // regions: Vec<(SelectedRegion, TargetRegion)>,
}

impl LinkedRegions {
    pub fn new() -> LinkedRegions {
        LinkedRegions {
            tag: hashmap(8),
            // regions: Vec::with_capacity(4),
        }
    }
}
