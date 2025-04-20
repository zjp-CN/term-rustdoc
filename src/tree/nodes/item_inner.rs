use super::{DEnum, DModule, DStruct, DTrait, DUnion};
use crate::tree::{DocTree, IDMap, Show};
use rustdoc_types::{Id, ItemEnum};

/// Data-carrying items that provide extra tree layer on fields/variants/impls.
#[derive(Debug, Clone, Copy)]
pub enum DataItemKind {
    Module,
    Struct,
    Enum,
    Trait,
    Union,
}

impl DataItemKind {
    pub fn new(id: &Id, map: &IDMap) -> Option<DataItemKind> {
        map.get_item(id).and_then(|item| {
            Some(match &item.inner {
                ItemEnum::Module(_) => DataItemKind::Module,
                ItemEnum::Struct(_) => DataItemKind::Struct,
                ItemEnum::Enum(_) => DataItemKind::Enum,
                ItemEnum::Trait(_) => DataItemKind::Trait,
                ItemEnum::Union(_) => DataItemKind::Union,
                ItemEnum::Use(reexport) => {
                    let id = &reexport.id?;
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
            id: &Id,
            f: impl Copy + Fn(&$typ) -> T,
        ) -> Option<T> {
            for item in &self.$field {
                if item.id == *id {
                    return Some(f(item));
                }
            }
            for m in &self.modules {
                let tree = m.$fname(id, f);
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
    fn search_for_module<T>(&self, id: &Id, f: impl Copy + Fn(&DModule) -> T) -> Option<T> {
        if self.id == *id {
            return Some(f(self));
        }
        for m in &self.modules {
            let tree = m.search_for_module(id, f);
            if tree.is_some() {
                return tree;
            }
        }
        None
    }

    pub fn item_inner_tree(&self, id: &Id, map: &IDMap) -> Option<DocTree> {
        let kind = DataItemKind::new(id, map)?;
        match kind {
            DataItemKind::Struct => self.search_for_struct(id, |x| x.show_prettier(map)),
            DataItemKind::Enum => self.search_for_enum(id, |x| x.show_prettier(map)),
            DataItemKind::Trait => self.search_for_trait(id, |x| x.show_prettier(map)),
            DataItemKind::Union => self.search_for_union(id, |x| x.show_prettier(map)),
            DataItemKind::Module => self.search_for_module(id, |x| x.item_tree(map)),
        }
    }

    pub fn impl_tree(&self, id: &Id, map: &IDMap) -> Option<DocTree> {
        let kind = DataItemKind::new(id, map)?;
        match kind {
            DataItemKind::Struct => self.search_for_struct(id, |x| x.impls.show_prettier(map)),
            DataItemKind::Enum => self.search_for_enum(id, |x| x.impls.show_prettier(map)),
            DataItemKind::Trait => self.search_for_trait(id, |x| x.show_prettier(map)),
            DataItemKind::Union => self.search_for_union(id, |x| x.impls.show_prettier(map)),
            _ => None,
        }
    }

    pub fn implementor_tree(&self, id: &Id, map: &IDMap) -> Option<DocTree> {
        self.search_for_trait(id, |x| x.implementors(map))
    }

    pub fn associated_item_tree(&self, id: &Id, map: &IDMap) -> Option<DocTree> {
        self.search_for_trait(id, |x| x.associated_items(map))
    }

    pub fn field_tree(&self, id: &Id, map: &IDMap) -> Option<DocTree> {
        let kind = DataItemKind::new(id, map)?;
        match kind {
            DataItemKind::Struct => self.search_for_struct(id, |x| x.fields_tree(map)),
            DataItemKind::Enum => self.search_for_enum(id, |x| x.variants_tree(map)),
            DataItemKind::Union => self.search_for_union(id, |x| x.fields_tree(map)),
            _ => None,
        }
    }
}
