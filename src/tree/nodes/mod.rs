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

mod function;

mod constants;

mod statics;

mod type_alias;
pub use type_alias::DTypeAlias;

mod imports;

mod macros;
pub use macros::{DMacro, DMacroKind};

use crate::tree::{IDs, IdToID, IndexMap, SliceToIds, ID};
use crate::util::{ToCompactString, XString};
use rustdoc_types::{Crate, Id, Item, ItemEnum, MacroKind, Module};
use termtree::Tree;

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
    pub functions: Vec<DFunctions>,
    pub traits: Vec<DTrait>,
    pub constants: Vec<DConstant>,
    pub statics: Vec<DStatic>,
    pub type_alias: Vec<DTypeAlias>,
    pub imports: Vec<()>,
    pub macros: Vec<DMacro>,
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
            Function(_) => self.functions.push(DFunctions::new(id)),
            Trait(item) => self.traits.push(DTrait::new(id, item, index)),
            Constant(_) => self.constants.push(DConstant::new(id)),
            Static(_) => self.statics.push(DStatic::new(id)),
            TypeAlias(item) => self.type_alias.push(DTypeAlias::new(id, item)),
            Macro(_) => self.macros.push(DMacro::new(id, DMacroKind::Declarative)),
            ProcMacro(proc) => self.macros.push(DMacro::new(
                id,
                match proc.kind {
                    MacroKind::Bang => DMacroKind::ProcFunction,
                    MacroKind::Attr => DMacroKind::ProcAttribute,
                    MacroKind::Derive => DMacroKind::ProcDerive,
                },
            )),
            // Primitive(_) => todo!(),
            _ => (),
        }
    }

    /// To a recursive tree displayed with ids as nodes.
    pub fn to_tree(&self) -> Tree<XString> {
        let mut tree = Tree::new(self.id.as_str().to_compact_string());
        tree
    }
}

/// generate id wrapper types for simple items
macro_rules! gen_simple_items {
    ($($name:ident),+) => {$(
        #[derive(Debug)] pub struct $name { pub id: ID, }
        impl $name { pub fn new(id: ID) -> Self { Self { id } } }
    )+};
}
gen_simple_items!(DFunctions, DStatic, DConstant);

// TODO:
// *  structs, enums, and enum variants: [non_exhaustive]
//
// [non_exhaustive]: https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
