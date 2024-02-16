use crate::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn registry_path() -> Result<Option<PathBuf>> {
    let mut cargo = home::cargo_home()?;
    cargo.extend(["registry", "src"]);
    let mut entries = Vec::new();
    for entry in fs::read_dir(&cargo)? {
        let entry = entry?;
        entries.push((entry.metadata()?.modified()?, entry.path()));
    }
    entries.sort_unstable_by_key(|v| v.0);
    // choose the lastest modified
    Ok(entries.pop().map(|v| v.1))
}

fn tokio_path() -> Result<PathBuf> {
    let mut path = registry_path()?.unwrap();
    let filename = fs::read_dir(&path)?
        .filter_map(|entry| {
            let file_name = &entry.ok()?.file_name();
            let filename = file_name.to_str()?;
            if filename.starts_with("tokio-1.") {
                Some(filename.to_owned())
            } else {
                None
            }
        })
        .max()
        .unwrap();
    path.extend([filename.as_str(), "Cargo.toml"]);
    Ok(path)
}

#[test]
fn checkout_tokio_path() -> Result<()> {
    let path = tokio_path()?;
    dbg!(path);
    Ok(())
}

#[test]
#[ignore = "replace the manifest_path in your machine"]
fn generate_tokio_json_doc() -> Result<()> {
    let dir = PathBuf::from_iter(["target", "deps"]);
    fs::create_dir_all(&dir)?;
    // like /root/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/tokio-1.35.1/Cargo.toml
    let manifest_path = tokio_path()?;
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
