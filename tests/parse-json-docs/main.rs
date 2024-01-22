#![feature(lazy_cell)]
use bytesize::ByteSize;
use color_eyre::eyre::{eyre, Result};
use insta::{assert_debug_snapshot as snap, assert_display_snapshot as shot};
use regex::Regex;
use rustc_hash::FxHashMap;
use rustdoc_types::{Crate, Id, Item, ItemKind, ItemSummary};
use std::{
    fmt::Display,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
    sync::LazyLock,
};
use term_rustdoc::{
    tree::DModule,
    util::{xformat, CompactStringExt, XString},
};

static INTEGRATION: LazyLock<JsonDoc> = LazyLock::new(|| {
    tracing_subscriber::fmt::init();
    get_integration_json_doc().expect("failed to get the json doc of tests/integration crate")
});

pub struct PatternId {
    // path: Regex,
    index: Regex,
}
impl PatternId {
    // const PATH: &'static str = r"(?<crate>\d+):(?<item>\d+):(?<name>\d+)";
    const INDEX: &'static str =
        r"((?<impl>[[:lower:]]):)?(?<crate>\d+):(?<item>\d+)(:(?<name>\d+))?(-(?<extra>.*))?";
}
static RE: LazyLock<PatternId> = LazyLock::new(|| PatternId {
    // path: Regex::new(PatternId::PATH).expect("PathId regex pattern not built "),
    index: Regex::new(PatternId::INDEX).expect("IndexId regex pattern not built "),
});
struct JsonDoc {
    json: String,
    doc: Crate,
}
impl JsonDoc {
    fn get_path_xstring(&self, id: &Id) -> Option<XString> {
        self.doc.paths.get(id).map(|p| p.path.join_compact("::"))
    }
    fn get_path_string(&self, id: &Id) -> Option<String> {
        self.doc.paths.get(id).map(|p| p.path.join("::"))
    }
    fn get_item_summary(&self, id: &Id) -> Option<&ItemSummary> {
        self.doc.paths.get(id)
    }
    /// An `[IMPL:]CRATE_ID:ITEM_ID[:NAME_ID][-EXTRA]` Id may represent multiple items:
    /// * `CRATE_ID:ITEM_ID[:NAME_ID]`
    /// * `[-EXTRA]`
    /// * `CRATE_ID:ITEM_ID[:NAME_ID]` + `[-EXTRA]`
    fn get_item(&self, idx: &IndexId) -> Vec<(Id, &[String], ItemKind)> {
        use std::fmt::Write;
        let mut paths = Vec::with_capacity(3);
        let mut id = Id(format!("{}:{}", idx.crate_id, idx.item_id));
        if let Some(item) = self.get_item_summary(&id) {
            paths.push((id.clone(), &*item.path, item.kind.clone()))
        }
        if let Some(name_id) = idx.name_id {
            write!(id.0, ":{name_id}").unwrap();
            if let Some(item) = self.get_item_summary(&id) {
                paths.push((id.clone(), &*item.path, item.kind.clone()))
            }
        }
        if let Some(extra) = idx.extra.as_ref() {
            id.0.clear();
            write!(id.0, "{extra}").unwrap();
            if let Some(item) = self.get_item_summary(&id) {
                paths.push((id, &*item.path, item.kind.clone()))
            }
        }
        paths
    }
    /// local crate id is always 0
    fn local_index(&self) -> impl Iterator<Item = (&Id, &Item)> {
        self.doc.index.iter().filter(|(_, item)| item.crate_id == 0)
    }
    /// local crate id is always 0
    fn local_path(&self) -> impl Iterator<Item = (&Id, &ItemSummary)> {
        self.doc.paths.iter().filter(|(_, item)| item.crate_id == 0)
    }
}

fn get_integration_json_doc() -> Result<JsonDoc> {
    let json_path = rustdoc_json::Builder::default()
        .toolchain("nightly")
        // .manifest_path(
        //     "/root/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/syn-2.0.48/Cargo.toml",
        // )
        .all_features(true)
        .target_dir(PathBuf::from_iter(["target", "json-docs"]))
        .manifest_path(PathBuf::from_iter(["tests", "integration", "Cargo.toml"]))
        // .manifest_path("./Cargo.toml")
        // .document_private_items(true)
        .build()?;

    let mut json_file = std::fs::File::open(json_path)?;
    let len = json_file.metadata()?.len() as usize;
    let mut buf = Vec::with_capacity(len);
    json_file.read_to_end(&mut buf)?;
    Ok(JsonDoc {
        doc: serde_json::from_slice(&buf)?,
        json: String::from_utf8(buf)?,
    })
}

