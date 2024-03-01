use crate::tree::{
    impls::show::{show_ids, DocTree, Show},
    DImpl, IDMap, IDs, SliceToIds, ID,
};
use rustdoc_types::Union;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DUnion {
    pub id: ID,
    pub fields: IDs,
    pub impls: DImpl,
}
impl DUnion {
    pub fn new(id: ID, item: &Union, map: &IDMap) -> Self {
        DUnion {
            id,
            fields: item.fields.to_ids(),
            impls: DImpl::new(&item.impls, map),
        }
    }

    /// External items need external crates compiled to know details,
    /// and the ID here is for PathMap, not IndexMap.
    pub fn new_external(id: ID) -> Self {
        let (fields, impls) = Default::default();
        DUnion { id, fields, impls }
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
        let fields = names_node!(@single
            self map NoFields,
            Fields fields Field
        );
        node!(Union: map, &self.id).with_leaves([fields, self.impls.show_prettier(map)])
    }
}
