#[macro_use]
mod impls;
// The inner macro `icon!` can be used afterwards in submods

mod id;
mod nodes;
mod stats;
mod tag;
mod textline;

use rustdoc_types::Crate;
use std::{fmt, ops::Deref, rc::Rc};

pub use id::{IDMap, IDs, IdAsStr, IdToID, IndexMap, PathMap, SliceToIds, ID};
pub use impls::show::{DocTree, Show};
pub use nodes::{
    DConstant, DEnum, DFunction, DImpl, DMacroAttr, DMacroDecl, DMacroDerv, DMacroFunc, DModule,
    DStatic, DStruct, DTrait, DTypeAlias, DUnion,
};
pub use stats::{ImplCount, ImplCounts, ImplKind, ItemCount};
pub use tag::Tag;
pub use textline::{Text, TextTag, TreeLine, TreeLines};

/// This should be the main data structure to refer to documentation
/// and the items tree structure in public modules.
///
/// It's cheap to clone and use a ID buffer to avoid the cost of generating a new string in query.
#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CrateDoc {
    inner: Rc<IDMap>,
}

impl CrateDoc {
    pub fn new(doc: Crate) -> CrateDoc {
        CrateDoc {
            inner: Rc::new(IDMap::new(doc)),
        }
    }
}

impl Deref for CrateDoc {
    type Target = IDMap;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl fmt::Debug for CrateDoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CrateDoc for {}", self.name(&self.dmodule().id))
    }
}
