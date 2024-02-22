use super::Cache;
use crate::{database::CachedDocInfo, ui::LineState};
use std::cmp::Ordering;
use term_rustdoc::{tree::CrateDoc, util::XString};

pub struct LoadedDoc {
    pub info: CachedDocInfo,
    pub doc: CrateDoc,
}

pub struct CacheID(pub usize);

impl LineState for CacheID {
    type State = usize;

    fn state(&self) -> Self::State {
        self.0
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        self.0 == *state
    }
}

#[derive(Default, Clone, Copy)]
pub struct Count {
    pub loaded: usize,
    pub unloaded: usize,
    pub in_progress: usize,
}

impl Count {
    pub fn describe(self) -> XString {
        use std::fmt::Write;
        let Count {
            loaded,
            unloaded,
            in_progress,
        } = self;
        let mut text = XString::new_inline(" ");
        if loaded != 0 {
            write!(&mut text, "Loaded: {loaded} / ").unwrap();
        }
        if unloaded != 0 {
            write!(&mut text, "Cached: {unloaded} / ").unwrap();
        }
        if in_progress != 0 {
            write!(&mut text, "HoldOn: {in_progress} / ").unwrap();
        }
        let total = loaded + unloaded + in_progress;
        if total != 0 {
            write!(&mut text, "Total: {total} ").unwrap();
        }
        text
    }
}

/// Sort kind for pkg list in database panel.
#[derive(Clone, Copy, Debug, Default)]
pub enum SortKind {
    #[default]
    TimeForAll,
    PkgKeyForAll,
    TimeGrouped,
    PkgKeyGrouped,
}

impl SortKind {
    pub fn cmp_fn(self) -> fn(&Cache, &Cache) -> Ordering {
        match self {
            SortKind::TimeForAll => Cache::cmp_by_time_for_all,
            SortKind::PkgKeyForAll => Cache::cmp_by_pkg_key_for_all,
            SortKind::TimeGrouped => Cache::cmp_by_time_grouped,
            SortKind::PkgKeyGrouped => Cache::cmp_by_pkg_key_grouped,
        }
    }

    pub fn next(self) -> Self {
        match self {
            SortKind::TimeForAll => SortKind::PkgKeyForAll,
            SortKind::PkgKeyForAll => SortKind::TimeGrouped,
            SortKind::TimeGrouped => SortKind::PkgKeyGrouped,
            SortKind::PkgKeyGrouped => SortKind::TimeForAll,
        }
    }

    pub fn describe(self) -> &'static str {
        match self {
            SortKind::TimeForAll => " [For All] Sort by time ",
            SortKind::PkgKeyForAll => " [For All] Sort by PkgKey ",
            SortKind::TimeGrouped => " [In Groups] Sort by time ",
            SortKind::PkgKeyGrouped => " [In Groups] Sort by PkgKey ",
        }
    }
}
