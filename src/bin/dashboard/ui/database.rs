mod cache;

use self::cache::{Cache, CacheID};
use crate::{dashboard::database::DataBase, ui::Scrollable};

#[derive(Default)]
struct PkgDocs {
    db: DataBase,
    caches: Vec<Cache>,
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
            ui.inner.lines.db = db;
        }
        ui
    }
}
