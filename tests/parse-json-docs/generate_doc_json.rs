use crate::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[test]
#[ignore = "replace the manifest_path in your machine"]
fn generate_tokio_json_doc() -> Result<()> {
    let dir = PathBuf::from_iter(["target", "deps"]);
    fs::create_dir_all(&dir)?;
    let manifest_path =
        "/root/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/tokio-1.35.1/Cargo.toml";
    generate(&dir, manifest_path)?;
    generate(
        &dir,
        PathBuf::from_iter(["tests", "integration", "Cargo.toml"]),
    )?;
    Ok(())
}

fn generate<P, Q>(dir: P, manifest_path: Q) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    rustdoc_json::Builder::default()
        .toolchain("nightly")
        .target_dir(dir.as_ref())
        .all_features(true)
        .manifest_path(manifest_path.as_ref())
        .build()?;
    Ok(())
}
