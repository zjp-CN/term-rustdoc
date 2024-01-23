use super::{DImpl, IDs, IndexMap, SliceToIds, ID};
use rustdoc_types::Enum;

pub struct DEnum {
    pub id: ID,
    pub variants: IDs,
    // variants_stripped: bool, -> Does this really make sense?
    pub impls: Box<DImpl>,
}
impl DEnum {
    pub fn new(id: ID, item: &Enum, index: &IndexMap) -> Self {
        DEnum {
            id,
            variants: item.variants.to_ids(),
            impls: DImpl::new(&item.impls, index),
        }
    }
}
