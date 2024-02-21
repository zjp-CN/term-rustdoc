use crate::{
    dashboard::database::{CachedDocInfo, PkgKey},
    ui::LineState,
};
use core::cmp::Ordering;
use ratatui::prelude::{Color, Style};
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

    pub fn is_in_progress(&self, key: &PkgKey) -> bool {
        matches!(&self.inner, CacheInner::BeingCached(pkg) if pkg == key)
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
            CacheInner::BeingCached(_) => (
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
