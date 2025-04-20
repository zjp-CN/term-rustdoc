use crate::tree::{
    impls::show::{show_ids, show_names, DocTree, Show},
    IDMap, IDs, Tag,
};
use itertools::Itertools;
use rustdoc_types::{Id, Impl, ItemEnum};
use serde::{Deserialize, Serialize};

/// All elements in slice are ordered by name.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct DImpl {
    pub inherent: Box<[DImplInner]>,
    pub trait_: Box<[DImplInner]>,
    pub auto: Box<[DImplInner]>,
    pub blanket: Box<[DImplInner]>,
    /// Inherent items from multiple impl blocks are merged into one block
    /// with themselves sorted by name.
    ///
    /// NOTE: the impl block id is empty and invalid!
    pub merged_inherent: Box<DImplInner>,
}

impl DImpl {
    pub fn new(ids: &[Id], map: &IDMap) -> Self {
        if ids.is_empty() {
            return Default::default();
        }
        // let [mut inherent, mut trait_, mut auto, mut blanket]: [Vec<_>; 4] = Default::default();
        todo!();
        // for id in ids {
        // FIXME:
        // if id.starts_with("a:") {
        //     auto.push(DImplInner::new_with_no_details(id));
        // } else if id.starts_with("b:") {
        //     blanket.push(DImplInner::new_with_no_details(id));
        // } else if let Some(item) = map.get_item(id) {
        //     if let ItemEnum::Impl(impl_) = &item.inner {
        //         if impl_.trait_.is_none() {
        //             inherent.push(DImplInner::new(id, impl_, map));
        //         } else {
        //             trait_.push(DImplInner::new(id, impl_, map));
        //         }
        //     } else {
        //         warn!("{id:?} in Crate's index doesn't refer to an impl item");
        //     }
        // } else {
        //     warn!("the impl with {id:?} not found in Crate's index");
        // }
        // }
        // inherent.sort_unstable_by_key(|x| map.name(&x.id));
        // trait_.sort_unstable_by_key(|x| map.name(&x.id));
        // auto.sort_unstable_by_key(|x| map.name(&x.id));
        // blanket.sort_unstable_by_key(|x| map.name(&x.id));
        // let merged_inherent = DImplInner::merge_inherent_impls(&inherent, map);
        // DImpl {
        //     inherent: inherent.into(),
        //     trait_: trait_.into(),
        //     auto: auto.into(),
        //     blanket: blanket.into(),
        //     merged_inherent: Box::new(merged_inherent),
        // }
    }
    pub fn is_empty(&self) -> bool {
        self.auto.is_empty()
            && self.blanket.is_empty()
            && self.inherent.is_empty()
            && self.trait_.is_empty()
    }
}

