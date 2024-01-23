use crate::tree::{
    impls::show::{show_ids, show_ids_with, DocTree, Show},
    DImpl, IDs, IndexMap, SliceToIds, ID,
};
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

impl Show for DUnion {
    fn show(&self) -> DocTree {
        format!("[union] {}", self.id).show().with_leaves([
            "Fields".show().with_leaves(show_ids(&self.fields)),
            self.impls.show(),
        ])
    }

    fn show_prettier(&self) -> DocTree {
        format!("[union] {}", self.id)
            .show_prettier()
            .with_leaves([
                "Fields"
                    .show_prettier()
                    .with_leaves(show_ids_with(&self.fields, icon!("[field]"))),
                self.impls.show_prettier(),
            ])
    }
}
