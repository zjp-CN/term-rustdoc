use super::{DImpl, IDs, IdToID, IndexMap, SliceToIds, ID};
use rustdoc_types::{Struct, StructKind};

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
