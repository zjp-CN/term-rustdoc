use crate::{
    database::{CachedDocInfo, PkgKey},
    ui::LineState,
};
use core::cmp::Ordering;
use ratatui::prelude::{Color, Style};
use semver::Version;
use std::time::SystemTime;
use term_rustdoc::{
    tree::CrateDoc,
    util::{xformat, XString},
};

pub struct LoadedDoc {
    info: CachedDocInfo,
    doc: CrateDoc,
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

#[derive(PartialEq, Eq)]
pub struct Cache {
    inner: CacheInner,
    ver: Version,
}

impl Cache {
    pub fn new_being_cached(pkg_key: PkgKey) -> Cache {
        Cache {
            ver: pkg_key.version(),
            inner: CacheInner::BeingCached(pkg_key, SystemTime::now()),
        }
    }

    pub fn new_unloaded(info: CachedDocInfo) -> Cache {
        Cache {
            ver: info.pkg.version(),
            inner: CacheInner::Unloaded(info),
        }
    }

    pub fn is_in_progress(&self, key: &PkgKey) -> bool {
        matches!(&self.inner, CacheInner::BeingCached(pkg, _) if pkg == key)
    }

    pub fn loadable(&self) -> bool {
        matches!(self.inner, CacheInner::Unloaded(_))
    }

    pub fn load_doc(self) -> Self {
        match self.inner {
            CacheInner::Unloaded(info) => match info.load_doc() {
                Ok(doc) => Cache {
                    inner: CacheInner::Loaded(LoadedDoc { info, doc }),
                    ver: self.ver,
                },
                Err(err) => {
                    error!("Failed to load {:?}:\n{err}", info.pkg);
                    Cache {
                        inner: CacheInner::Unloaded(info),
                        ver: self.ver,
                    }
                }
            },
            _ => self,
        }
    }

    pub fn line(&self) -> [(&str, Style); 3] {
        let kind = self.inner.kind();
        let key = self.inner.pkg_key();
        [
            kind,
            (
                key.name(),
                Style {
                    fg: Some(Color::White),
                    ..Style::new()
                },
            ),
            (
                key.ver_str(),
                Style {
                    fg: Some(Color::DarkGray),
                    ..Style::new()
                },
            ),
        ]
    }

    fn pkg_key(&self) -> &PkgKey {
        self.inner.pkg_key()
    }

    /// Sort by name, version and features, in groups.
    pub fn cmp_by_pkg_key_grouped(&self, other: &Self) -> Ordering {
        match (&self.inner, &other.inner) {
            (CacheInner::Loaded(a), CacheInner::Loaded(b)) => {
                match a.info.pkg.name().cmp(b.info.pkg.name()) {
                    Ordering::Equal => match self.ver.cmp(&other.ver) {
                        Ordering::Equal => {
                            let features1 = a.info.pkg.features();
                            let features2 = b.info.pkg.features();
                            features1.cmp(features2)
                        }
                        ord => ord,
                    },
                    ord => ord,
                }
            }
            (CacheInner::Loaded(_), _) => Ordering::Less,
            (CacheInner::BeingCached(_, _), CacheInner::Loaded(_)) => Ordering::Greater,
            (CacheInner::BeingCached(a, _), CacheInner::BeingCached(b, _)) => {
                match a.name().cmp(b.name()) {
                    Ordering::Equal => match self.ver.cmp(&other.ver) {
                        Ordering::Equal => {
                            let features1 = a.features();
                            let features2 = b.features();
                            features1.cmp(features2)
                        }
                        ord => ord,
                    },
                    ord => ord,
                }
            }
            (CacheInner::BeingCached(_, _), CacheInner::Unloaded(_)) => Ordering::Less,
            (CacheInner::Unloaded(a), CacheInner::Unloaded(b)) => {
                match a.pkg.name().cmp(b.pkg.name()) {
                    Ordering::Equal => match self.ver.cmp(&other.ver) {
                        Ordering::Equal => {
                            let features1 = a.pkg.features();
                            let features2 = b.pkg.features();
                            features1.cmp(features2)
                        }
                        ord => ord,
                    },
                    ord => ord,
                }
            }
            (CacheInner::Unloaded(_), _) => Ordering::Greater,
        }
    }

