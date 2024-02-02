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

#[derive(Clone)]
pub struct CrateDoc {
    pub doc: Rc<Crate>,
}

impl CrateDoc {
    pub fn new(doc: Crate) -> CrateDoc {
        CrateDoc { doc: Rc::new(doc) }
    }

    pub fn dmodule_idmap(&self) -> (DModule, IDMap<'_>) {
        (DModule::new(&self.doc), IDMap::new(&self.doc))
    }

    pub fn idmap(&self) -> IDMap {
        IDMap::new(&self.doc)
    }
}

impl Default for CrateDoc {
    fn default() -> Self {
        let (crate_version, includes_private, index, paths, external_crates, format_version) =
            Default::default();
        CrateDoc {
            doc: Rc::new(Crate {
                root: rustdoc_types::Id(String::new()),
                crate_version,
                includes_private,
                index,
                paths,
                external_crates,
                format_version,
            }),
        }
    }
}
