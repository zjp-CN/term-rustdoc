use crate::Result;
use std::{collections::BTreeMap, path::Path};
use term_rustdoc::util::{HashMap, Hasher, XString};

type ControlledFeatures = BTreeMap<XString, FeatureControlledByUsers>;

/// Only contains/enables features that are able to be controlled by user.
/// Thus `a_dep/feature` will be filtered out.
// #[derive(Debug)]
pub(super) struct FeaturesControlledByUsers {
    pub features: ControlledFeatures,
    pub manifest: ManifestFeatures,
}

type FeaturesMap<'a, 'b> = HashMap<&'a str, cargo_toml::features::Feature<'b>>;

impl FeaturesControlledByUsers {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let manifest = ManifestFeatures::from_path(path.as_ref())?;
        let features = BTreeMap::new();
        let mut features = FeaturesControlledByUsers { features, manifest };
        features.split(|map, feat| {
            feat.extend(
                map.keys()
                    .map(|&k| (XString::from(k), FeatureControlledByUsers::default())),
            )
        });
        features.enable("default");
        debug!(?features.features);
        Ok(features)
    }

    fn split(&mut self, f: impl Fn(&FeaturesMap, &mut ControlledFeatures)) {
        f(
            &self.manifest.borrow_dependent().features,
            &mut self.features,
        );
    }

    fn enable(&mut self, key: &str) {
        fn enable_recursive(
            map: &FeaturesMap,
            feat: &mut ControlledFeatures,
            related: &str,
            key: &str,
        ) {
            if let Some(feature) = map.get(related) {
                for &f in &feature.enables_features {
                    // push key to enabled_by vec
                    if let Some(indirect) = feat.get_mut(f) {
                        indirect.enabled_by(key);
                    }
                    // recursive push key
                    enable_recursive(map, feat, f, key);
                }
            }
        }

        if let Some(f) = self.features.get_mut(key) {
            f.specify_enabled = true;
            self.split(|map, feat| enable_recursive(map, feat, key, key));
        }
    }

    fn disable(&mut self, key: &str) {
        fn disable_recursive(
            map: &FeaturesMap,
            feat: &mut ControlledFeatures,
            related: &str,
            key: &str,
        ) {
            if let Some(feature) = map.get(related) {
                for &f in &feature.enables_features {
                    // pop off key from enabled_by vec
                    if let Some(indirect) = feat.get_mut(f) {
                        indirect.disabled_by(key);
                    }
                    // recursive pop off key
                    disable_recursive(map, feat, f, key);
                }
            }
        }

        if let Some(f) = self.features.get_mut(key) {
            f.specify_enabled = false;
            self.split(|map, feat| disable_recursive(map, feat, key, key));
        }
    }

    pub fn toggle(&mut self, key: &str) {
        if let Some(f) = self.features.get_mut(key) {
            if f.specify_enabled {
                self.disable(key);
            } else {
                self.enable(key);
            }
        }
    }
}

#[cfg(test)]
impl FeaturesControlledByUsers {
    fn list_user_controlled(&self) -> Vec<&str> {
        self.features
            .iter()
            .filter_map(|(k, f)| f.is_enabled().then_some(&**k))
            .collect::<Vec<_>>()
    }

    fn list_specified_enabled(&self) -> Vec<&str> {
        self.features
            .iter()
            .filter_map(|(k, f)| f.specify_enabled.then_some(&**k))
            .collect::<Vec<_>>()
    }

    fn list_recursive(&self) -> Vec<XString> {
        use term_rustdoc::util::xformat;
        let mfeatures = self.manifest.borrow_dependent();
        let feats = &mfeatures.features;
        let mut all = Vec::<XString>::with_capacity(8);
        for feat in self.list_specified_enabled() {
            if let Some(feature) = feats.get(feat) {
                let (map, _) = feature.enables_recursive(feats);
                for (key, f) in map {
                    all.push(key.into());
                    all.extend(f.enables_features.iter().copied().map(XString::from));
                    all.extend(f.enables_deps.iter().flat_map(|(dep, x)| {
                        x.dep_features.iter().map(move |f| xformat!("{dep}/{f}"))
                    }));
                }
            }
        }
        all.sort_unstable();
        all.dedup();
        all
    }

