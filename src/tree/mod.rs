#[macro_use]
mod impls;
// The inner macro `icon!` can be used afterwards in submods

#[allow(non_snake_case)]
mod id;
mod nodes;
mod stats;

pub use id::{IDMap, IDs, IdAsStr, IdToID, IndexMap, PathMap, SliceToIds, ID};
pub use impls::show::{DocTree, Show, Tag, TreeLine, TreeLines};
pub use nodes::{
    DConstant, DEnum, DFunction, DImpl, DMacroAttr, DMacroDecl, DMacroDerv, DMacroFunc, DModule,
    DStatic, DStruct, DTrait, DTypeAlias, DUnion,
};
pub use stats::{ImplCount, ImplCounts, ImplKind, ItemCount};
