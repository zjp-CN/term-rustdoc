use super::{DEnum, DModule, DStruct, DTrait, DUnion};
use crate::tree::{DocTree, IDMap, Show};
use rustdoc_types::ItemEnum;

/// Data-carrying items that provide extra tree layer on fields/variants/impls.
#[derive(Debug, Clone, Copy)]
pub enum DataItemKind {
    Struct,
    Enum,
    Trait,
    Union,
}

impl DataItemKind {
    pub fn new(id: &str, map: &IDMap) -> Option<DataItemKind> {
        map.get_item(id).and_then(|item| {
            Some(match &item.inner {
                ItemEnum::Struct(_) => DataItemKind::Struct,
                ItemEnum::Enum(_) => DataItemKind::Enum,
                ItemEnum::Trait(_) => DataItemKind::Trait,
                ItemEnum::Union(_) => DataItemKind::Union,
                ItemEnum::Import(reexport) => {
                    let id = reexport.id.as_ref().map(|id| &*id.0)?;
                    DataItemKind::new(id, map)?
                }
                _ => return None,
            })
        })
    }
}

macro_rules! search {
    () => {
        search! {
            search_for_struct structs DStruct,
            search_for_enum   enums   DEnum,
            search_for_trait  traits  DTrait,
            search_for_union  unions  DUnion,
        }
    };
    ($fname:ident $field:ident $typ:ident) => {
        fn $fname<T>(
            &self,
            id: &str,
            map: &IDMap,
            f: impl Copy + Fn(&$typ) -> T,
        ) -> Option<T> {
            for item in &self.$field {
                if item.id.as_str() == id {
                    return Some(f(item));
                }
            }
            for m in &self.modules {
                let tree = m.$fname(id, map, f);
                if tree.is_some() {
                    return tree;
                }
            }
            None
        }
    };
    ($($fname:ident $field:ident $typ:ident,)+) => {
        impl DModule { $( search! { $fname $field $typ } )+ }
    };
}

search! {}

// Search after the kind is known to improve efficiency.
impl DModule {
    pub fn item_inner_tree(&self, id: &str, map: &IDMap) -> Option<DocTree> {
        let kind = DataItemKind::new(id, map)?;
        match kind {
            DataItemKind::Struct => self.search_for_struct(id, map, |x| x.show_prettier(map)),
            DataItemKind::Enum => self.search_for_enum(id, map, |x| x.show_prettier(map)),
            DataItemKind::Trait => self.search_for_trait(id, map, |x| x.show_prettier(map)),
            DataItemKind::Union => self.search_for_union(id, map, |x| x.show_prettier(map)),
        }
    }

    pub fn impl_tree(&self, id: &str, map: &IDMap) -> Option<DocTree> {
        let kind = DataItemKind::new(id, map)?;
        match kind {
            DataItemKind::Struct => self.search_for_struct(id, map, |x| x.impls.show_prettier(map)),
            DataItemKind::Enum => self.search_for_enum(id, map, |x| x.impls.show_prettier(map)),
            DataItemKind::Trait => self.search_for_trait(id, map, |x| x.show_prettier(map)),
            DataItemKind::Union => self.search_for_union(id, map, |x| x.impls.show_prettier(map)),
        }
    }

    pub fn implementor_tree(&self, id: &str, map: &IDMap) -> Option<DocTree> {
        self.search_for_trait(id, map, |x| x.implementors(map))
    }

    pub fn associated_item_tree(&self, id: &str, map: &IDMap) -> Option<DocTree> {
        self.search_for_trait(id, map, |x| x.associated_items(map))
    }
}
