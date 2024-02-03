mod impls;
pub use impls::DImpl;

mod structs;
pub use structs::DStruct;

mod unions;
pub use unions::DUnion;

mod enums;
pub use enums::DEnum;

mod traits;
pub use traits::DTrait;

mod imports;

use super::IDMap;
use crate::tree::{
    impls::show::{DocTree, Show},
    IdToID, IndexMap, ID,
};
use rustdoc_types::{Crate, Id, Item, ItemEnum, MacroKind, Module};
use std::ops::Not;

/// Module tree with structural items.
/// All the items only carry ids without actual data.
// NOTE: small improvement by turning all the types of fields
// from Vec to Arr after instantiation.
#[derive(Default)]
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
        let index = map.indexmap();
        let (id, root) = index
            .iter()
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
        let mut dmod = Self::new_inner(id, root, index);
        dmod.sort_by_name(map);
        dmod
    }

    fn new_inner(id: ID, inner_items: &[Id], index: &IndexMap) -> Self {
        let mut dmod = DModule {
            id,
            ..Default::default()
        };
        dmod.extract_items(inner_items, index);
        dmod
    }

    fn extract_items(&mut self, inner_items: &[Id], index: &IndexMap) {
        for item_id in inner_items {
            match index.get(item_id) {
                Some(item) => self.append(item, index),
                None => warn!("the local item {item_id:?} not found in Crate's index"),
            }
        }
    }

    fn append(&mut self, item: &Item, index: &IndexMap) {
        use ItemEnum::*;
        let id = item.id.to_ID();
        match &item.inner {
            Module(item) => self.modules.push(Self::new_inner(id, &item.items, index)),
            Struct(item) => self.structs.push(DStruct::new(id, item, index)),
            Union(item) => self.unions.push(DUnion::new(id, item, index)),
            Enum(item) => self.enums.push(DEnum::new(id, item, index)),
            Trait(item) => self.traits.push(DTrait::new(id, item, index)),
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
            Import(import) => imports::parse_import(id, import, index, self),
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
            self.modules.iter().map(DModule::show)
            $(
                .chain( impl_show!(@show $field $node $fty self map) )
            )+
        )
    }

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        node!(Module: map, &self.id).with_leaves(
            self.modules.iter().map(|m| m.show_prettier(map))
            $(
                .chain( impl_show!(@pretty $field $node self map) )
            )+
        )
    }
}

impl DModule {
    /// The main tree view as public items in module tree.
    pub fn item_tree(&self, map: &IDMap) -> DocTree {
        node!(Module: map, &self.id).with_leaves(
            self.modules.iter().map(|m| m.item_tree(map))
            $(
                .chain(self.$field.iter().map(|item| {
                    node!(@name $tag : map, &item.id)
                }))
            )+
        )
    }

    /// sort items in their item types by name
    fn sort_by_name(&mut self, map: &IDMap) {
        self.modules.sort_unstable_by(|a, b| map.name(&a.id).cmp(&map.name(&b.id)));
        self.modules.iter_mut().for_each(|m| m.sort_by_name(map));
        $(self.$field.sort_unstable_by(|a, b| map.name(&a.id).cmp(&map.name(&b.id)));)+
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
    structs     => Struct    => Structs    => DStruct,
    unions      => Union     => Unions     => DUnion,
    enums       => Enum      => Enums      => DEnum,
    traits      => Trait     => Traits     => DTrait,
    functions   => Function  => Functions  => DFunction,
    constants   => Constant  => Constants  => DConstant,
    statics     => Static    => Statics    => DStatic,
    type_alias  => TypeAlias => TypeAliass => DTypeAlias,
    macros_decl => MacroDecl => MacroDecls => DMacroDecl,
    macros_func => MacroFunc => MacroFuncs => DMacroFunc,
    macros_attr => MacroAttr => MacroAttrs => DMacroAttr,
    macros_derv => MacroDerv => MacroDervs => DMacroDerv,
}

/// generate id wrapper types for simple items
macro_rules! gen_simple_items {
    ($( $name:ident => $tag:ident => $kind:ident , )+ ) => {$(
        #[derive(Debug)] pub struct $name { pub id: ID, }
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
