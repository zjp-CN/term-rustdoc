use super::Features;
use crate::local_registry::PkgNameVersion;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The key in doc db file.
///
/// NOTE: the reason why PkgKey doesn't implement PartialOrd and Ord is
/// we can't directly compare the version string, and the parsed Version
/// should be stored outside this struct.
#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
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
    pub fn new_with_default_feature(name_ver: PkgNameVersion) -> PkgKey {
        PkgKey {
            name_ver,
            features: Features::Default,
        }
    }

    pub fn name(&self) -> &str {
        self.name_ver.name()
    }

    pub fn ver_str(&self) -> &str {
        self.name_ver.ver_str()
    }

    /// Parse the version. When the version can't be parsed, this will return a `0.0.0` version.
    pub fn version(&self) -> Version {
        self.ver_str()
            .parse()
            .map_err(|err| error!("Failed to parse the version in {self:?}:\n{err}"))
            .unwrap_or(Version::new(0, 0, 0))
    }

    pub fn features(&self) -> &Features {
        &self.features
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
        super::util::decode(data).unwrap()
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        super::util::encode(value).unwrap()
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
