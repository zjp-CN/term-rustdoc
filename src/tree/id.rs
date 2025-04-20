#![allow(non_snake_case)]
use super::{DModule, DocTree, Show};
use crate::type_name::style::{long, long_path};
use crate::util::{join_path, xformat, XString};
use rustdoc_types::{Crate, Id, Item, ItemEnum, ItemKind, ItemSummary, Target};
use std::{borrow::Borrow, collections::HashMap};

pub type IDs = Box<[Id]>;

/// This is usually used behind a shared reference.
/// For owned version, use [`CrateDoc`][super::CrateDoc] instead.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IDMap {
    krate: Crate,
    dmod: DModule,
}

impl IDMap {
    pub fn new(krate: Crate) -> IDMap {
        let mut map = IDMap {
            krate,
            // placeholder for DModule: we'll construct it at once
            dmod: DModule::default(),
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
        let (triple, target_features) = Default::default();
        let target = Target {
            triple,
            target_features,
        };
        IDMap {
            krate: Crate {
                root: Id(0),
                crate_version,
                includes_private,
                index,
                paths,
                external_crates,
                format_version,
                target,
            },
            dmod: DModule::default(),
        }
    }
}

// Crate.index
pub type IndexMap = HashMap<Id, Item>;
// Crate.paths
pub type PathMap = HashMap<Id, ItemSummary>;

impl IDMap {
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
    pub fn get_doc(&self, id: &Id) -> Option<&str> {
        self.get_item(id).and_then(|item| match &item.inner {
            ItemEnum::Use(item) => {
                if let Some(inner_id) = item.id.as_ref() {
                    if let Some(reexport_item) = self.get_item(inner_id) {
                        if matches!(reexport_item.inner, ItemEnum::Use(_)) {
                            error!(
                                "Reexport item with {id:?} shouldn't \
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
    pub fn get_item(&self, id: &Id) -> Option<&Item> {
        self.indexmap().get(id)
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
    pub fn name(&self, id: &Id) -> XString {
        if let Some(item) = self.get_item(id) {
            let name = item.name.as_deref().map(XString::from);
            name.unwrap_or_else(|| item_name(item).unwrap_or_else(|| xformat!("{id:?}")))
        } else if let Some(path) = self.get_path(id) {
            path.path
                .last()
                .map(|p| p.into())
                .unwrap_or_else(|| xformat!("{id:?}"))
        } else {
            xformat!("{id:?}")
        }
    }

    /// Since there will be reexported item id, we have to check the real defining id.
    pub fn is_same_id(&self, src: &Id, target: &Id) -> bool {
        if src == target {
            info!("found exactly same id {src:?}");
            return true;
        }
        self.get_item(src)
            .map(|item| match &item.inner {
                // FIXME: check id for primitive types
                ItemEnum::Use(x) => {
                    let res = x.id.map(|id| id == *target).unwrap_or(false);
                    if res {
                        info!(?src, ?target, "found same id through reexport item");
                    }
                    res
                }
                _ => false,
            })
            .unwrap_or(false)
    }
}

/// Deduce the name from its item type.
fn item_name(item: &Item) -> Option<XString> {
    match &item.inner {
        ItemEnum::Impl(item) => {
            let implementor = item
                .blanket_impl
                .as_ref()
                .map_or_else(|| long(&item.for_), long);
            if let Some(trait_) = item.trait_.as_ref().map(long_path) {
                Some(xformat!("{implementor}: {trait_}"))
            } else {
                Some(xformat!("{implementor}"))
            }
        }
        ItemEnum::Use(item) => Some(item.name.as_str().into()),
        _ => None,
    }
}

/// Get the external item path only based on PathMap.
impl IDMap {
    pub fn get_path(&self, id: &Id) -> Option<&ItemSummary> {
        self.pathmap().get(id)
    }

    // fn use_path(&self, id: &Id, f: impl FnOnce(&ItemSummary) -> XString) -> Option<XString> {
    //     self.get_path(&id.0).map(f)
    // }

    /// If the id doesn't refer to an ItemSummary, emit a warn and use the id as the result.
    fn use_path_well(&self, id: &Id, f: impl FnOnce(&ItemSummary) -> XString) -> XString {
        match self.get_path(id).map(f) {
            Some(s) => s,
            None => {
                warn!("{id:?} doesn't refer to an Item in IndexMap");
                xformat!("{id:?}")
            }
        }
    }

    /// Like `path`, but with strict item kind checking.
    /// If the id doesn't refer to an ItemSummary with exact given kind, emit a warn.
    pub fn path_with_kind_check<K>(&self, id: &Id, kind: K) -> XString
    where
        K: Borrow<ItemKind>,
    {
        self.use_path_well(id, move |item| {
            let kind = kind.borrow();
            if &item.kind != kind {
                warn!(
                    "{id:?} in PathMap is found as {:?}, but {kind:?} is required",
                    item.kind
                );
            }
            join_path(&item.path.clone())
        })
    }

    /// Returns the full path if it exists, or name if it exists or id if neither exists.
    pub fn path(&self, id: &Id) -> XString {
        self.get_path(id)
            .map(|item| join_path(&item.path.clone()))
            .unwrap_or_else(|| self.name(id))
    }

    /// Like `path`, but returns the choice for name/id fallback an Err variant.
    pub fn path_or_name(&self, id: &Id) -> Result<XString, XString> {
        self.get_path(id)
            .map(|item| join_path(&item.path))
            .ok_or_else(|| self.name(id))
    }
}
