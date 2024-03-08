mod impls;
pub use impls::{DImpl, DImplInner};

mod structs;
pub use structs::DStruct;

mod unions;
pub use unions::DUnion;

mod enums;
pub use enums::DEnum;

mod traits;
pub use traits::DTrait;

mod imports;

mod item_inner;
pub use item_inner::ItemInnerKind;

use super::IDMap;
use crate::tree::{
    impls::show::{DocTree, Show},
    IdToID, ID,
};
use rustdoc_types::{Id, Item, ItemEnum, MacroKind, Module};
use serde::{Deserialize, Serialize};
use std::ops::Not;

/// Module tree with structural items.
/// All the items only carry ids without actual data.
// NOTE: small improvement by turning all the types of fields
// from Vec to Arr after instantiation.
#[derive(Default, Serialize, Deserialize)]
pub struct DModule {
    pub id: ID,
    // If true, this module is not part of the public API,
    // but it contains items that are re-exported as public API.
    // is_stripped: bool,
    pub modules: Vec<DModule>,
    pub structs: Vec<DStruct>,
    pub unions: Vec<DUnion>,
    pub enums: Vec<DEnum>,
    pub traits: Vec<DTrait>,
    pub functions: Vec<DFunction>,
    pub constants: Vec<DConstant>,
    pub statics: Vec<DStatic>,
    pub type_alias: Vec<DTypeAlias>,
    pub macros_decl: Vec<DMacroDecl>,
    pub macros_func: Vec<DMacroFunc>,
    pub macros_attr: Vec<DMacroAttr>,
    pub macros_derv: Vec<DMacroDerv>,
}

impl DModule {
    pub fn new(map: &IDMap) -> Self {
        // root module/crate name
        let mut index = map.indexmap().iter();
        let (id, root) = index
            .find_map(|(id, item)| {
                if item.crate_id == 0 {
                    if let ItemEnum::Module(Module {
                        is_crate: true,
                        items,
                        ..
                    }) = &item.inner
                    {
                        return Some((id.to_ID(), items.as_slice()));
                    }
                }
                None
            })
            .expect("root module not found");

        // module component list, used to stop a recursive reexported module
        let mut ancestor = Vec::with_capacity(8);

        let mut dmod = Self::new_inner(id, root, map, &mut ancestor);
        dmod.sort_by_name(map);
        dmod
    }

    fn new_inner(id: ID, inner_items: &[Id], map: &IDMap, ancestor: &mut Vec<ID>) -> Self {
        ancestor.push(id.clone());
        debug!(
            "Module Paths = {:?}",
            ancestor.iter().map(|id| map.path(id)).collect::<Vec<_>>()
        );
        let mut dmod = DModule {
            id,
            ..Default::default()
        };
        dmod.extract_items(inner_items, map, ancestor);
        dmod
    }

    /// External items need external crates compiled to know details,
    /// and the ID here is for PathMap, not IndexMap.
    fn new_external(id: ID) -> Self {
        DModule {
            id,
            ..Default::default()
        }
    }

    fn extract_items(&mut self, inner_items: &[Id], map: &IDMap, ancestor: &mut Vec<ID>) {
        for item_id in inner_items {
            match map.indexmap().get(item_id) {
                Some(item) => self.append(item, map, ancestor),
                None => warn!("the local item {item_id:?} not found in Crate's index"),
            }
        }
    }

    fn append(&mut self, item: &Item, map: &IDMap, ancestor: &mut Vec<ID>) {
        use ItemEnum::*;
        let id = item.id.to_ID();
        match &item.inner {
            Module(item) => {
                let mut kin = ancestor.clone();
                self.modules
                    .push(Self::new_inner(id, &item.items, map, &mut kin))
            }
            Struct(item) => self.structs.push(DStruct::new(id, item, map)),
            Union(item) => self.unions.push(DUnion::new(id, item, map)),
            Enum(item) => self.enums.push(DEnum::new(id, item, map)),
            Trait(item) => self.traits.push(DTrait::new(id, item, map)),
            Function(_) => self.functions.push(DFunction::new(id)),
            Constant(_) => self.constants.push(DConstant::new(id)),
            Static(_) => self.statics.push(DStatic::new(id)),
            TypeAlias(_) => self.type_alias.push(DTypeAlias::new(id)),
            Macro(_) => self.macros_decl.push(DMacroDecl::new(id)),
            ProcMacro(proc) => match proc.kind {
                MacroKind::Bang => self.macros_func.push(DMacroFunc::new(id)),
                MacroKind::Attr => self.macros_attr.push(DMacroAttr::new(id)),
                MacroKind::Derive => self.macros_derv.push(DMacroDerv::new(id)),
            },
            Import(import) => imports::parse_import(id, import, map, self, ancestor),
            // Primitive(_) => todo!(),
            _ => (),
        }
    }
}

