use rustdoc_types::*;
use std::collections::HashMap;

mod display;

#[allow(non_snake_case)]
mod id;
pub use id::{IDs, IdToID, SliceToIds, ID};

pub type Arr<T> = Box<[T]>;
/// Use `IDs::default()` instead of `IDs::new()` to create an empty IDs.
// pub type Vec<T> = std::sync::Arc<[T]>;
// pub type Arc<T> = std::sync::Arc<T>;

// Crate.index
pub type IndexMap = HashMap<Id, Item>;
// Crate.paths
pub type PathMap = HashMap<Id, ItemSummary>;

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
    pub imports: Vec<Import>,
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
                        return Some((id.clone(), items.as_slice()));
                    }
                }
                None
            })
            .expect("root module not found");
        Self::new_inner(id.to_ID(), root, &doc.index)
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
}

#[derive(Default)]
pub struct DImpl {
    pub inherent: IDs,
    pub trait_: IDs,
    pub auto: IDs,
    pub blanket: IDs,
}
impl DImpl {
    fn new(ids: &[Id], index: &IndexMap) -> Box<Self> {
        if ids.is_empty() {
            return Default::default();
        }
        let [mut inherent, mut trait_, mut auto, mut blanket]: [Vec<ID>; 4] = Default::default();
        for Id(id) in ids {
            if id.starts_with("a:") {
                auto.push(id.to_ID());
            } else if id.starts_with("b:") {
                blanket.push(id.to_ID());
            } else {
                let id = Id(id.clone());
                if let Some(item) = index.get(&id) {
                    if let ItemEnum::Impl(impl_) = &item.inner {
                        if impl_.trait_.is_none() {
                            inherent.push(id.into_ID());
                        } else {
                            trait_.push(id.into_ID());
                        }
                    } else {
                        warn!("{id:?} in Crate's index doesn't refer to an impl item");
                    }
                } else {
                    warn!("the impl with {id:?} not found in Crate's index");
                }
            }
        }
        Box::new(DImpl {
            inherent: inherent.into(),
            trait_: trait_.into(),
            auto: auto.into(),
            blanket: blanket.into(),
        })
    }
    fn is_empty(&self) -> bool {
        self.inherent.is_empty()
            && self.trait_.is_empty()
            && self.auto.is_empty()
            && self.blanket.is_empty()
    }
}

// TODO:
// *  structs, enums, and enum variants: [non_exhaustive]
//
// [non_exhaustive]: https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute

pub struct DStruct {
    pub id: ID,
    pub fields: IDs,
    pub contain_private_fields: bool,
    pub impls: Box<DImpl>,
}
impl DStruct {
    fn new(id: ID, item: &Struct, index: &IndexMap) -> Self {
        let mut contain_private_fields = false;
        let fields = match &item.kind {
            StructKind::Unit => IDs::default(),
            StructKind::Tuple(fields) => fields
                .iter()
                .filter_map(|f| {
                    let id = f.as_ref().map(|id| id.to_ID());
                    if id.is_none() {
                        contain_private_fields = true;
                    }
                    id
                })
                .collect(),
            StructKind::Plain {
                fields,
                fields_stripped,
            } => {
                contain_private_fields = *fields_stripped;
                fields.to_ids()
            }
        };
        let impls = DImpl::new(&item.impls, index);
        DStruct {
            id,
            fields,
            contain_private_fields,
            impls,
        }
    }
}

pub struct DUnion {
    pub id: ID,
    pub fields: IDs,
    pub impls: Box<DImpl>,
}
impl DUnion {
    fn new(id: ID, item: &Union, index: &IndexMap) -> Self {
        DUnion {
            id,
            fields: item.fields.to_ids(),
            impls: DImpl::new(&item.impls, index),
        }
    }
}

pub struct DEnum {
    pub id: ID,
    pub variants: IDs,
    // variants_stripped: bool, -> Does this really make sense?
    pub impls: Box<DImpl>,
}
impl DEnum {
    fn new(id: ID, item: &Enum, index: &IndexMap) -> Self {
        DEnum {
            id,
            variants: item.variants.to_ids(),
            impls: DImpl::new(&item.impls, index),
        }
    }
}

pub struct DTrait {
    pub id: ID,
    pub types: IDs,
    pub constants: IDs,
    pub functions: IDs,
    pub implementations: IDs,
}
impl DTrait {
    fn new(id: ID, item: &Trait, index: &IndexMap) -> Self {
        let [mut types, mut constants, mut functions]: [Vec<ID>; 3] = Default::default();
        let trait_id = &id;
        for id in &item.items {
            if let Some(assoc) = index.get(id) {
                let id = id.to_ID(); // id == assoc.id
                match &assoc.inner {
                    ItemEnum::AssocType { .. } => types.push(id),
                    ItemEnum::AssocConst { .. } => constants.push(id),
                    ItemEnum::Function(_) => functions.push(id),
                    _ => warn!(
                        "`{id}` should refer to an associated item \
                         (type/constant/function) in Trait `{trait_id}`"
                    ),
                }
            } else {
                warn!("the trait item {id:?} not found in Crate's index");
            }
        }
        DTrait {
            id,
            types: types.into(),
            constants: constants.into(),
            functions: functions.into(),
            implementations: item.implementations.to_ids(),
        }
    }
}

/// generate id wrapper types for simple items
macro_rules! gen_simple_items {
    ($($name:ident),+) => {$(
        #[derive(Debug)] pub struct $name { pub id: ID, }
        impl $name { fn new(id: ID) -> Self { Self { id } } }
    )+};
}
gen_simple_items!(DFunctions, DStatic, DConstant);

pub struct DTypeAlias {
    pub id: ID,
    /// points to a source type resolved as path
    ///
    /// A type alias may point to non-path-based type though.
    pub source_path: Option<ID>,
}
impl DTypeAlias {
    pub fn new(id: ID, item: &TypeAlias) -> Self {
        Self {
            id,
            source_path: match &item.type_ {
                Type::ResolvedPath(Path { id, .. }) => Some(id.to_ID()),
                _ => None,
            },
        }
    }
}

#[derive(Debug)]
pub enum DMacroKind {
    Declarative,
    ProcFunction,
    ProcAttribute,
    ProcDerive,
}
#[derive(Debug)]
pub struct DMacro {
    pub id: ID,
    pub kind: DMacroKind,
}
impl DMacro {
    pub fn new(id: ID, kind: DMacroKind) -> Self {
        Self { id, kind }
    }
}
