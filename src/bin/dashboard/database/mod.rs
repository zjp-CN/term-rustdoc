mod cache_info;
mod meta;
mod pkg_key;

use self::{cache_info::CachedDocInfo, meta::DocMeta};
use crate::{err, local_registry::PkgInfo, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
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

    pub fn compile_doc(&self, pkg_dir: PathBuf, pkg_info: PkgInfo) {
        let Some(parent) = self.dir.as_ref() else {
            return;
        };
        build(
            self.in_progress.clone(),
            parent.to_owned(),
            pkg_dir,
            pkg_info,
        );
    }
}

fn build(progress: Progress, db_dir: PathBuf, pkg_dir: PathBuf, pkg_info: PkgInfo) {
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
        let mut cache_info = CachedDocInfo::new(pkg_info.to_name_ver(), db_dir);
        info!(?cache_info.pkg, "begin to compile the doc under {}", dir.path().display());
        match rustdoc_json::Builder::default()
            .toolchain("nightly")
            .silent(true)
            .target_dir(&dir)
            .manifest_path(&cargo_toml)
            .build()
        {
            Ok(json_path) => {
                let meta = cache_info.meta_mut();
                meta.set_finished_duration();
                let duration = meta.duration_as_secs();
                info!(?cache_info.pkg, ?json_path, "succeefully compiled the doc in {duration:.2}s");
                if let Err(err) = cache_info.save_doc(&json_path, pkg_info) {
                    error!("{err}");
                }
                match progress.lock() {
                    Ok(mut v) => v.push(cache_info),
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

fn encode<T: Serialize>(t: T) -> Result<Vec<u8>> {
    Ok(bincode::serde::encode_to_vec(
        t,
        bincode::config::standard(),
    )?)
}

fn decode<T: DeserializeOwned>(v: &[u8]) -> Result<T> {
    Ok(bincode::serde::decode_from_slice(v, bincode::config::standard())?.0)
}

#[derive(Deserialize, Serialize)]
struct PackageDoc {
    /// source pkg:
    /// * the path direct to pkg dir under local registry_src
    /// * the modified time is for pkg dir
    src: PkgInfo,
    doc: CrateDoc,
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
