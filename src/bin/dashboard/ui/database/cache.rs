use crate::{
    dashboard::database::{CachedDocInfo, PkgKey},
    ui::LineState,
};
use semver::Version;
use term_rustdoc::tree::CrateDoc;

pub struct LoadedDoc {
    info: CachedDocInfo,
    doc: CrateDoc,
}

pub struct CacheID(usize);

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

impl Cache {}

impl PartialOrd for Cache {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cache {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let name = self.inner.pkg_key().name();
        match name.cmp(other.inner.pkg_key().name()) {
            core::cmp::Ordering::Equal => self.ver.cmp(&other.ver),
            ord => ord,
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
