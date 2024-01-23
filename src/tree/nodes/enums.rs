use crate::tree::{
    impls::show::{show_ids, show_ids_with, DocTree, Show},
    DImpl, IDs, IndexMap, SliceToIds, ID,
};
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

impl Show for DEnum {
    fn show(&self) -> DocTree {
        "[enum]".show().with_leaves([
            "Variants".show().with_leaves(show_ids(&self.variants)),
            self.impls.show(),
        ])
    }

    fn show_prettier(&self) -> DocTree {
        "[enum]".show_prettier().with_leaves([
            "Variants"
                .show_prettier()
                .with_leaves(show_ids_with(&self.variants, icon!("[variant]"))),
            self.impls.show_prettier(),
        ])
    }
}
