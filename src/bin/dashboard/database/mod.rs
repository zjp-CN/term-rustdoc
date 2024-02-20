mod pkg_key;

use self::pkg_key::PkgKey;
use crate::{
    err,
    local_registry::{PkgInfo, PkgNameVersion},
    Result,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::SystemTime,
};
use term_rustdoc::{tree::CrateDoc, util::XString};

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
    /// loaded pkg docs
    loaded: Vec<PackageDoc>,
    /// all cached docs (not loaded)
    cached: Vec<CachedDocInfo>,
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

    pub fn compile_doc(&mut self, pkg_dir: PathBuf, name_ver: PkgNameVersion) {
        let Some(parent) = self.dir.as_ref() else {
            return;
        };
        let info = PkgKey::new(name_ver).into_cached_info(parent);
        PackageDoc::build(self.in_progress.clone(), pkg_dir, info);
    }
}

#[derive(Deserialize, Serialize)]
struct CachedDocInfo {
    pkg: PkgKey,
    /// file name for doc db (with parent path included); usually is `self.pkg-self.ver.db`.
    db_file: PathBuf,
    /// the time when the doc is compiled and generated
    created: SystemTime,
}

impl CachedDocInfo {
    fn save_doc(&self, doc: &CrateDoc) -> Result<()> {
        let db = redb::Database::create(&self.db_file)?;
        let table = redb::TableDefinition::<PkgKey, Vec<u8>>::new("host");
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(table)?;
            let doc = bincode::serde::encode_to_vec(doc, bincode::config::standard())?;
            table.insert(&self.pkg, &doc)?;
        }
        write_txn.commit()?;
        info!(?self.pkg, "succeefully saved");
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
struct PackageDoc {
    /// source pkg:
    /// * the path direct to pkg dir under local registry_src
    /// * the modified time is for pkg dir
    src: PkgInfo,
    doc: CrateDoc,
    meta: DocMeta,
}

impl PackageDoc {
    pub fn build(progress: Progress, pkg_dir: PathBuf, info: CachedDocInfo) {
        let mut cargo_toml = pkg_dir;
        cargo_toml.push("Cargo.toml");
        rayon::spawn(move || {
            let dir = match tempfile::tempdir() {
                Ok(dir) => dir,
                Err(err) => {
                    error!("Can't create a tempdir:\n{err}");
                    return;
                }
            };
            info!(?info.pkg, "begin to compile the doc under {}", dir.path().display());
            match rustdoc_json::Builder::default()
                .toolchain("nightly")
                .quiet(true)
                .target_dir(&dir)
                .manifest_path(&cargo_toml)
                .build()
            {
                Ok(json_path) => {
                    info!(?info.pkg, ?json_path, "succeefully compiled the doc");
                    if let Err(err) = save_doc(&json_path, &info) {
                        error!("{err}");
                    }
                    match progress.lock() {
                        Ok(mut v) => v.push(info),
                        Err(err) => {
                            error!(
                                "Failed to lock the progress to write generated PkgKey.\
                                 The doc is generated though.\n{err}"
                            )
                        }
                    }
                }
                Err(err) => error!("Failed to compile {}:\n{err}", cargo_toml.display()),
            }
        });
    }
}

fn save_doc(json_path: &Path, info: &CachedDocInfo) -> Result<()> {
    let file = fs::File::open(json_path).map_err(|err| {
        err!(
            "Failed to open compiled json doc under {}:\n{err}",
            json_path.display()
        )
    })?;
    let doc = CrateDoc::new(serde_json::from_reader(file)?);
    info.save_doc(&doc)?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
struct DocMeta {
    /// the rustc/rustdoc version compiling the doc, gotten by `rustc -Vv`
    rustc_version: String,
    /// the host field from `rustc_version`
    host_triple: XString,
    /// TODO: the target platform. we haven't supported this other than host triple,
    /// so usually this equals to host_triple.
    target_triple: XString,
    // /// For now, each doc is generated on local machine.
    // /// TODO:
    // /// But for the future, we can support save and load docs non-locally generated.
    // /// For example, crates.io or docs.rs or somthing can provide compiled docs, so
    // /// we don't need to compile them locally. Or if you migrate/duplicate docs from
    // /// one machine to another machine.
    // is_local: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[allow(dead_code)]
enum Features {
    #[default]
    Default,
    NoDefault,
    All,
    DefaultPlus(Box<[XString]>),
    NoDefaultPlus(Box<[XString]>),
}
