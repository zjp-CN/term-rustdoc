use crate::tree::{
    impls::show::{show_ids, show_names, DocTree, Show},
    DImpl, IDMap, IDs, Tag,
};
use rustdoc_types::{Id, Struct, StructKind};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DStruct {
    pub id: Id,
    pub fields: IDs,
    pub contain_private_fields: bool,
    pub impls: DImpl,
}

impl DStruct {
    pub fn new(id: Id, item: &Struct, map: &IDMap) -> Self {
        let mut contain_private_fields = false;
        let fields = match &item.kind {
            StructKind::Unit => IDs::default(),
            StructKind::Tuple(fields) => fields
                .iter()
                .filter_map(|&id| {
                    if id.is_none() {
                        contain_private_fields = true;
                    }
                    id
                })
                .collect(),
            StructKind::Plain {
                fields,
                has_stripped_fields,
            } => {
                contain_private_fields = *has_stripped_fields;
                fields.clone().into_boxed_slice()
            }
        };
        let impls = DImpl::new(&item.impls, map);
        DStruct {
            id,
            fields,
            contain_private_fields,
            impls,
        }
    }

    /// External items need external crates compiled to know details,
    /// and the ID here is for PathMap, not IndexMap.
    pub fn new_external(id: Id) -> Self {
        let (fields, impls) = Default::default();
        DStruct {
            id,
            fields,
            impls,
            contain_private_fields: true,
        }
    }

    pub fn fields_tree(&self, map: &IDMap) -> DocTree {
        let mut root = node!(Struct: map, self.id);
        match (self.fields.len(), self.contain_private_fields) {
            (0, true) => root.push(private_fields()),
            (0, false) => root.push(Tag::NoFields.show()),
            (_, true) => {
                root.extend(show_names(&*self.fields, Tag::Field, map).chain([private_fields()]))
            }
            (_, false) => root.extend(show_names(&*self.fields, Tag::Field, map)),
        };
        root
    }
}

fn private_fields() -> DocTree {
    Tag::FieldsPrivate.show()
}

fn fields_root() -> DocTree {
    Tag::Fields.show()
}

impl Show for DStruct {
    fn show(&self) -> DocTree {
        format!("[struct] {:?}", self.id).show().with_leaves([
            "Fields".show().with_leaves(
                show_ids(&self.fields).chain(self.contain_private_fields.then(private_fields)),
            ),
            self.impls.show(),
        ])
    }

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        let mut root = node!(Struct: map, self.id);
        match (self.fields.len(), self.contain_private_fields) {
            (0, true) => root.push(private_fields()),
            (0, false) => root.push(Tag::NoFields.show()),
            (_, true) => {
                root.push(fields_root().with_leaves(
                    show_names(&*self.fields, Tag::Field, map).chain([private_fields()]),
                ))
            }
            (_, false) => {
                root.push(fields_root().with_leaves(show_names(&*self.fields, Tag::Field, map)))
            }
        };
        root.push(self.impls.show_prettier(map));
        root
    }
}
