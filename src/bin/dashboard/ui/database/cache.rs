mod inner;
mod util;

use self::inner::CacheInner;
use crate::color::{PKG_FEATURES, PKG_NAME, PKG_VERSION};
use crate::database::{CachedDocInfo, DataBase, Features, PkgKey};
use ratatui::prelude::Style;
use semver::Version;
use std::time::SystemTime;
use std::{cmp::Ordering, mem};
use term_rustdoc::{
    tree::CrateDoc,
    util::{xformat, XString},
};

pub use self::util::{CacheID, Count, LoadedDoc, SortKind};

#[derive(PartialEq, Eq)]
pub struct Cache {
    inner: CacheInner,
    features: XString,
    ver: Version,
}

impl Cache {
    pub fn new_being_cached(pkg_key: PkgKey) -> Cache {
        Cache {
            ver: pkg_key.version(),
            features: xformat!("{:?}", pkg_key.features()),
            inner: CacheInner::BeingCached(pkg_key, SystemTime::now()),
        }
    }

    pub fn new_unloaded(info: CachedDocInfo) -> Cache {
        Cache {
            ver: info.pkg.version(),
            features: xformat!("{:?}", info.pkg.features()),
            inner: CacheInner::Unloaded(info),
        }
    }

    pub fn pkg_feat(&self) -> (&str, &str, &Features) {
        let pkg = self.inner.pkg_key();
        (pkg.name(), pkg.ver_str(), pkg.features())
    }

    pub fn is_in_progress(&self, key: &PkgKey) -> bool {
        matches!(&self.inner, CacheInner::BeingCached(pkg, _) if pkg == key)
    }

    pub fn loadable(&self) -> bool {
        matches!(self.inner, CacheInner::Unloaded(_) | CacheInner::Loaded(_))
    }

    /// An empty PkgKey placeholder for temporary use.
    /// Be aware to write old valid value back after replacement.
    fn empty_state() -> Cache {
        Cache {
            inner: CacheInner::BeingCached(PkgKey::empty_state(), SystemTime::now()),
            features: XString::const_new(""),
            ver: Version::new(0, 0, 0),
        }
    }

    pub fn load_doc(&mut self, db: &DataBase) {
        let mut old = mem::replace(self, Cache::empty_state());
        match old.inner {
            CacheInner::Unloaded(unloaded) => {
                old = match unloaded.load_doc() {
                    Ok(doc) => {
                        let key = Box::new(unloaded.pkg.clone());
                        if let Err(err) = db.send_doc(key) {
                            error!("Loaded Error:\n{err}");
                        }
                        Cache {
                            inner: CacheInner::Loaded(LoadedDoc {
                                info: unloaded,
                                doc,
                            }),
                            features: old.features,
                            ver: old.ver,
                        }
                    }
                    Err(err) => {
                        error!(
                            "Failed to load {:?}, but trying to recompile the doc:\n{err}",
                            unloaded.pkg
                        );
                        match unloaded.load_pkg_info_features() {
                            Ok(pkg) => match db.compile_doc(pkg) {
                                Some(key) => {
                                    // FIXME: use UI to notify the doc is being recompiled,
                                    // for now we only see a HOLDON icon after clicking CACHE.
                                    info!("Recompiling the doc for {key:?}!");
                                    *self = Cache::new_being_cached(key);
                                    return;
                                }
                                None => error!("Failed to recompile the doc"),
                            },
                            Err(err) => error!("Failed to load info:\n{err}"),
                        }
                        Cache {
                            inner: CacheInner::Unloaded(unloaded),
                            features: old.features,
                            ver: old.ver,
                        }
                    }
                }
            }
            CacheInner::Loaded(loaded) => {
                let key = Box::new(loaded.info.pkg.clone());
                if let Err(err) = db.send_doc(key) {
                    error!("Loaded Error:\n{err}");
                }
                old = Cache {
                    inner: CacheInner::Loaded(loaded),
                    features: old.features,
                    ver: old.ver,
                };
            }
            _ => (),
        }
        *self = old;
    }

    pub fn get_loaded_doc(&self, key: &PkgKey) -> Option<CrateDoc> {
        match &self.inner {
            CacheInner::Loaded(loaded) if loaded.info.pkg == *key => Some(loaded.doc.clone()),
            _ => None,
        }
    }

    pub fn downgrade(&mut self) -> Option<Box<PkgKey>> {
        let mut key = None;
        let mut old = mem::replace(self, Cache::empty_state());
        if let CacheInner::Loaded(loaded) = old.inner {
            key = Some(Box::new(loaded.info.pkg.clone()));
            info!("Downgrade a loaded {:?} into cached one.", loaded.info.pkg);
            old = Cache::new_unloaded(loaded.info)
        };
        *self = old;
        key
    }

    pub fn line(&self) -> [(&str, Style); 4] {
        let kind = self.inner.kind();
        let key = self.inner.pkg_key();
        [
            kind,
            (key.name(), PKG_NAME),
            (key.ver_str(), PKG_VERSION),
            (&self.features, PKG_FEATURES),
        ]
    }

    fn pkg_key(&self) -> &PkgKey {
        self.inner.pkg_key()
    }

    pub fn name(&self) -> &str {
        self.pkg_key().name()
    }

    pub fn add(&self, count: &mut Count) {
        match &self.inner {
            CacheInner::Loaded(_) => count.loaded += 1,
            CacheInner::Unloaded(_) => count.unloaded += 1,
            CacheInner::BeingCached(_, _) => count.in_progress += 1,
        }
    }
}

impl Cache {
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
