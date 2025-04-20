use crate::tree::{
    impls::show::{show_ids, DocTree, Show},
    DImpl, IDMap, IDs,
};
use rustdoc_types::{Id, Union};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DUnion {
    pub id: Id,
    pub fields: IDs,
    pub impls: DImpl,
}
impl DUnion {
    pub fn new(id: Id, item: &Union, map: &IDMap) -> Self {
        DUnion {
            id,
            fields: item.fields.clone().into_boxed_slice(),
            impls: DImpl::new(&item.impls, map),
        }
    }

    /// External items need external crates compiled to know details,
    /// and the ID here is for PathMap, not IndexMap.
    pub fn new_external(id: Id) -> Self {
        let (fields, impls) = Default::default();
        DUnion { id, fields, impls }
    }

    pub fn fields_tree(&self, map: &IDMap) -> DocTree {
        let mut root = node!(Union: map, self.id);
        names_node!(@iter self map root
            fields Field
        );
        root
    }
}

impl Show for DUnion {
    fn show(&self) -> DocTree {
        format!("[union] {:?}", self.id).show().with_leaves([
            "Fields".show().with_leaves(show_ids(&self.fields)),
            self.impls.show(),
        ])
    }

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        let fields = names_node!(@single
            self map NoFields,
            Fields fields Field
        );
        node!(Union: map, self.id).with_leaves([fields, self.impls.show_prettier(map)])
    }
}
