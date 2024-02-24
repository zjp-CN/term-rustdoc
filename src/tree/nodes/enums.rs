use crate::tree::{
    impls::show::{show_ids, DocTree, Show},
    DImpl, IDMap, IDs, IndexMap, SliceToIds, ID,
};
use rustdoc_types::Enum;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DEnum {
    pub id: ID,
    pub variants: IDs,
    // variants_stripped: bool, -> Does this really make sense?
    pub impls: Box<DImpl>,
}
impl DEnum {
    pub fn new(id: ID, item: &Enum, index: &IDMap) -> Box<Self> {
        Box::new(DEnum {
            id,
            variants: item.variants.to_ids(),
            impls: DImpl::new(&item.impls, index),
        })
    }
}

impl Show for DEnum {
    fn show(&self) -> DocTree {
        "[enum]".show().with_leaves([
            "Variants".show().with_leaves(show_ids(&self.variants)),
            self.impls.show(),
        ])
    }

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        let variant = names_node!(@single
            self map NoVariants,
            Variants variants Variant
        );
        let leaves = [variant, self.impls.show_prettier(map)];
        node!(Enum: map, &self.id).with_leaves(leaves)
    }
}
