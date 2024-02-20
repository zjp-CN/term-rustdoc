use crate::{
    dashboard::database::{CachedDocInfo, PkgKey},
    ui::LineState,
};
use core::cmp::Ordering;
use semver::Version;
use term_rustdoc::tree::CrateDoc;

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
            inner: CacheInner::BeingCached(pkg_key),
        }
    }

    pub fn new_unloaded(info: CachedDocInfo) -> Cache {
        Cache {
            ver: info.pkg.version(),
            inner: CacheInner::Unloaded(info),
        }
    }
}

impl PartialOrd for Cache {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cache {
    /// Loaded is always before Unloaded, and Unloaded is always before BeingCached.
    /// When both are of the same kind, compare with name and parsed version.
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.inner, &other.inner) {
            (CacheInner::Loaded(a), CacheInner::Loaded(b)) => {
                match a.info.pkg.name().cmp(b.info.pkg.name()) {
                    Ordering::Equal => self.ver.cmp(&other.ver),
                    ord => ord,
                }
            }
            (CacheInner::Loaded(_), _) => Ordering::Less,
            (CacheInner::Unloaded(a), CacheInner::Unloaded(b)) => {
                match a.pkg.name().cmp(b.pkg.name()) {
                    Ordering::Equal => self.ver.cmp(&other.ver),
                    ord => ord,
                }
            }
            (CacheInner::Unloaded(_), CacheInner::BeingCached(_)) => Ordering::Less,
            (CacheInner::Unloaded(_), CacheInner::Loaded(_)) => Ordering::Greater,
            (CacheInner::BeingCached(a), CacheInner::BeingCached(b)) => {
                match a.name().cmp(b.name()) {
                    Ordering::Equal => self.ver.cmp(&other.ver),
                    ord => ord,
                }
            }
            (CacheInner::BeingCached(_), _) => Ordering::Greater,
        }
    }
}

pub enum CacheInner {
    /// cached & loaded pkg docs
    Loaded(LoadedDoc),
    /// cached but not loaded docs
    Unloaded(CachedDocInfo),
    /// pkgs which is being sent to compile doc
    BeingCached(PkgKey),
}

impl CacheInner {
    fn pkg_key(&self) -> &PkgKey {
        match self {
            CacheInner::Loaded(load) => &load.info.pkg,
            CacheInner::Unloaded(unload) => &unload.pkg,
            CacheInner::BeingCached(pk) => pk,
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
