use crate::tree::{
    impls::show::{show_ids, DocTree, Show},
    DImpl, IDMap, IDs, SliceToIds, ID,
};
use rustdoc_types::Enum;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DEnum {
    pub id: ID,
    pub variants: IDs,
    // variants_stripped: bool, -> Does this really make sense?
    pub impls: DImpl,
}
impl DEnum {
    pub fn new(id: ID, item: &Enum, map: &IDMap) -> Self {
        DEnum {
            id,
            variants: item.variants.to_ids(),
            impls: DImpl::new(&item.impls, map),
        }
    }

    /// External items need external crates compiled to know details,
    /// and the ID here is for PathMap, not IndexMap.
    pub fn new_external(id: ID) -> Self {
        let (variants, impls) = Default::default();
        DEnum {
            id,
            variants,
            impls,
        }
    }

    pub fn variants_tree(&self, map: &IDMap) -> DocTree {
        let variants = names_node!(@single
            self map NoVariants,
            Variants variants Variant
        );
        node!(Enum: map, &self.id).with_leaves([variants])
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
        let mut root = self.variants_tree(map);
        root.push(self.impls.show_prettier(map));
        root
    }
}
