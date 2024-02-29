mod cache_info;
mod features;
mod meta;
mod pkg_key;
mod util;

use self::meta::DocMeta;
use crate::{
    err,
    event::{Event, Sender},
    Result,
};
use color_eyre::eyre::WrapErr;
use std::{fs, path::PathBuf};

pub use self::{
    cache_info::CachedDocInfo, features::FeaturesUI, pkg_key::PkgKey, util::PkgWithFeatures,
};

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
    /// When a pkg doc is compiled and written into its db file, use this to send an event to notify UI.
    sender: Option<Sender>,
}

impl DataBase {
    pub fn init(sender: Sender) -> Result<Self> {
        let mut dir =
            dirs::data_local_dir().ok_or_else(|| err!("Can't find the config_local_dir"))?;
        dir.push("term-rustdoc");
        if !dir.exists() {
            fs::create_dir(&dir)?;
        }
        Ok(DataBase {
            dir: Some(dir),
            sender: Some(sender),
        })
    }

    pub fn compile_doc(&self, pkg: PkgWithFeatures) -> Option<PkgKey> {
        let Some(parent) = self.dir.clone() else {
            error!("data_local_dir/term_rustdoc does not exist");
            return None;
        };
        let Some(sender) = self.sender.clone() else {
            error!("DataBase doesn't have a sender. This is a bug.");
            return None;
        };
        Some(util::build(sender, parent, pkg))
    }

    pub fn all_caches(&self) -> Result<Vec<CachedDocInfo>> {
        use redb::ReadableTable;
        let dir = self
            .dir
            .as_deref()
            .ok_or_else(|| err!("Can't fetch all caches because the dir path is not set up"))?;
        let db = redb::Database::create(dir.join("index.db"))
            .wrap_err_with(|| "Can't create index.db")?;
        let table = redb::TableDefinition::<PkgKey, CachedDocInfo>::new("CachedDocInfo");
        let read_txn = db.begin_read()?;
        let read_only_table = match read_txn.open_table(table) {
            Ok(tab) => tab,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(Vec::new()),
            err => err.wrap_err_with(|| "Can't read CachedDocInfo table from index.db")?,
        };
        let info: Vec<CachedDocInfo> = read_only_table
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

    pub fn send_doc(&self, key: Box<PkgKey>) -> Result<()> {
        if let Some(sender) = &self.sender {
            Ok(sender.send(Event::CrateDoc(key))?)
        } else {
            Err(err!(
                "DataBase doesn't have a sender to send loaded CrateDoc for {key:?}. This is a bug."
            ))
        }
    }

    pub fn send_downgraded_doc(&self, key: Box<PkgKey>) {
        if let Some(sender) = &self.sender {
            if let Err(err) = sender.send(Event::Downgraded(key)) {
                error!("Fail to send the downgraded pkg_key:\n{err}");
            }
        }
    }
}
