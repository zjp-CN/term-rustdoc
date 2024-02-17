use crate::Result;
use itertools::Itertools;
use regex::Regex;
use semver::Version;
use std::{
    fs,
    path::{Component, Path, PathBuf},
};
use term_rustdoc::util::XString;
use walkdir::WalkDir;

fn latest_registry() -> Result<Option<PathBuf>> {
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

pub fn all_pkgs_in_latest_registry(registry_src: &Path) -> Result<Vec<PkgNameVersion>> {
    let mut cargo_tomls: Vec<_> = WalkDir::new(registry_src)
        .max_depth(2)
        .into_iter()
        .filter_map(|f| {
            f.ok().and_then(|f| {
                (f.file_name() == "Cargo.toml")
                    .then(|| PkgNameVersion::new(f.into_path()))
                    .flatten()
            })
        })
        .collect();
    cargo_tomls.sort_unstable_by(|a, b| (&*a.name, &a.version).cmp(&(&*b.name, &b.version)));
    cargo_tomls.shrink_to_fit();
    Ok(cargo_tomls)
}

fn pkg_name_version(path: &Path) -> Option<Component<'_>> {
    // Since the 2nd depth directs to Cargo.toml,
    // the last but one is considered to be pkg name and version.
    // i.e. <registry_src>/pkg_name_version/Cargo.toml
    // ~/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/regex-1.10.3/Cargo.toml
    path.components().rev().nth(1)
}

#[derive(Debug)]
pub struct PkgNameVersion {
    name: XString,
    version: Version,
    path: PathBuf,
}

impl PkgNameVersion {
    fn new(path: PathBuf) -> Option<Self> {
        if let Some(c) = pkg_name_version(&path) {
            let s = c.as_os_str().to_str()?;
            let (name, version) = get_pkg_name(s)?;
            Some(PkgNameVersion {
                name,
                version,
                path,
            })
        } else {
            None
        }
    }
}

thread_local! {
    static RE: Regex = Regex::new(r"-\d+\.\d+\.\d+.*?$").unwrap();
}

fn get_pkg_name(name_ver: &str) -> Option<(XString, Version)> {
    RE.with(|re| {
        re.find(name_ver).and_then(|m| {
            let start = m.start();
            name_ver[start + 1..m.end()]
                .parse()
                .ok()
                .map(|version| (name_ver[..start].into(), version))
        })
    })
}

pub fn lastest_pkgs_in_latest_registry(registry_src: &Path) -> Result<Vec<PkgNameVersion>> {
    let mut paths: Vec<_> = all_pkgs_in_latest_registry(registry_src)?
        .into_iter()
        .group_by(|pkg| pkg.name.clone())
        .into_iter()
        .map(|(_, pkg)| {
            pkg.into_iter()
                .max_by(|a, b| a.version.cmp(&b.version))
                .unwrap()
        })
        .collect();
    paths.shrink_to_fit();
    Ok(paths)
}

#[test]
fn local_registry_pkgs() -> Result<()> {
    let registry_src = latest_registry()?.unwrap();
    // let paths = all_pkgs_in_latest_registry(&registry_src)?;
    // dbg!(&paths);
    let pkgs = lastest_pkgs_in_latest_registry(&registry_src)?;
    dbg!(pkgs.iter().map(|p| &p.path).collect::<Vec<_>>());
    Ok(())
}
