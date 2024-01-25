use crate::tree::{
    impls::show::{show_ids, show_names, DocTree, Show},
    DImpl, IDMap, IDs, IndexMap, SliceToIds, ID,
};
use rustdoc_types::{Enum, ItemKind};

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

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        node!("[enum] {}", map.path(&self.id, ItemKind::Enum)).with_leaves([
            "Variants"
                .show()
                .with_leaves(show_names(&*self.variants, icon!("[variant]"), map)),
            self.impls.show_prettier(map),
        ])
    }
}
