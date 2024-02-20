use super::Features;
use crate::local_registry::PkgNameVersion;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The key in doc db file.
#[derive(Deserialize, Serialize)]
pub struct PkgKey {
    name_ver: PkgNameVersion,
    /// features enabled/used when the doc is compiled
    /// TODO: for now, we haven't supported feature selection.
    features: Features,
}

impl fmt::Debug for PkgKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [name, ver] = self.name_ver.name_ver();
        let features = &self.features;
        if matches!(features, Features::Default) {
            write!(f, "{name}_v{ver}")
        } else {
            write!(f, "{name}_v{ver} [{features:?}]")
        }
    }
}

impl PkgKey {
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
        super::decode(data).unwrap()
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        super::encode(value).unwrap()
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
