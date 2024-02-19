use crate::local_registry::PkgNameVersion;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use term_rustdoc::{tree::CrateDoc, util::XString};

pub struct DataBase {
    /// [`dirs::config_local_dir`] + `term-rustdoc` folder
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
}

#[derive(Deserialize, Serialize)]
struct CachedDocInfo {
    /// cached pkg doc: especially
    /// * the path direct to db file storing the doc rather than pkg dir
    /// * the modified time is when the doc is compiled and generated
    pkg: PkgNameVersion,
    /// file name for doc db (with parent path excluded); usually is `self.pkg-self.ver.db`.
    db_file: XString,
}

#[derive(Deserialize, Serialize)]
struct PackageDoc {
    /// source pkg:
    /// * the path direct to pkg dir under local registry_src
    /// * the modified time is for pkg dir
    src: PkgNameVersion,
    doc: CrateDoc,
    meta: DocMeta,
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
    /// features enabled/used when the doc is compiled
    /// TODO: for now, we haven't supported feature selection.
    features: Features,
    // /// For now, each doc is generated on local machine.
    // /// TODO:
    // /// But for the future, we can support save and load docs non-locally generated.
    // /// For example, crates.io or docs.rs or somthing can provide compiled docs, so
    // /// we don't need to compile them locally. Or if you migrate/duplicate docs from
    // /// one machine to another machine.
    // is_local: bool,
}

#[derive(Default, Deserialize, Serialize)]
#[allow(dead_code)]
enum Features {
    #[default]
    Default,
    NoDefault,
    All,
    DefaultPlus(Box<[XString]>),
    NoDefaultPlus(Box<[XString]>),
}
