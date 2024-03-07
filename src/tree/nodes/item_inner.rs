use super::{DEnum, DModule, DStruct, DTrait, DUnion};
use crate::tree::{DocTree, IDMap, Show};

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
        for m in &self.modules {
            let item = m.get_item_inner(id);
            if item.is_some() {
                return item;
            }
        }
        None
    }

    pub fn item_inner_tree(&self, id: &str, map: &IDMap) -> Option<DocTree> {
        for item in &self.structs {
            if item.id.as_str() == id {
                return Some(item.show_prettier(map));
            }
        }
        for item in &self.enums {
            if item.id.as_str() == id {
                return Some(item.show_prettier(map));
            }
        }
        for item in &self.traits {
            if item.id.as_str() == id {
                return Some(item.show_prettier(map));
            }
        }
        for item in &self.unions {
            if item.id.as_str() == id {
                return Some(item.show_prettier(map));
            }
        }
        for m in &self.modules {
            let tree = m.item_inner_tree(id, map);
            if tree.is_some() {
                return tree;
            }
        }
        None
    }

    pub fn impl_tree(&self, id: &str, map: &IDMap) -> Option<DocTree> {
        for item in &self.structs {
            if item.id.as_str() == id {
                return Some(item.impls.show_prettier(map));
            }
        }
        for item in &self.enums {
            if item.id.as_str() == id {
                return Some(item.impls.show_prettier(map));
            }
        }
        for item in &self.traits {
            if item.id.as_str() == id {
                return Some(item.show_prettier(map));
            }
        }
        for item in &self.unions {
            if item.id.as_str() == id {
                return Some(item.impls.show_prettier(map));
            }
        }
        for m in &self.modules {
            let tree = m.impl_tree(id, map);
            if tree.is_some() {
                return tree;
            }
        }
        None
    }
}
