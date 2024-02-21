use super::PkgKey;
use crate::{
    database::CachedDocInfo,
    event::{Event, Sender},
    local_registry::PkgInfo,
    Result,
};
use bincode::config;
use bytesize::ByteSize;
use serde::{de::DeserializeOwned, Serialize};
use std::{io::Write, path::PathBuf};
use xz2::write::{XzDecoder, XzEncoder};

pub fn build(sender: Sender, db_dir: PathBuf, pkg_dir: PathBuf, pkg_info: PkgInfo) -> PkgKey {
    let mut cargo_toml = pkg_dir;
    cargo_toml.push("Cargo.toml");
    let in_progress = PkgKey::new_with_default_feature(pkg_info.to_name_ver());
    rayon::spawn(move || {
        let dir = match tempfile::tempdir() {
            Ok(dir) => dir,
            Err(err) => {
                error!("Can't create a tempdir:\n{err}");
                return;
            }
        };
        let mut cache_info =
            CachedDocInfo::new_with_default_feature(pkg_info.to_name_ver(), db_dir);
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
                match sender.send(Event::DocCompiled(Box::new(cache_info))) {
                    Ok(()) => (),
                    Err(err) => {
                        error!(
                            "Failed to send `DocCompiled` event when CachedDocInfo is ready:\n{err}"
                        )
                    }
                }
            }
            Err(err) => error!("Failed to compile {}:\n{err}", cargo_toml.display()),
        }
    });
    in_progress
}

/// Write source data into db file.
pub fn encode<T: Serialize>(t: T) -> Result<Vec<u8>> {
    Ok(bincode::serde::encode_to_vec(t, config::standard())?)
}

/// Read data from db file.
pub fn decode<T: DeserializeOwned>(raw: &[u8]) -> Result<T> {
    Ok(bincode::serde::decode_from_slice(raw, config::standard())?.0)
}

/// Write source data into db file with xz compression.
///
/// NOTE: not all bytes are compressed via xz, because compression on small bytes
/// will increase the size.
/// For now, only compress the doc data, i.e. host-json and host-parsed table.
/// Be careful to decompress them before deserialization.
pub fn encode_with_xz<T: Serialize>(t: T) -> Result<Vec<u8>> {
    let raw = bincode::serde::encode_to_vec(t, config::standard())?;
    xz_encode_on_bytes(&raw)
}

pub fn xz_encode_on_bytes(raw: &[u8]) -> Result<Vec<u8>> {
    let mut compressed = Vec::with_capacity(raw.len() / 2);
    let mut xz_encoder = XzEncoder::new(&mut compressed, 9);
    xz_encoder.write_all(raw)?;
    xz_encoder.finish()?;
    compressed.shrink_to_fit();
    let (before, after) = (raw.len(), compressed.len());
    info!(
        "compress {} => {} (reduced {:.1}%)",
        ByteSize(before as u64),
        ByteSize(after as u64),
        (after as f32 / before as f32 - 1.0) * 100.0
    );
    Ok(compressed)
}

/// Read data from db file with xz decompression.
pub fn decode_with_xz<T: DeserializeOwned>(raw: &[u8]) -> Result<T> {
    let decompressed = xz_decode_on_bytes(raw)?;
    Ok(bincode::serde::decode_from_slice(&decompressed, config::standard())?.0)
}

pub fn xz_decode_on_bytes(raw: &[u8]) -> Result<Vec<u8>> {
    let mut decompressed = Vec::with_capacity(raw.len() * 4);
    {
        let mut xz_decoder = XzDecoder::new(&mut decompressed);
        xz_decoder.write_all(raw)?;
        xz_decoder.finish()?;
    }
    let (before, after) = (raw.len(), decompressed.len());
    info!(
        "decompress {} => {} (ratio {:.1}%)",
        ByteSize(before as u64),
        ByteSize(after as u64),
        (1.0 - before as f32 / after as f32) * 100.0
    );
    Ok(decompressed)
}
