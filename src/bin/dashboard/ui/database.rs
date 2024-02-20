mod cache;

use self::cache::{Cache, CacheID};
use crate::{dashboard::database::DataBase, local_registry::PkgInfo, ui::Scrollable};
use std::path::PathBuf;

#[derive(Default)]
struct PkgDocs {
    db: DataBase,
    caches: Vec<Cache>,
    /// NOTE: the indices only change when the length of caches changes,
    /// because we need to sort caches for displaying, thus both lengths should equal.
    indices: Vec<CacheID>,
}

impl std::ops::Deref for PkgDocs {
    type Target = [CacheID];

    fn deref(&self) -> &Self::Target {
        &self.indices
    }
}

#[derive(Default)]
pub struct DataBaseUI {
    inner: Scrollable<PkgDocs>,
}

impl DataBaseUI {
    pub fn init() -> Self {
        let mut ui = DataBaseUI::default();
        if let Ok(db) = DataBase::init() {
            ui.pkg_docs().db = db;
        }
        ui
    }

    fn pkg_docs(&mut self) -> &mut PkgDocs {
        &mut self.inner.lines
    }

    pub fn compile_doc(&mut self, pkg_dir: PathBuf, pkg_info: PkgInfo) {
        if let Some(pkg_key) = self.pkg_docs().db.compile_doc(pkg_dir, pkg_info) {
            let caches = &mut self.pkg_docs().caches;
            let id = CacheID(caches.len());
            caches.push(Cache::new_being_cached(pkg_key));
            self.pkg_docs().indices.push(id);
            self.sort_caches();
        }
    }

    /// Sort the Cache vec because the inner states have changed.
    fn sort_caches(&mut self) {
        self.pkg_docs().caches.sort_unstable();
    }
}