impl Show for DImpl {
    fn show(&self) -> DocTree {
        "Implementations".show().with_leaves([
            "Inherent Impls"
                .show()
                .with_leaves(self.inherent.iter().map(|i| i.show())),
            "Trait Impls"
                .show()
                .with_leaves(self.trait_.iter().map(|i| i.show())),
            "Auto Impls"
                .show()
                .with_leaves(self.auto.iter().map(|i| i.show())),
            "Blanket Impls"
                .show()
                .with_leaves(self.blanket.iter().map(|i| i.show())),
        ])
    }

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        if self.is_empty() {
            return Tag::NoImpls.show();
        }
        let mut root = Tag::Implementations.show();
        if !self.merged_inherent.is_empty() {
            let tree = Tag::InherentImpls.show();
            // let tag = Tag::ImplInherent;
            // root.push(tree.with_leaves(self.inherent.iter().map(|i| i.show_prettier(tag, map))));
            root.push(tree.with_leaves(self.merged_inherent.show_prettier_iter(map)));
        }
        if !self.trait_.is_empty() {
            let tree = Tag::TraitImpls.show();
            let tag = Tag::ImplTrait;
            root.push(tree.with_leaves(self.trait_.iter().map(|i| i.show_prettier(tag, map))));
        }
        if !self.auto.is_empty() {
            let tree = Tag::AutoImpls.show();
            let tag = Tag::ImplAuto;
            root.push(tree.with_leaves(self.auto.iter().map(|i| i.show_prettier(tag, map))));
        }
        if !self.blanket.is_empty() {
            let tree = Tag::BlanketImpls.show();
            let tag = Tag::ImplBlanket;
            root.push(tree.with_leaves(self.blanket.iter().map(|i| i.show_prettier(tag, map))));
        }
        root
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DImplInner {
    pub id: Id,
    pub functions: IDs,
    pub constants: IDs,
    pub types: IDs,
}

impl Default for DImplInner {
    fn default() -> Self {
        let id = Id(0);
        let (functions, constants, types) = Default::default();
        DImplInner {
            id,
            functions,
            constants,
            types,
        }
    }
}

impl DImplInner {
    pub fn new(id: &Id, imp: &Impl, map: &IDMap) -> Self {
        let [mut functions, mut constants, mut types]: [Vec<Id>; 3] = Default::default();
        for item in imp.items.iter().flat_map(|assc| map.get_item(assc)) {
            match &item.inner {
                ItemEnum::Function(_) => functions.push(item.id),
                ItemEnum::Constant { .. } => constants.push(item.id),
                ItemEnum::TypeAlias(_) => types.push(item.id),
                _ => (),
            };
        }
        functions.sort_unstable_by_key(|id| map.name(id));
        constants.sort_unstable_by_key(|id| map.name(id));
        types.sort_unstable_by_key(|id| map.name(id));
        DImplInner {
            id: *id,
            functions: functions.into(),
            constants: constants.into(),
            types: types.into(),
        }
    }

    pub fn new_with_no_details(id: Id) -> Self {
        DImplInner {
            id,
            ..Default::default()
        }
    }

    fn show(&self) -> DocTree {
        let mut root = self.id.show();
        if !self.functions.is_empty() {
            root.push("Functions".show().with_leaves(show_ids(&self.functions)));
        }
        if !self.constants.is_empty() {
            root.push("Constants".show().with_leaves(show_ids(&self.constants)));
        }
        if !self.types.is_empty() {
            root.push("Types".show().with_leaves(show_ids(&self.types)));
        }
        root
    }

    fn show_prettier(&self, tag: Tag, map: &IDMap) -> DocTree {
        let root = DocTree::new(map.name(&self.id), tag, Some(self.id));
        // too verbose!
        // let leaves = names_node!(
        //     self map root,
        //     Functions  functions Function,
        //     Constants  constants Constant,
        //     TypeAliass types     TypeAlias,
        // );
        root.with_leaves(self.show_prettier_iter(map))
    }

    /// mainly for inherent impls
    fn show_prettier_iter<'s: 'ret, 'map: 'ret, 'ret>(
        &'s self,
        map: &'map IDMap,
    ) -> impl 'ret + Iterator<Item = DocTree> {
        show_names(&*self.constants, Tag::Constant, map)
            .chain(show_names(&*self.types, Tag::TypeAlias, map))
            .chain(show_names(&*self.functions, Tag::Function, map))
    }

    fn merge_inherent_impls(impls: &[Self], map: &IDMap) -> Self {
        let iter = impls.iter();
        Self {
            id: Id(0),
            functions: iter
                .clone()
                .flat_map(|x| x.functions.iter())
                .sorted_unstable_by_key(|id| map.name(id))
                .cloned()
                .collect(),
            constants: iter
                .clone()
                .flat_map(|x| x.constants.iter())
                .sorted_unstable_by_key(|id| map.name(id))
                .cloned()
                .collect(),
            types: iter
                .flat_map(|x| x.types.iter())
                .sorted_unstable_by_key(|id| map.name(id))
                .cloned()
                .collect(),
        }
    }

    fn is_empty(&self) -> bool {
        self.functions.is_empty() && self.constants.is_empty() && self.types.is_empty()
    }
}
