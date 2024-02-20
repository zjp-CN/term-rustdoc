mod cache_info;
mod meta;
mod pkg_key;
mod util;

use self::meta::DocMeta;
use crate::{err, local_registry::PkgInfo, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use term_rustdoc::util::XString;

pub use self::{cache_info::CachedDocInfo, pkg_key::PkgKey};

type Progress = Arc<Mutex<Vec<CachedDocInfo>>>;

#[derive(Default)]
pub struct DataBase {
    /// [`dirs::data_local_dir`] + `term-rustdoc` folder
    ///
    /// `Some` means the folder does exist.
    ///
    /// `None` means
    /// * can't find config_local_dir
    /// * or the term-rustdoc folder is checked to be created
    dir: Option<PathBuf>,
    /// The pkg which doc is compiled and written into its db file.
    in_progress: Progress,
}

impl DataBase {
    pub fn init() -> Result<Self> {
        let mut dir =
            dirs::data_local_dir().ok_or_else(|| err!("Can't find the config_local_dir"))?;
        dir.push("term-rustdoc");
        if !dir.exists() {
            fs::create_dir(&dir)?;
        }
        Ok(DataBase {
            dir: Some(dir),
            ..Default::default()
        })
    }

    pub fn compile_doc(&self, pkg_dir: PathBuf, pkg_info: PkgInfo) -> Option<PkgKey> {
        let Some(parent) = self.dir.as_ref() else {
            error!("data_local_dir/term_rustdoc does not exist");
            return None;
        };
        Some(util::build(
            self.in_progress.clone(),
            parent.to_owned(),
            pkg_dir,
            pkg_info,
        ))
    }

    pub fn all_caches(&self) -> Result<Vec<CachedDocInfo>> {
        use redb::ReadableTable;
        let dir = self
            .dir
            .as_deref()
            .ok_or_else(|| err!("Can't fetch all caches because the dir path is not set up"))?;
        let db = redb::Database::open(dir.join("index.db"))
            .map_err(|err| err!("Can't open index.db:\n{err}"))?;
        let table = redb::TableDefinition::<PkgKey, CachedDocInfo>::new("CachedDocInfo");
        let read_txn = db.begin_read()?;
        let info: Vec<CachedDocInfo> = read_txn
            .open_table(table)?
            .iter()?
            .filter_map(|res| match res {
                Ok((_, v)) => Some(v.value()),
                Err(err) => {
                    error!("Failed to read a key-value pair in index.db:\n{err}");
                    None
                }
            })
            .collect();
        info!("Succeefully read {} CachedDocInfo", info.len());
        Ok(info)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum Features {
    #[default]
    Default,
    All,
    DefaultPlus(Box<[XString]>),
    NoDefault,
    NoDefaultPlus(Box<[XString]>),
}
