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
    inner: Rc<IDMap>,
}

impl CrateDoc {
    pub fn new(doc: Crate) -> CrateDoc {
        CrateDoc {
            inner: Rc::new(IDMap::new(doc)),
        }
    }

    pub fn doc(&self) -> &Crate {
        self.inner.doc()
    }

    pub fn dmodule(&self) -> &DModule {
        self.inner.dmodule()
    }

    pub fn idmap(&self) -> &IDMap {
        &self.inner
    }

    pub fn dmodule_show_prettier(&self) -> DocTree {
        self.dmodule().show_prettier(&self.inner)
    }
}
