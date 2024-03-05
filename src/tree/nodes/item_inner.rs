use super::{DEnum, DModule, DStruct, DTrait, DUnion};

/// Data-carrying items that provide extra tree layer on fields/variants/impls.
pub enum ItemInnerKind {
    Struct(DStruct),
    Enum(DEnum),
    Trait(DTrait),
    Union(DUnion),
}

impl DModule {
    pub fn get_item_inner(&self, id: &str) -> Option<ItemInnerKind> {
        for item in &self.structs {
            if item.id.as_str() == id {
                return Some(ItemInnerKind::Struct(item.clone()));
            }
        }
        for item in &self.enums {
            if item.id.as_str() == id {
                return Some(ItemInnerKind::Enum(item.clone()));
            }
        }
        for item in &self.traits {
            if item.id.as_str() == id {
                return Some(ItemInnerKind::Trait(item.clone()));
            }
        }
        for item in &self.unions {
            if item.id.as_str() == id {
                return Some(ItemInnerKind::Union(item.clone()));
            }
        }
        None
    }
}