    #[allow(clippy::type_complexity)]
    fn dbg(&mut self, fns: &[(&str, fn(&mut Self))]) {
        for (key, f) in fns {
            f(self);
            println!(
                "{key} => {:?}\ncontrol {:?}\nspecified {:?}\n",
                self.list_recursive(),
                self.list_user_controlled(),
                self.list_specified_enabled()
            );
        }
    }
}

/// The enables only contains features that are keys in the features map.
///
/// So stuff like `a_crate/feature` will not be included because we can't enable them alone.
#[derive(Debug, Default)]
pub(super) struct FeatureControlledByUsers {
    /// Pushing a key to this setting means the feature is enabled by XString;
    /// popping off a key means the feature is disabled by it.
    ///
    /// This field is used to display how the feature is enabled by users' controlling.
    pub enabled_by: Vec<XString>,
    /// The state of the feature is enabled by user-controlling features or not.
    ///
    /// Whether enabled_by vec is empty or not will not affact this value.
    ///
    /// The specify_enabled features are used as part of PkgKey.
    pub specify_enabled: bool,
}

impl FeatureControlledByUsers {
    fn enabled_by(&mut self, feat: &str) {
        // Only push the enabling feature if not exists
        if !self.enabled_by.iter().any(|f| *f == feat) {
            self.enabled_by.push(feat.into());
        }
    }

    fn disabled_by(&mut self, feat: &str) {
        if let Some(pos) = self.enabled_by.iter().position(|f| f.as_str() == feat) {
            self.enabled_by.remove(pos);
        }
    }

    pub fn is_enabled(&self) -> bool {
        !self.enabled_by.is_empty() || self.specify_enabled
    }
}

type Resolver<'a> = cargo_toml::features::Resolver<'a, Hasher>;
type CargoFeatures<'a> = cargo_toml::features::Features<'a, 'a, Hasher>;

// The lifetime of dependent Features comes from a borrow from Manifest.
// I don't want to clone data in Features by defining trivial owned structs,
// thus simply use a self-referential struct instead.
self_cell::self_cell! {
    pub struct ManifestFeatures {
        owner: cargo_toml::Manifest,
        #[covariant]
        dependent: CargoFeatures,
    }
    impl {Debug}
}

impl ManifestFeatures {
    fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let manifest = cargo_toml::Manifest::from_path(path.as_ref())?;
        Ok(Self::new(manifest, |manifest| {
            // ignore inner features the name of which starts with _
            Resolver::new_with_hasher_and_filter(&|_| false).parse(manifest)
        }))
    }

    /// `default` feature doesn't enable anything
    pub fn default_for_nothing(&self) -> bool {
        let default = self.borrow_dependent().features.get("default");
        default
            .map(|f| f.enables_features.is_empty() && f.enables_deps.is_empty())
            .unwrap_or(false)
    }
}

#[test]
#[ignore = "replace TOKIO cargo path on your machine"]
fn parse_features() -> Result<()> {
    const TOKIO: &str =
        "/root/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/tokio-1.35.1/Cargo.toml";
    let mut features = FeaturesControlledByUsers::new(TOKIO)?;
    dbg!(features.features.keys().collect::<Vec<_>>());
    features.dbg(&[
        ("default", |_| {}),
        ("+net", |f| f.enable("net")),
        ("+full", |f| f.enable("full")),
        ("-full", |f| f.disable("full")),
        ("-libc", |f| f.disable("libc")),
        ("+rt-multi-thread", |f| {
            f.enable("rt-multi-thread");
            let enabled_by = f
                .features
                .iter()
                .filter_map(|(k, v)| (!v.enabled_by.is_empty()).then_some((k, &v.enabled_by)))
                .collect::<Vec<_>>();
            dbg!(enabled_by);
        }),
        ("-rt-multi-thread", |f| f.toggle("rt-multi-thread")),
    ]);
    Ok(())
}
