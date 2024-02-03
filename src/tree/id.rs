use super::{DModule, DocTree, Show};
use crate::util::{xformat, CompactStringExt, XString};
use rustdoc_types::{
    Crate, GenericArg, GenericArgs, Id, Item, ItemEnum, ItemKind, ItemSummary, Path, Type,
};
use std::{borrow::Borrow, cell::RefCell, collections::HashMap};

// NOTE: potentially small improvements on id by using CompactString,
// but this requires the HashMaps to be indexed by other id types:
// * IndexMap<CompactString, Value, FxHash> can improve the hashing speed and
//   get the value from multiple type keys via Equivalent trait
// * so does the tiny improvement really matter?
// #[repr(transparent)]
// pub struct ID {
//     pub id: XString,
// }
pub type ID = String;
pub type IDs = Box<[ID]>;

pub trait IdToID: Sized {
    fn to_ID(&self) -> ID;
    fn into_ID(self) -> ID;
}

impl IdToID for Id {
    fn to_ID(&self) -> ID {
        self.0.clone()
    }

    fn into_ID(self) -> ID {
        self.0
    }
}

impl IdToID for String {
    fn to_ID(&self) -> ID {
        self.clone()
    }

    fn into_ID(self) -> ID {
        self
    }
}

impl IdToID for &str {
    fn to_ID(&self) -> ID {
        (*self).to_owned()
    }

    fn into_ID(self) -> ID {
        self.to_owned()
    }
}

pub trait SliceToIds {
    fn to_ids(&self) -> IDs;
}
impl<T: IdToID> SliceToIds for [T] {
    fn to_ids(&self) -> IDs {
        self.iter().map(|id| id.to_ID()).collect()
    }
}

pub trait IdAsStr {
    fn id_str(&self) -> &str;
}
impl IdAsStr for str {
    fn id_str(&self) -> &str {
        self
    }
}
impl IdAsStr for ID {
    fn id_str(&self) -> &str {
        self
    }
}
impl IdAsStr for Id {
    fn id_str(&self) -> &str {
        &self.0
    }
}
impl<T: IdAsStr> IdAsStr for &T {
    fn id_str(&self) -> &str {
        T::id_str(self)
    }
}

/// This is usually used behind a shared reference.
/// For owned version, use [`CrateDoc`][super::CrateDoc] instead.
pub struct IDMap {
    krate: Crate,
    dmod: DModule,
    id_buffer: RefCell<String>,
}

impl IDMap {
    pub fn new(doc: Crate) -> IDMap {
        let dmod = DModule::new(&doc);
        IDMap {
            krate: doc,
            dmod,
            id_buffer: RefCell::new(String::with_capacity(24)),
        }
    }

    pub fn dmodule(&self) -> &DModule {
        &self.dmod
    }

    pub fn raw_crate_doc(&self) -> &Crate {
        &self.krate
    }
}

impl Default for IDMap {
    fn default() -> Self {
        let (crate_version, includes_private, index, paths, external_crates, format_version) =
            Default::default();
        IDMap {
            krate: Crate {
                root: rustdoc_types::Id(String::new()),
                crate_version,
                includes_private,
                index,
                paths,
                external_crates,
                format_version,
            },
            dmod: DModule::default(),
            id_buffer: RefCell::default(),
        }
    }
}

// Crate.index
pub type IndexMap = HashMap<Id, Item>;
// Crate.paths
pub type PathMap = HashMap<Id, ItemSummary>;

impl IDMap {
    /// Use  in a buffered way in hot querys.
    pub fn use_id<T>(&self, id: &str, f: impl FnOnce(&Id) -> T) -> T {
        // idbuf always serves as Id used in a query
        let mut buf = self.id_buffer.take();
        buf.clear();
        buf.push_str(id);

        let id = Id(buf);
        let val = f(&id);

        // put the buffer back to use next time
        self.id_buffer.replace(id.0);

        val
    }

    pub fn indexmap(&self) -> &IndexMap {
        &self.krate.index
    }

    pub fn pathmap(&self) -> &PathMap {
        &self.krate.paths
    }
}

/// DModule related.
impl IDMap {
    pub fn dmodule_show_prettier(&self) -> DocTree {
        self.dmod.show_prettier(self)
    }
}

// Documentation on an item.
impl IDMap {
    pub fn get_doc(&self, id: &str) -> Option<&str> {
        self.get_item(id).and_then(|item| item.docs.as_deref())
    }
}

/// Get the shortest item name only based on IndexMap.
impl IDMap {
    pub fn get_item(&self, id: &str) -> Option<&Item> {
        self.use_id(id, |id| self.indexmap().get(id))
    }

