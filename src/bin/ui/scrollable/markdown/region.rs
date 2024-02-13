use smallvec::SmallVec;

/// The selected texts will be rendered with original fg but grey bg.
///
/// NOTE: the region is continuous across lines.
#[derive(Default, Debug)]
pub struct SelectedRegion {
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
}

/// When the cursor or selection falls into this region,
/// the regions in targets will be into the same background color.
pub struct Linkage {
    selected: SelectedRegion,
    targets: SmallVec<[SelectedRegion; 1]>,
}

/// Regions that bidirect to each other.
pub struct LinkedRegions {
    regions: Vec<Linkage>,
}

impl LinkedRegions {
    pub fn new() -> LinkedRegions {
        LinkedRegions {
            regions: Vec::with_capacity(8),
        }
    }
}
