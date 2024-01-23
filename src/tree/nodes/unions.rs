use super::{DImpl, IDs, IndexMap, SliceToIds, ID};
use rustdoc_types::Union;

pub struct DUnion {
    pub id: ID,
    pub fields: IDs,
    pub impls: Box<DImpl>,
}
impl DUnion {
    pub fn new(id: ID, item: &Union, index: &IndexMap) -> Self {
        DUnion {
            id,
            fields: item.fields.to_ids(),
            impls: DImpl::new(&item.impls, index),
        }
    }
}
