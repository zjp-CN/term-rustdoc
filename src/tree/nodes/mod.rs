mod impls;
use std::ops::Not;

pub use impls::DImpl;

mod structs;
pub use structs::DStruct;

mod unions;
pub use unions::DUnion;

mod enums;
pub use enums::DEnum;

mod traits;
pub use traits::DTrait;

mod type_alias;
pub use type_alias::DTypeAlias;

mod imports;

use super::IDMap;
use crate::tree::{
    impls::show::{DocTree, Show},
    IdToID, IndexMap, ID,
};
use rustdoc_types::{Crate, Id, Item, ItemEnum, ItemKind, MacroKind, Module};

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
    pub functions: Vec<DFunction>,
    pub traits: Vec<DTrait>,
    pub constants: Vec<DConstant>,
    pub statics: Vec<DStatic>,
    pub type_alias: Vec<DTypeAlias>,
    pub imports: Vec<()>,
    pub macros_decl: Vec<DMacroDecl>,
    pub macros_func: Vec<DMacroFunc>,
    pub macros_attr: Vec<DMacroAttr>,
    pub macros_derv: Vec<DMacroDerv>,
}

impl DModule {
    pub fn new(doc: &Crate) -> Self {
        // root module/crate name
        let (id, root) = doc
            .index
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
        Self::new_inner(id, root, &doc.index)
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
            Function(_) => self.functions.push(DFunction::new(id)),
            Trait(item) => self.traits.push(DTrait::new(id, item, index)),
            Constant(_) => self.constants.push(DConstant::new(id)),
            Static(_) => self.statics.push(DStatic::new(id)),
            TypeAlias(item) => self.type_alias.push(DTypeAlias::new(id, item)),
            Macro(_) => self.macros_decl.push(DMacroDecl::new(id)),
            ProcMacro(proc) => match proc.kind {
                MacroKind::Bang => self.macros_func.push(DMacroFunc::new(id)),
                MacroKind::Attr => self.macros_attr.push(DMacroAttr::new(id)),
                MacroKind::Derive => self.macros_derv.push(DMacroDerv::new(id)),
            },
            // Primitive(_) => todo!(),
            _ => (),
        }
    }
}

macro_rules! chain {
    ($field:ident, $node:literal, $self:ident, $map:ident) => {
        $self.$field.is_empty().not().then(|| {
            $node
                .show()
                .with_leaves($self.$field.iter().map(|val| val.show_prettier($map)))
        })
    };
}

/// To a recursive tree displayed with ids as nodes.
impl Show for DModule {
    fn show(&self) -> DocTree {
        format!("[mod] {}", self.id).show().with_leaves(
            self.modules.iter().map(DModule::show).chain([
                "Structs"
                    .show()
                    .with_leaves(self.structs.iter().map(DStruct::show)),
                "Unions"
                    .show()
                    .with_leaves(self.unions.iter().map(DUnion::show)),
                "Enums"
                    .show()
                    .with_leaves(self.enums.iter().map(DEnum::show)),
                "Traits"
                    .show()
                    .with_leaves(self.traits.iter().map(DTrait::show)),
                "Functions"
                    .show()
                    .with_leaves(self.functions.iter().map(DFunction::show)),
                "Constants"
                    .show()
                    .with_leaves(self.constants.iter().map(DConstant::show)),
                "Statics"
                    .show()
                    .with_leaves(self.statics.iter().map(DStatic::show)),
                "Macros - Declarative"
                    .show()
                    .with_leaves(self.macros_decl.iter().map(DMacroDecl::show)),
                "Macro - Function"
                    .show()
                    .with_leaves(self.macros_func.iter().map(DMacroFunc::show)),
                "Macro - Attribute"
                    .show()
                    .with_leaves(self.macros_attr.iter().map(DMacroAttr::show)),
                "Macro - Derive"
                    .show()
                    .with_leaves(self.macros_derv.iter().map(DMacroDerv::show)),
            ]),
        )
    }

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        node!("[mod] {}", map.path(&self.id, ItemKind::Module)).with_leaves(
            self.modules
                .iter()
                .map(|m| m.show_prettier(map))
                .chain(chain!(structs, "Structs", self, map))
                .chain(chain!(unions, "Unions", self, map))
                .chain(chain!(enums, "Enums", self, map))
                .chain(chain!(traits, "Traits", self, map))
                .chain(chain!(functions, "Functions", self, map))
                .chain(chain!(constants, "Constants", self, map))
                .chain(chain!(statics, "Statics", self, map))
                .chain(chain!(macros_decl, "Macros - Declarative", self, map))
                .chain(chain!(macros_func, "Macros - Function", self, map))
                .chain(chain!(macros_attr, "Macros - Attribute", self, map))
                .chain(chain!(macros_derv, "Macros - Derive", self, map)),
        )
    }
}

/// generate id wrapper types for simple items
macro_rules! gen_simple_items {
    ($($name:ident => $show:literal -> $kind:ident),+ $(,)?) => {$(
        #[derive(Debug)] pub struct $name { pub id: ID, }
        impl $name { pub fn new(id: ID) -> Self { Self { id } } }
        impl Show for $name {
            fn show(&self) -> DocTree { self.id.show() }
            fn show_prettier(&self, map: &IDMap) -> DocTree {
                node!($show, map.path(&self.id, ItemKind::$kind))
            }
        }
    )+};
}
gen_simple_items! {
    DFunction => "[fn] {}" -> Function ,
    DStatic => "[static] {}" -> Static,
    DConstant => "[constant] {}" -> Constant,
    DMacroDecl => "[macro decl] {}" -> Macro,
    DMacroFunc => "[macro func] {}" -> Macro,
    DMacroAttr => "[macro attr] {}" -> ProcAttribute,
    DMacroDerv => "[macro derv] {}" -> ProcDerive,
}

// TODO:
// *  structs, enums, and enum variants: [non_exhaustive]
//
// [non_exhaustive]: https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
