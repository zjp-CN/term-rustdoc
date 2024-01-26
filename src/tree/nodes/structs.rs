use crate::tree::{
    impls::show::{show_ids, show_names, DocTree, Show},
    DImpl, IDMap, IDs, IdToID, IndexMap, SliceToIds, ID,
};
use crate::util::xformat;
use rustdoc_types::{ItemKind, Struct, StructKind};

pub struct DStruct {
    pub id: ID,
    pub fields: IDs,
    pub contain_private_fields: bool,
    pub impls: Box<DImpl>,
}

impl DStruct {
    pub fn new(id: ID, item: &Struct, index: &IndexMap) -> Self {
        let mut contain_private_fields = false;
        let fields = match &item.kind {
            StructKind::Unit => IDs::default(),
            StructKind::Tuple(fields) => fields
                .iter()
                .filter_map(|f| {
                    let id = f.as_ref().map(|id| id.to_ID());
                    if id.is_none() {
                        contain_private_fields = true;
                    }
                    id
                })
                .collect(),
            StructKind::Plain {
                fields,
                fields_stripped,
            } => {
                contain_private_fields = *fields_stripped;
                fields.to_ids()
            }
        };
        let impls = DImpl::new(&item.impls, index);
        DStruct {
            id,
            fields,
            contain_private_fields,
            impls,
        }
    }
}

fn private_fields() -> DocTree {
    "/* private fields */".show()
}

fn fields_root(len: usize) -> DocTree {
    xformat!("{len} field{}", if len > 1 { "s" } else { "" }).show()
}

impl Show for DStruct {
    fn show(&self) -> DocTree {
        format!("[struct] {}", self.id).show().with_leaves([
            "Fields".show().with_leaves(
                show_ids(&self.fields).chain(self.contain_private_fields.then(private_fields)),
            ),
            self.impls.show(),
        ])
    }

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        let mut node = node!("[struct] {}", map.path(&self.id, ItemKind::Struct));
        let len = self.fields.len();
        match (len, self.contain_private_fields) {
            (0, true) => node.extend([private_fields()]),
            (0, false) => node.extend(["No fields!".show()]),
            (_, true) => node.extend([
                fields_root(len).with_leaves(show_names(&*self.fields, icon!("[field]"), map)),
                private_fields(),
            ]),
            (_, false) => node.extend([fields_root(len).with_leaves(show_names(
                &*self.fields,
                icon!("[field]"),
                map,
            ))]),
        }
        node.push(self.impls.show_prettier(map));
        node
    }
}
