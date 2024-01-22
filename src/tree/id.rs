use rustdoc_types::Id;

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