    /// Recent ones are first, in groups.
    pub fn cmp_by_time_grouped(&self, other: &Self) -> Ordering {
        match (&self.inner, &other.inner) {
            (CacheInner::Loaded(a), CacheInner::Loaded(b)) => {
                b.info.started_time().cmp(&a.info.started_time())
            }
            (CacheInner::Loaded(_), _) => Ordering::Less,
            (CacheInner::BeingCached(_, _), CacheInner::Loaded(_)) => Ordering::Greater,
            (CacheInner::BeingCached(_, a), CacheInner::BeingCached(_, b)) => b.cmp(a),
            (CacheInner::BeingCached(_, _), CacheInner::Unloaded(_)) => Ordering::Less,
            (CacheInner::Unloaded(a), CacheInner::Unloaded(b)) => {
                b.started_time().cmp(&a.started_time())
            }
            (CacheInner::Unloaded(_), _) => Ordering::Greater,
        }
    }

    /// Sort by name, version and features, for all.
    pub fn cmp_by_pkg_key_for_all(&self, other: &Self) -> Ordering {
        let name = self.pkg_key().name();
        match name.cmp(other.inner.pkg_key().name()) {
            Ordering::Equal => match self.ver.cmp(&other.ver) {
                Ordering::Equal => {
                    let features1 = self.pkg_key().features();
                    let features2 = other.inner.pkg_key().features();
                    features1.cmp(features2)
                }
                ord => ord,
            },
            ord => ord,
        }
    }

    /// Recent ones are first, for all.
    pub fn cmp_by_time_for_all(&self, other: &Self) -> Ordering {
        other.started_time().cmp(&self.started_time())
    }

    pub fn started_time(&self) -> SystemTime {
        match &self.inner {
            CacheInner::Loaded(loaded) => loaded.info.started_time(),
            CacheInner::Unloaded(unloaded) => unloaded.started_time(),
            CacheInner::BeingCached(_, time) => *time,
        }
    }

    pub fn add(&self, count: &mut Count) {
        match &self.inner {
            CacheInner::Loaded(_) => count.loaded += 1,
            CacheInner::Unloaded(_) => count.unloaded += 1,
            CacheInner::BeingCached(_, _) => count.in_progress += 1,
        }
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

impl PartialEq<PkgKey> for Cache {
    fn eq(&self, other: &PkgKey) -> bool {
        self.inner.pkg_key() == other
    }
}

impl PartialEq<Cache> for PkgKey {
    fn eq(&self, other: &Cache) -> bool {
        Cache::eq(other, self)
    }
}

pub enum CacheInner {
    /// cached & loaded pkg docs
    Loaded(LoadedDoc),
    /// cached but not loaded docs
    Unloaded(CachedDocInfo),
    /// pkgs which is being sent to compile doc
    BeingCached(PkgKey, SystemTime),
}

impl CacheInner {
    fn pkg_key(&self) -> &PkgKey {
        match self {
            CacheInner::Loaded(load) => &load.info.pkg,
            CacheInner::Unloaded(unload) => &unload.pkg,
            CacheInner::BeingCached(pk, _) => pk,
        }
    }

    fn kind(&self) -> (&'static str, Style) {
        match self {
            CacheInner::Loaded(_) => ("[Loaded]", Style::new()),
            CacheInner::Unloaded(_) => (
                "[Cached]",
                Style {
                    fg: Some(Color::DarkGray),
                    ..Style::new()
                },
            ),
            CacheInner::BeingCached(_, _) => (
                "[HoldOn]",
                Style {
                    fg: Some(Color::LightMagenta),
                    ..Style::new()
                },
            ),
        }
    }
}

impl Eq for CacheInner {}
impl PartialEq for CacheInner {
    /// Only use PkgKey to compare if both are equal.
    fn eq(&self, other: &Self) -> bool {
        self.pkg_key() == other.pkg_key()
    }
}
