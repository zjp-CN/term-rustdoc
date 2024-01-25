use crate::tree::{
    impls::show::{show_ids, show_paths, DocTree, Show},
    DImpl, IDMap, IDs, IndexMap, SliceToIds, ID,
};
use rustdoc_types::{ItemKind, Union};

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

impl Show for DUnion {
    fn show(&self) -> DocTree {
        format!("[union] {}", self.id).show().with_leaves([
            "Fields".show().with_leaves(show_ids(&self.fields)),
            self.impls.show(),
        ])
    }

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        node!("[union] {}", map.path(&self.id, ItemKind::Union)).with_leaves([
            "Fields".show().with_leaves(show_paths(
                &*self.fields,
                ItemKind::StructField,
                icon!("[field]"),
                map,
            )),
            self.impls.show_prettier(map),
        ])
    }
}