macro_rules! impl_show {
    ($( $field:ident => $tag:ident => $node:ident => $fty:ident , )+ ) => {
/// To a recursive tree displayed with ids as nodes.
impl Show for DModule {
    fn show(&self) -> DocTree {
        format!("[mod] {}", self.id).show().with_leaves(
            std::iter::empty()
            $(
                .chain( impl_show!(@show $field $node $fty self map) )
            )+
            .chain(self.modules.iter().map(DModule::show))
        )
    }

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        node!(Module: map, &self.id).with_leaves(
            std::iter::empty()
            $(
                .chain( impl_show!(@pretty $field $node self map) )
            )+
            .chain(self.modules.iter().map(|m| m.show_prettier(map)))
        )
    }
}

impl DModule {
    /// The main tree view as public items in module tree.
    pub fn item_tree(&self, map: &IDMap) -> DocTree {
        node!(Module: map, &self.id).with_leaves(
            std::iter::empty()
            $(
                .chain(self.$field.iter().map(|item| {
                    node!(@name $tag : map, &item.id)
                }))
            )+
            .chain(self.modules.iter().map(|m| m.item_tree(map)))
        )
    }

    /// sort items in their item types by name
    fn sort_by_name(&mut self, map: &IDMap) {
        self.modules.sort_unstable_by(|a, b| map.name(&a.id).cmp(&map.name(&b.id)));
        self.modules.iter_mut().for_each(|m| m.sort_by_name(map));
        $(self.$field.sort_unstable_by(|a, b| map.name(&a.id).cmp(&map.name(&b.id)));)+
    }

    /// NOTE: this method doesn't include nested modules; only returns one-level items with mod root.
    pub fn item_tree_only_in_one_specified_mod(&self, map: &IDMap) -> DocTree {
        node!(Module: map, &self.id).with_leaves(
            std::iter::empty()
            $(
                .chain(self.$field.iter().map(|item| {
                    node!(@name $tag : map, &item.id)
                }))
            )+
        )
    }
}
    };
    (@show $field:ident $node:ident $fty:ident $self:ident $map:ident) => {
        $self.$field.is_empty().not().then(|| {
            $crate::tree::Tag::$node.show().with_leaves($self.$field.iter().map($fty::show))
        })
    };
    (@pretty $field:ident $node:ident $self:ident $map:ident) => {
        $self.$field.is_empty().not().then(|| {
            $crate::tree::Tag::$node.show().with_leaves($self.$field.iter().map(|val| val.show_prettier($map)))
        })
    };
}

impl_show! {
    functions   => Function  => Functions  => DFunction,
    constants   => Constant  => Constants  => DConstant,
    statics     => Static    => Statics    => DStatic,
    type_alias  => TypeAlias => TypeAliass => DTypeAlias,
    macros_decl => MacroDecl => MacroDecls => DMacroDecl,
    macros_func => MacroFunc => MacroFuncs => DMacroFunc,
    macros_attr => MacroAttr => MacroAttrs => DMacroAttr,
    macros_derv => MacroDerv => MacroDervs => DMacroDerv,
    traits      => Trait     => Traits     => DTrait,
    structs     => Struct    => Structs    => DStruct,
    unions      => Union     => Unions     => DUnion,
    enums       => Enum      => Enums      => DEnum,
}

/// generate id wrapper types for simple items
macro_rules! gen_simple_items {
    ($( $name:ident => $tag:ident => $kind:ident , )+ ) => {$(
        #[derive(Debug, Serialize, Deserialize)] pub struct $name { pub id: ID, }
        impl $name { pub fn new(id: ID) -> Self { Self { id } } }
        impl Show for $name {
            fn show(&self) -> DocTree { self.id.show() }
            fn show_prettier(&self, map: &IDMap) -> DocTree {
                // node!($show, map.path(&self.id, ItemKind::$kind))
                node!(@name $tag: map, &self.id)
            }
        }
    )+};
}

gen_simple_items! {
    DFunction  => Function  => Function,
    DConstant  => Constant  => Constant,
    DStatic    => Static    => Static,
    DTypeAlias => TypeAlias => TypeAlias,
    DMacroDecl => MacroDecl => Macro,
    DMacroFunc => MacroFunc => Macro,
    DMacroAttr => MacroAttr => ProcAttribute,
    DMacroDerv => MacroDerv => ProcDerive,
}

// TODO:
// *  structs, enums, and enum variants: [non_exhaustive]
//
// [non_exhaustive]: https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
