use crate::Result;
use itertools::Itertools;
use regex::Regex;
use semver::Version;
use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};
use term_rustdoc::util::XString;

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

fn find_pkgs(registry_src: &Path) -> Vec<PkgNameVersion> {
    match fs::read_dir(registry_src) {
        Ok(entries) => {
            entries
                .filter_map(|entry| {
                    let dir = entry.ok()?;
                    if dir.file_type().ok()?.is_dir() {
                        let mut pkg_path = dir.path();
                        // check if the pkg contains Cargo.toml in its root
                        pkg_path.push("Cargo.toml");
                        if pkg_path.exists() {
                            pkg_path.pop();
                            return PkgNameVersion::new(pkg_path);
                        }
                    }
                    None
                })
                .collect()
        }
        Err(err) => {
            error!("Failed to read `{}` dir:\n{err}", registry_src.display());
            Vec::new()
        }
    }
}

pub fn all_pkgs_in_latest_registry(registry_src: &Path) -> Vec<PkgNameVersion> {
    let mut pkgs = find_pkgs(registry_src);
    pkgs.sort_unstable_by(|a, b| (&*a.name, &a.version).cmp(&(&*b.name, &b.version)));
    pkgs.shrink_to_fit();
    pkgs
}

#[derive(Debug, Default)]
pub struct LocalRegistry {
    pkgs: Vec<PkgNameVersion>,
    path: PathBuf,
}

impl std::ops::Deref for LocalRegistry {
    type Target = [PkgNameVersion];
    fn deref(&self) -> &Self::Target {
        &self.pkgs
    }
}

impl LocalRegistry {
    pub fn all_pkgs_in_latest_registry() -> Result<Self> {
        let Some(path) = latest_registry()? else {
            return Ok(Self::default());
        };
        let pkgs = all_pkgs_in_latest_registry(&path);
        Ok(LocalRegistry { pkgs, path })
    }

    pub fn lastest_pkgs_in_latest_registry() -> Result<Self> {
        let Some(path) = latest_registry()? else {
            return Ok(Self::default());
        };
        let pkgs = lastest_pkgs_in_latest_registry(&path);
        Ok(LocalRegistry { pkgs, path })
    }

    pub fn len(&self) -> usize {
        self.pkgs.len()
    }

    pub fn registry_src_path(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PkgNameVersion {
    /// Pkg name from dir name.
    name: XString,
    /// Pkg version from dir name.
    ver_str: XString,
    /// Pkg version parsed from ver_str.
    version: Version,
    /// The pkg dir path not including Cargo.toml.
    path: PathBuf,
    /// The last modified time for pkg dir.
    modified: SystemTime,
}

impl Default for PkgNameVersion {
    fn default() -> Self {
        let (name, ver_str, path) = Default::default();
        PkgNameVersion {
            name,
            ver_str,
            version: Version::new(0, 0, 0),
            path,
            modified: SystemTime::now(),
        }
    }
}

impl PkgNameVersion {
    fn new(pkg_path: PathBuf) -> Option<Self> {
        let modified = pkg_path.metadata().ok()?.modified().ok()?;
        let (name, ver, version) = get_pkg_name(pkg_path.file_name()?.to_str()?)?;
        Some(PkgNameVersion {
            name,
            ver_str: ver,
            version,
            path: pkg_path,
            modified,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ver(&self) -> &str {
        &self.ver_str
    }
}

thread_local! {
    static RE: Regex = Regex::new(r"-\d+\.\d+\.\d+.*?$").unwrap();
}

fn get_pkg_name(name_ver: &str) -> Option<(XString, XString, Version)> {
    RE.with(|re| {
        re.find(name_ver).and_then(|m| {
            let start = m.start();
            let ver = &name_ver[start + 1..m.end()];
            ver.parse()
                .ok()
                .map(|version| (name_ver[..start].into(), ver.into(), version))
        })
    })
}

/// Pkgs with lastest version.
pub fn lastest_pkgs_in_latest_registry(registry_src: &Path) -> Vec<PkgNameVersion> {
    let mut pkgs: Vec<_> = all_pkgs_in_latest_registry(registry_src)
        .into_iter()
        .group_by(|pkg| pkg.name.clone())
        .into_iter()
        .map(|(_, pkg)| {
            pkg.into_iter()
                .max_by(|a, b| a.version.cmp(&b.version))
                .unwrap()
        })
        .collect();
    pkgs.shrink_to_fit();
    pkgs
}

#[test]
fn local_registry_pkgs() -> Result<()> {
    let registry_src = latest_registry()?.unwrap();
    // let paths = all_pkgs_in_latest_registry(&registry_src);
    // dbg!(&paths);
    let pkgs = lastest_pkgs_in_latest_registry(&registry_src);
    dbg!(pkgs.iter().map(|p| &p.path).collect::<Vec<_>>());
    Ok(())
}