    // fn use_item(&self, id: &Id, f: impl FnOnce(&Item) -> XString) -> Option<XString> {
    //     self.get_item(&id.0).map(f)
    // }
    //

    /// If the id doesn't refer to an Item, emit a warn and use the id as the result.
    fn use_item_well(&self, id: &str, f: impl FnOnce(&Item) -> XString) -> XString {
        match self.get_item(id).map(f) {
            Some(s) => s,
            None => {
                warn!("Id({id}) doesn't refer to an Item in IndexMap");
                XString::from(id)
            }
        }
    }

    /// * If the id refers to an Item with a name, use the name;
    /// * if not, try getting the name depending on item type;
    /// * id will be used as the result when name parsing is not yet supported.
    pub fn name<S>(&self, id: &S) -> XString
    where
        S: ?Sized + IdAsStr,
    {
        let id = id.id_str();
        self.use_item_well(id, |item| {
            let name = item.name.as_deref().map(XString::from);
            name.unwrap_or_else(|| item_name(item).unwrap_or_else(|| id.into()))
        })
    }
}

/// Deduce the name from its item type.
fn item_name(item: &Item) -> Option<XString> {
    match &item.inner {
        ItemEnum::Impl(item) => {
            let implementor = item
                .blanket_impl
                .as_ref()
                .and_then(type_name)
                .or_else(|| type_name(&item.for_))
                .unwrap_or_default();
            let trait_ = item.trait_.as_ref().and_then(resolved_path_name)?;
            Some(xformat!("{implementor}: {trait_}"))
        }
        _ => None,
    }
}

const COMMA: XString = XString::new_inline(", ");

fn type_name(ty: &Type) -> Option<XString> {
    match ty {
        Type::ResolvedPath(p) => resolved_path_name(p),
        Type::Generic(t) => Some(t.as_str().into()),
        _ => None,
    }
}

fn resolved_path_name(p: &Path) -> Option<XString> {
    let name = p.name.as_str();
    match p.args.as_deref() {
        Some(GenericArgs::AngleBracketed { args, bindings: _ }) => {
            // FIXME: bindings without args
            if args.is_empty() {
                Some(name.into())
            } else {
                Some(xformat!(
                    "{name}<{}>",
                    args.iter()
                        .filter_map(generic_arg_name)
                        .intersperse(COMMA)
                        .collect::<XString>()
                ))
            }
        }
        Some(GenericArgs::Parenthesized { inputs, output }) => {
            let args = inputs
                .iter()
                .filter_map(type_name)
                .intersperse(COMMA)
                .collect::<XString>();
            let ret = output
                .as_ref()
                .and_then(|t| Some(xformat!(" -> {}", type_name(t)?)))
                .unwrap_or_default();
            Some(xformat!("{name}({args}){ret}"))
        }
        None => Some(name.into()),
    }
}

fn generic_arg_name(arg: &GenericArg) -> Option<XString> {
    match arg {
        GenericArg::Lifetime(life) => Some(life.as_str().into()),
        GenericArg::Type(ty) => type_name(ty),
        GenericArg::Const(_) => None,
        GenericArg::Infer => Some(XString::new_inline("_")),
    }
}

/// Get the external item path only based on PathMap.
impl IDMap {
    pub fn get_path(&self, id: &str) -> Option<&ItemSummary> {
        self.use_id(id, |id| self.pathmap().get(id))
    }

    // fn use_path(&self, id: &Id, f: impl FnOnce(&ItemSummary) -> XString) -> Option<XString> {
    //     self.get_path(&id.0).map(f)
    // }

    /// If the id doesn't refer to an ItemSummary, emit a warn and use the id as the result.
    fn use_path_well(&self, id: &str, f: impl FnOnce(&ItemSummary) -> XString) -> XString {
        match self.get_path(id).map(f) {
            Some(s) => s,
            None => {
                warn!("Id({id}) doesn't refer to an Item in IndexMap");
                XString::from(id)
            }
        }
    }

    /// If the id doesn't refer to an ItemSummary with exact given kind,
    /// emit a warn and use the id as the result.
    pub fn path<S, K>(&self, id: &S, kind: K) -> XString
    where
        S: ?Sized + IdAsStr,
        K: Borrow<ItemKind>,
    {
        self.use_path_well(id.id_str(), move |item| {
            let id = id.id_str();
            let kind = kind.borrow();
            if &item.kind == kind {
                item.path.join_compact("::")
            } else {
                warn!(
                    "Id({id}) in PathMap is found as {:?}, but {kind:?} is required",
                    item.kind
                );
                XString::from(id)
            }
        })
    }
}