#[test]
fn basic_info() -> Result<()> {
    let js @ JsonDoc { json, doc } = &*INTEGRATION;
    let mut paths = doc
        .paths
        .values()
        .filter(|p| p.crate_id == 0) // documented local crate
        .map(|p| {
            format!(
                "{path:<50} [{kind:?}]",
                kind = p.kind,
                path = p.path.join("::")
            )
        })
        .collect::<Vec<_>>();
    paths.sort_unstable();

    // local items
    snap!(paths, @r###"
    [
        "integration                                        [Module]",
        "integration::S                                     [Struct]",
        "integration::Trait                                 [Trait]",
        "integration::a                                     [Module]",
        "integration::a::c                                  [Module]",
    ]
    "###);

    // item counts
    shot!(doc.paths.len(), @"1974");
    shot!(js.local_path().count(), @"5");
    shot!(doc.index.len(), @"107");
    shot!(js.local_index().count(), @"20");

    // data sizes
    shot!(ByteSize(json.len() as _), @"330.8 KB");

    Ok(())
}

#[test]
fn compression() -> Result<()> {
    /// compress any byte source via xz
    fn compress(buf: &[u8]) -> Result<u64> {
        let len = buf.len();
        let mut compressor = xz2::write::XzEncoder::new(Vec::with_capacity(len / 5), 9);
        compressor.write_all(buf)?;
        Ok(compressor.finish()?.len() as _)
    }
    /// compress binary data generated by bincode from json string
    fn compress_bin(doc: &Crate) -> Result<[u64; 2]> {
        let config = bincode::config::standard();
        let serialized = bincode::serde::encode_to_vec(doc, config)?;
        compress(&serialized)?;
        // ensure bytes can be decoded back
        bincode::serde::decode_from_slice::<Crate, _>(&serialized, config)?;
        Ok([serialized.len() as _, compress(&serialized)?])
    }

    let reduced_size = |large, small| {
        format!(
            "{} => {} (-{:.0}%)",
            bytesize::ByteSize(large),
            bytesize::ByteSize(small),
            100.0 - (small as f32 / large as f32) * 100.0
        )
    };
    let JsonDoc { json, doc } = &*INTEGRATION;
    let json_size = json.len() as u64;

    let json_compression = format!(
        "[raw json text => xz] {}",
        reduced_size(json_size, compress(json.as_bytes())?)
    );
    shot!(json_compression, @"[raw json text => xz] 330.8 KB => 41.4 KB (-87%)");

    let [bin_size, xz_size] = compress_bin(doc)?;
    let bin_compression = format!(
        "[raw json text => bb] {}\n\
         [binary bytes  => xz] {}\n\
         [raw json text => xz] {} ",
        reduced_size(json_size, bin_size),
        reduced_size(bin_size, xz_size),
        reduced_size(json_size, xz_size)
    );
    shot!(bin_compression, @r###"
    [raw json text => bb] 330.8 KB => 167.3 KB (-49%)
    [binary bytes  => xz] 167.3 KB => 39.9 KB (-76%)
    [raw json text => xz] 330.8 KB => 39.9 KB (-88%) 
    "###);

    Ok(())
}

#[test]
fn stats() {
    let js @ JsonDoc { doc, .. } = &*INTEGRATION;
    let local_crate_name = js.get_path_xstring(&doc.root);
    let mut crates: FxHashMap<XString, ID> = FxHashMap::default();
    // local crate id is always 0
    crates.insert(local_crate_name.expect("local crate name not found"), 0);
    for (id, krate) in &doc.external_crates {
        // external id is always non-zero
        crates.insert(krate.name.as_str().into(), *id);
    }
    // ensure 0 refers to the local crate
    assert_eq!(crates.values().filter(|id| **id == 0).count(), 1);
    dbg!(&crates);
}

pub type ID = u32;
pub enum ImplKind {
    Auto,
    Blanket,
}

pub struct IndexId {
    pub impl_kind: Option<ImplKind>,
    pub crate_id: ID,
    pub item_id: ID,
    pub name_id: Option<ID>,
    pub extra: Option<PathId>,
}
impl FromStr for IndexId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const TARGET: &str = "IndexId";
        if s.is_empty() {
            return Err(format!(
                "index id `{s}` is an empty string to be unable to parse"
            ));
        };
        let Some(found) = RE.index.captures(s) else {
            return Err(format!(
                "index id `{s}` is not matched against the regex pattern\n`{}`\n\
                 or equivalently `[IMPL:]CRATE_ID:ITEM_ID[:NAME_ID][-EXTRA]`",
                PatternId::INDEX
            ));
        };
        let impl_kind = found.name("impl").and_then(|m| match m.as_str() {
            "a" => Some(ImplKind::Auto),
            "b" => Some(ImplKind::Blanket),
            _ => None,
        });
        let crate_id = {
            let id = &found["crate"];
            id.parse::<ID>()
                .map_err(|_| err_parse_int(id, s, TARGET, "crate_id"))?
        };
        let item_id = {
            let id = &found["item"];
            id.parse::<ID>()
                .map_err(|_| err_parse_int(id, s, TARGET, "item_id"))?
        };
        let name_id = match found.name("name") {
            Some(id) => Some({
                let id = id.as_str();
                id.parse::<ID>()
                    .map_err(|_| err_parse_int(id, s, TARGET, "name_id"))?
            }),
            None => None,
        };
        let extra = match found.name("extra") {
            Some(id) => Some(id.as_str().parse::<PathId>()?),
            None => None,
        };
        Ok(IndexId {
            impl_kind,
            crate_id,
            item_id,
            name_id,
            extra,
        })
    }
}

