mod cache_info;
mod pkg_key;

use self::cache_info::CachedDocInfo;
use crate::{err, local_registry::PkgInfo, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
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
                let duration = meta.duration.as_secs_f32();
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

#[derive(Debug, Deserialize, Serialize)]
struct DocMeta {
    /// the rustc/rustdoc/cargo version compiling the doc, gotten by `cargo +nightly -Vv`
    /// NOTE: only nightly toolchain is supported for now
    cargo_version: String,
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
    /// the time when the doc starts to compile
    started: SystemTime,
    /// the time when the doc takes to be compiled and generated
    duration: Duration,
}

impl Default for DocMeta {
    fn default() -> Self {
        let started = SystemTime::now();
        let (cargo_version, host_triple, target_triple, duration) = Default::default();
        DocMeta {
            cargo_version,
            host_triple,
            target_triple,
            started,
            duration,
        }
    }
}

impl DocMeta {
    fn new() -> Self {
        match std::process::Command::new("cargo")
            .args(["+nightly", "-Vv"])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    let started = SystemTime::now();
                    let cargo_version = String::from_utf8_lossy(&output.stdout).into_owned();
                    let host_triple = cargo_version
                        .lines()
                        .find_map(|line| {
                            if line.starts_with("host: ") {
                                line.get(6..).map(XString::from)
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default();
                    let target_triple = host_triple.clone();
                    return DocMeta {
                        cargo_version,
                        host_triple,
                        target_triple,
                        started,
                        duration: Duration::default(),
                    };
                }
                let err = String::from_utf8_lossy(&output.stderr);
                error!("Failed to run `cargo +nightly -Vv` to get version and host_triple:\n{err}");
            }
            Err(err) => {
                error!("Failed to run `cargo +nightly -Vv` to get version and host_triple:\n{err}")
            }
        }
        DocMeta::default()
    }

    fn set_finished_duration(&mut self) {
        self.duration = self.started.elapsed().unwrap_or_default();
    }
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
