use crate::util::{CompactStringExt, XString};
use rustdoc_types::{Crate, Id, Item, ItemKind, ItemSummary};
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

// Crate.index
pub type IndexMap = HashMap<Id, Item>;
// Crate.paths
pub type PathMap = HashMap<Id, ItemSummary>;

pub struct IDMap<'krate> {
    pub index: &'krate IndexMap,
    pub paths: &'krate PathMap,
    id_buffer: RefCell<String>,
}

impl IDMap<'_> {
    pub fn from_crate(krate: &Crate) -> IDMap<'_> {
        IDMap {
            index: &krate.index,
            paths: &krate.paths,
            id_buffer: Default::default(),
        }
    }

    /// Use [`rustdoc_types::Id`] in a buffered way in hot querys.
    pub fn use_id<T>(&self, id: &str, f: impl FnOnce(&Id) -> T) -> T {
        // idbuf always serves as Id used in a query
        let mut buf = self.id_buffer.take();
        buf.clear();
        buf.push_str(id);

        let id = Id(buf);
        let val = f(&id);

        // put the buffer back to be used next time
        self.id_buffer.replace(id.0);

        val
    }

    pub fn get_item(&self, id: &str) -> Option<&Item> {
        self.use_id(id, |id| self.index.get(id))
    }

    pub fn get_path(&self, id: &str) -> Option<&ItemSummary> {
        self.use_id(id, |id| self.paths.get(id))
    }

    // fn use_item(&self, id: &Id, f: impl FnOnce(&Item) -> XString) -> Option<XString> {
    //     self.get_item(&id.0).map(f)
    // }
    //
    // fn use_path(&self, id: &Id, f: impl FnOnce(&ItemSummary) -> XString) -> Option<XString> {
    //     self.get_path(&id.0).map(f)
    // }

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

    /// If the id doesn't refer to an Item, emit a warn and use the id as the result.
    pub fn item_node<'id, S: IdAsStr>(
        &'id self,
        ids: &'id [S],
        mut f: impl 'id + FnMut(&str, &Item) -> XString,
    ) -> impl 'id + Iterator<Item = XString> {
        ids.iter().map(move |id| {
            let id = id.id_str();
            self.use_item_well(id, |item| f(id, item))
        })
    }

    /// If the id doesn't refer to an ItemSummary with exact given kind,
    /// emit a warn and use the id as the result.
    pub fn path_node<'id, S: 'id + ?Sized + IdAsStr>(
        &'id self,
        ids: impl 'id + IntoIterator<Item = &'id S>,
        kind: ItemKind,
    ) -> impl 'id + Iterator<Item = XString> {
        ids.into_iter().map(move |id| self.path(id, &kind))
    }

    pub fn path<S: ?Sized + IdAsStr, K: Borrow<ItemKind>>(&self, id: &S, kind: K) -> XString {
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