#[cold]
fn err_parse_int(id: &str, s: &str, target: &str, current: &str) -> String {
    format!("{current} `{id}` in {target} `{s}` can't be parsed as an u32")
}
#[cold]
fn err_parse_incompletely(s: &str, lack: &str, target: &str) -> String {
    format!("{target} `{s}` can't be parsed due to lack of {lack}")
}

pub struct PathId {
    pub crate_id: ID,
    pub item_id: ID,
    pub name_id: ID,
}
impl FromStr for PathId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const TARGET: &str = "PathId";
        let mut component = s.split(':');
        let Some(impl_or_crate) = component.next() else {
            return Err(format!(
                "path id `{s}` is an empty string to be unable to parse"
            ));
        };
        match impl_or_crate {
            "a" => Err(format!("path id `{s}` starts with an auto impl tag")),
            "b" => Err(format!("path id `{s}` starts with a blanket impl tag")),
            id => {
                let crate_id = id
                    .parse::<ID>()
                    .map_err(|_| err_parse_int(id, s, TARGET, "crate_id"))?;
                if let Some(id) = component.next() {
                    let item_id = id
                        .parse::<ID>()
                        .map_err(|_| err_parse_int(id, s, TARGET, "item_id"))?;
                    if let Some(id) = component.next() {
                        let name_id = id
                            .parse::<ID>()
                            .map_err(|_| err_parse_int(id, s, TARGET, "name_id"))?;
                        Ok(PathId {
                            crate_id,
                            item_id,
                            name_id,
                        })
                    } else {
                        Err(err_parse_incompletely(s, "name_id", TARGET))
                    }
                } else {
                    Err(err_parse_incompletely(s, "item_id", TARGET))
                }
            }
        }
    }
}
impl Display for PathId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let PathId {
            crate_id,
            item_id,
            name_id,
        } = *self;
        write!(f, "{crate_id}:{item_id}:{name_id}")
    }
}

#[test]
fn parse_check() -> Result<()> {
    let js @ JsonDoc { doc, .. } = &*INTEGRATION;

    // all the path id contains exact three components
    let _ = doc
        .paths
        .iter()
        .map(|(id, item)| {
            id.0.parse::<PathId>().map_err(|err| {
                eyre!(
                    "failed to parse the id of path `{}`: {err}",
                    item.path.join("::")
                )
            })
        })
        .collect::<Result<Vec<_>>>()?;

    // all the index ids can be parsed to IndexIds
    let _ = doc
        .index
        .keys()
        .map(|id| {
            id.0.parse::<IndexId>().map_err(|err| {
                eyre!(
                    "failed to parse the id of path `{}`: {err}",
                    js.get_path_string(id).unwrap_or_default()
                )
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(())
}

#[test]
fn parse_extract_local() {
    let js = &*INTEGRATION;
    let mut local_items = js
        .local_index()
        .map(|(id, item)| {
            let name = item
                .name
                .as_deref()
                .map(|s| xformat!(": ({s})"))
                .unwrap_or_default();
            (
                &*id.0,
                js.get_item(&id.0.parse::<IndexId>().unwrap())
                    .into_iter()
                    .map(|item| {
                        format!(
                            "{path:50} {id:20} [{kind:?}{name}]",
                            id = item.0 .0,
                            path = item.1.join("::"),
                            kind = item.2
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    local_items.sort_unstable();
    snap!("local_items", local_items);
}

#[test]
fn parse_module() {
    let parsed = DModule::new(&INTEGRATION.doc);
    snap!("DModule", parsed);
}
