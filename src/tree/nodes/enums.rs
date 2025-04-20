use crate::tree::{
    impls::show::{show_ids, DocTree, Show},
    DImpl, IDMap, IDs,
};
use rustdoc_types::{Enum, Id};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DEnum {
    pub id: Id,
    pub variants: IDs,
    // variants_stripped: bool, -> Does this really make sense?
    pub impls: DImpl,
}
impl DEnum {
    pub fn new(id: Id, item: &Enum, map: &IDMap) -> Self {
        DEnum {
            id,
            variants: item.variants.clone().into_boxed_slice(),
            impls: DImpl::new(&item.impls, map),
        }
    }

    /// External items need external crates compiled to know details,
    /// and the ID here is for PathMap, not IndexMap.
    pub fn new_external(id: Id) -> Self {
        let (variants, impls) = Default::default();
        DEnum {
            id,
            variants,
            impls,
        }
    }

    pub fn variants_tree(&self, map: &IDMap) -> DocTree {
        let mut root = node!(Enum: map, self.id);
        names_node!(@iter self map root
            variants Variant
        );
        root
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
        let variants = names_node!(@single
            self map NoVariants,
            Variants variants Variant
        );
        node!(Enum: map, self.id).with_leaves([variants, self.impls.show_prettier(map)])
    }
}
