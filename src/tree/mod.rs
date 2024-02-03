#[macro_use]
mod impls;
// The inner macro `icon!` can be used afterwards in submods

#[allow(non_snake_case)]
mod id;
mod nodes;
mod stats;
mod tag;
mod textline;

use rustdoc_types::Crate;
use std::rc::Rc;

pub use id::{IDMap, IDs, IdAsStr, IdToID, IndexMap, PathMap, SliceToIds, ID};
pub use impls::show::{DocTree, Show};
pub use nodes::{
    DConstant, DEnum, DFunction, DImpl, DMacroAttr, DMacroDecl, DMacroDerv, DMacroFunc, DModule,
    DStatic, DStruct, DTrait, DTypeAlias, DUnion,
};
pub use stats::{ImplCount, ImplCounts, ImplKind, ItemCount};
pub use tag::Tag;
pub use textline::{Text, TextTag, TreeLine, TreeLines};

#[derive(Clone, Default)]
pub struct CrateDoc {
    doc: Rc<Inner>,
}

struct Inner {
    krate: Crate,
    dmod: DModule,
}

impl CrateDoc {
    pub fn new(doc: Crate) -> CrateDoc {
        let dmod = DModule::new(&doc);
        CrateDoc {
            doc: Rc::new(Inner { krate: doc, dmod }),
        }
    }

    pub fn dmodule(&self) -> &DModule {
        &self.doc.dmod
    }

    pub fn idmap(&self) -> IDMap {
        IDMap::new(&self.doc.krate)
    }

    pub fn doc(&self) -> &Crate {
        &self.doc.krate
    }
}

impl Default for Inner {
    fn default() -> Self {
        let (crate_version, includes_private, index, paths, external_crates, format_version) =
            Default::default();
        Inner {
            krate: Crate {
                root: rustdoc_types::Id(String::new()),
                crate_version,
                includes_private,
                index,
                paths,
                external_crates,
                format_version,
            },
            dmod: DModule::default(),
        }
    }
}
