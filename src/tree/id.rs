#![allow(non_snake_case)]
use super::{DModule, DocTree, Show};
use crate::util::{xformat, CompactStringExt, XString};
use itertools::intersperse;
use rustdoc_types::{
    Crate, GenericArg, GenericArgs, Id, Item, ItemEnum, ItemKind, ItemSummary, Path, Type,
};
use std::{borrow::Borrow, cell::RefCell, collections::HashMap};

/// basic impls for ID
mod impls;

pub type IDs = Box<[ID]>;

#[derive(
    Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[repr(transparent)]
pub struct ID {
    pub id: XString,
}

pub trait IdToID: Sized {
    fn to_ID(&self) -> ID;
    fn into_ID(self) -> ID;
}

impl IdToID for Id {
    fn to_ID(&self) -> ID {
        self.0.to_ID()
    }

    fn into_ID(self) -> ID {
        String::into_ID(self.0)
    }
}

impl IdToID for String {
    fn to_ID(&self) -> ID {
        ID {
            id: self.as_str().into(),
        }
    }

    fn into_ID(self) -> ID {
        ID {
            id: XString::from_string_buffer(self),
        }
    }
}

impl IdToID for &str {
    fn to_ID(&self) -> ID {
        ID::new(self)
    }

    fn into_ID(self) -> ID {
        ID::new(self)
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
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IDMap {
    krate: Crate,
    dmod: DModule,
    id_buffer: RefCell<String>,
}

impl IDMap {
    pub fn new(krate: Crate) -> IDMap {
        let mut map = IDMap {
            krate,
            // placeholder for DModule: we'll construct it at once
            dmod: DModule::default(),
            id_buffer: RefCell::new(String::with_capacity(24)),
        };
        map.dmod = DModule::new(&map);
        info!("IDMap and DModule ready");
        map
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
    /// FIXME: show_prettier should be renamed
    pub fn dmodule_show_prettier(&self) -> DocTree {
        self.dmod.show_prettier(self)
    }

    /// This is the default tree view for most cases.
    pub fn dmodule_item_tree(&self) -> DocTree {
        self.dmod.item_tree(self)
    }
}

// Documentation on an item.
impl IDMap {
    pub fn get_doc(&self, id: &str) -> Option<&str> {
        self.get_item(id).and_then(|item| match &item.inner {
            ItemEnum::Import(item) => {
                if let Some(inner_id) = item.id.as_ref() {
                    if let Some(reexport_item) = self.get_item(&inner_id.0) {
                        if matches!(reexport_item.inner, ItemEnum::Import(_)) {
                            error!(
                                "Reexport item with Id({id}) shouldn't \
                                 recursively contains another Import.\n{item:?} "
                            );
                        } else {
                            return reexport_item.docs.as_deref();
                        }
                    }
                }
                None
            }
            _ => item.docs.as_deref(),
        })
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

    // /// If the id doesn't refer to an Item, emit a warn and use the id as the result.
    // fn use_item_well(&self, id: &str, f: impl FnOnce(&Item) -> XString) -> XString {
    //     match self.get_item(id).map(f) {
    //         Some(s) => s,
    //         None => {
    //             warn!("Id({id}) doesn't refer to an Item in IndexMap");
    //             XString::from(id)
    //         }
    //     }
    // }

    /// * If the id refers to an Item with a name, use the name;
    ///     * if name is None, try getting the name depending on item type (reexported local items
    ///       may hit this);
    /// * If id isn't in IndexMap, try searching the PathMap for last path component (reexported
    ///   external items may hit this);
    /// * otherwise id.
    pub fn name<S>(&self, id: &S) -> XString
    where
        S: ?Sized + IdAsStr,
    {
        let id = id.id_str();
        if let Some(item) = self.get_item(id) {
            let name = item.name.as_deref().map(XString::from);
            name.unwrap_or_else(|| item_name(item).unwrap_or_else(|| id.into()))
        } else if let Some(path) = self.get_path(id) {
            path.path.last().map(|p| p.as_str()).unwrap_or(id).into()
        } else {
            id.into()
        }
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
        ItemEnum::Import(item) => Some(item.name.as_str().into()),
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
                let arg: XString =
                    intersperse(args.iter().filter_map(generic_arg_name), COMMA).collect();
                Some(xformat!("{name}<{arg}>"))
            }
        }
        Some(GenericArgs::Parenthesized { inputs, output }) => {
            let args: XString = intersperse(inputs.iter().filter_map(type_name), COMMA).collect();
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

    /// Like `path`, but with strict item kind checking.
    /// If the id doesn't refer to an ItemSummary with exact given kind, emit a warn.
    pub fn path_with_kind_check<S, K>(&self, id: &S, kind: K) -> XString
    where
        S: ?Sized + IdAsStr,
        K: Borrow<ItemKind>,
    {
        let id = id.id_str();
        self.use_path_well(id, move |item| {
            let kind = kind.borrow();
            if &item.kind != kind {
                warn!(
                    "Id({id}) in PathMap is found as {:?}, but {kind:?} is required",
                    item.kind
                );
            }
            item.path.join_compact("::")
        })
    }

    /// Returns the full path if it exists, or name if it exists or id if neither exists.
    pub fn path(&self, id: &str) -> XString {
        self.get_path(id)
            .map(|item| item.path.join_compact("::"))
            .unwrap_or_else(|| self.name(id))
    }

    /// Like `path`, but returns the choice for name/id fallback an Err variant.
    pub fn path_or_name(&self, id: &str) -> Result<XString, XString> {
        self.get_path(id)
            .map(|item| item.path.join_compact("::"))
            .ok_or_else(|| self.name(id))
    }
}
