use rustdoc_types::*;
use std::collections::HashMap;

#[allow(non_snake_case)]
mod id;
mod impls;
mod nodes;
mod stats;

pub use id::{IDs, IdToID, SliceToIds, ID};
pub use nodes::{
    DConstant, DEnum, DFunctions, DImpl, DMacro, DMacroKind, DModule, DStatic, DStruct, DTrait,
    DTypeAlias, DUnion,
};
pub use stats::{ImplCount, ImplCounts, ImplKind, ItemCount};

// Crate.index
pub type IndexMap = HashMap<Id, Item>;
// Crate.paths
pub type PathMap = HashMap<Id, ItemSummary>;
