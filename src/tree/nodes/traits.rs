use crate::tree::{
    impls::show::{show_ids, DocTree, Show},
    IDMap, IDs, Tag,
};
use rustdoc_types::{Id, ItemEnum, Trait};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DTrait {
    pub id: Id,
    pub types: IDs,
    pub constants: IDs,
    pub functions: IDs,
    pub implementations: IDs,
}
impl DTrait {
    pub fn new(id: Id, item: &Trait, map: &IDMap) -> Self {
        let [mut types, mut constants, mut functions]: [Vec<Id>; 3] = Default::default();
        let trait_id = &id;
        for &id in &item.items {
            if let Some(assoc) = map.get_item(&id) {
                match &assoc.inner {
                    ItemEnum::AssocType { .. } => types.push(id),
                    ItemEnum::AssocConst { .. } => constants.push(id),
                    ItemEnum::Function(_) => functions.push(id),
                    _ => warn!(
                        "`{}` ({id:?}) should refer to an associated item \
                         (type/constant/function) in Trait `{}` ({trait_id:?})",
                        map.name(&id),
                        map.name(trait_id)
                    ),
                }
            } else {
                warn!(
                    "the trait item `{}` ({id:?}) not found in Crate's index",
                    map.name(&id)
                );
            }
        }
        types.sort_unstable_by_key(|id| map.name(id));
        constants.sort_unstable_by_key(|id| map.name(id));
        functions.sort_unstable_by_key(|id| map.name(id));
        DTrait {
            id,
            types: types.into(),
            constants: constants.into(),
            functions: functions.into(),
            implementations: item.implementations.clone().into_boxed_slice(),
        }
    }

    /// External items need external crates compiled to know details,
    /// and the ID here is for PathMap, not IndexMap.
    pub fn new_external(id: Id) -> Self {
        let (types, constants, functions, implementations) = Default::default();
        DTrait {
            id,
            types,
            constants,
            functions,
            implementations,
        }
    }

    pub fn associated_items(&self, map: &IDMap) -> DocTree {
        let mut root = node!(Trait: map, self.id);
        names_node!(@iter self map root
            constants AssocConst,
            types     AssocType,
            functions AssocFn,
        );
        root
    }

    pub fn implementors(&self, map: &IDMap) -> DocTree {
        let mut root = node!(Trait: map, self.id);
        names_node!(@iter self map root
            implementations Implementor,
        );
        root
    }
}

impl Show for DTrait {
    fn show(&self) -> DocTree {
        format!("[trait] {:?}", self.id).show().with_leaves([
            "Associated Constants"
                .show()
                .with_leaves(show_ids(&self.constants)),
            "Associated Types".show().with_leaves(show_ids(&self.types)),
            "Associated Functions"
                .show()
                .with_leaves(show_ids(&self.functions)),
            "Implementors"
                .show()
                .with_leaves(show_ids(&self.implementations)),
        ])
    }

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        let root = node!(Trait: map, self.id);
        let leaves = names_node!(
            self map root.with_leaves([Tag::NoAssocOrImpls.show()]),
            AssocConsts  constants AssocConst,
            AssocTypes   types     AssocType,
            AssocFns     functions AssocFn,
            Implementors implementations Implementor,
        );
        root.with_leaves(leaves)
    }
}
