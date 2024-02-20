use super::Progress;
use crate::{dashboard::database::cache_info::CachedDocInfo, local_registry::PkgInfo, Result};
use bincode::config;
use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;

pub fn build(progress: Progress, db_dir: PathBuf, pkg_dir: PathBuf, pkg_info: PkgInfo) {
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

pub fn encode<T: Serialize>(t: T) -> Result<Vec<u8>> {
    Ok(bincode::serde::encode_to_vec(t, config::standard())?)
}

pub fn decode<T: DeserializeOwned>(v: &[u8]) -> Result<T> {
    Ok(bincode::serde::decode_from_slice(v, config::standard())?.0)
}
