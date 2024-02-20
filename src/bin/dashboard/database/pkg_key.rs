use super::{CachedDocInfo, Features};
use crate::local_registry::PkgNameVersion;
use serde::{Deserialize, Serialize};
use std::{path::Path, time::SystemTime};

/// The key in doc db file.
#[derive(Deserialize, Serialize, Debug)]
pub struct PkgKey {
    name_ver: PkgNameVersion,
    /// features enabled/used when the doc is compiled
    /// TODO: for now, we haven't supported feature selection.
    features: Features,
}

impl PkgKey {
    pub fn into_cached_info(self, parent: &Path) -> CachedDocInfo {
        CachedDocInfo {
            db_file: parent.join(&*self.name_ver.doc_db_file_name()),
            pkg: self,
            created: SystemTime::now(),
        }
    }

    pub fn new(name_ver: PkgNameVersion) -> PkgKey {
        PkgKey {
            name_ver,
            features: Features::Default,
        }
    }
}

impl redb::RedbValue for PkgKey {
    type SelfType<'a> = PkgKey;

    type AsBytes<'a> = Vec<u8>;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        bincode::serde::decode_from_slice(data, bincode::config::standard())
            .unwrap()
            .0
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        bincode::serde::encode_to_vec(value, bincode::config::standard()).unwrap()
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new("PkgKey")
    }
}

impl redb::RedbKey for PkgKey {
    fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
        data1.cmp(data2)
    }
}
