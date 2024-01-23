use super::{IdToID, ID};
use rustdoc_types::{Path, Type, TypeAlias};

pub struct DTypeAlias {
    pub id: ID,
    /// points to a source type resolved as path
    ///
    /// A type alias may point to non-path-based type though.
    pub source_path: Option<ID>,
}
impl DTypeAlias {
    pub fn new(id: ID, item: &TypeAlias) -> Self {
        Self {
            id,
            source_path: match &item.type_ {
                Type::ResolvedPath(Path { id, .. }) => Some(id.to_ID()),
                _ => None,
            },
        }
    }
}
