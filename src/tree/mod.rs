use rustdoc_types::*;
use std::collections::HashMap;

#[macro_use]
mod impls;
// The inner macro `icon!` can be used afterwards in submods

#[allow(non_snake_case)]
mod id;
mod nodes;
mod stats;

pub use id::{IDs, IdToID, SliceToIds, ID};
pub use impls::show::{DocTree, GlyphPalette, Show};
pub use nodes::{
    DConstant, DEnum, DFunction, DImpl, DMacro, DMacroKind, DModule, DStatic, DStruct, DTrait,
    DTypeAlias, DUnion,
};
pub use stats::{ImplCount, ImplCounts, ImplKind, ItemCount};

// Crate.index
pub type IndexMap = HashMap<Id, Item>;
// Crate.paths
pub type PathMap = HashMap<Id, ItemSummary>;
