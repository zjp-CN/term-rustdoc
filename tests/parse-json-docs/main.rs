use bytesize::ByteSize;
use color_eyre::eyre::Result;
use insta::{assert_debug_snapshot as snap, assert_snapshot as shot};
use rustc_hash::FxHashMap;
use rustdoc_types::{Crate, Id, Item, ItemKind, ItemSummary};
use std::{
    io::{Read, Write},
    path::PathBuf,
    sync::LazyLock,
};
use term_rustdoc::{
    tree::{CrateDoc, Tag},
    util::{join_path, XString},
};

mod fn_item_decl;
mod generate_doc_json;
mod parse;
mod syntect_set;

static INTEGRATION: LazyLock<JsonDoc> = LazyLock::new(|| {
    tracing_subscriber::fmt::init();
    println!("start!");
    get_integration_json_doc().expect("failed to get the json doc of tests/integration crate")
});

fn doc() -> CrateDoc {
    thread_local! {
        static DOC: CrateDoc = CrateDoc::new(INTEGRATION.doc.clone());
    }
    DOC.with(|d| d.clone())
}

struct JsonDoc {
    json: String,
    doc: Crate,
}
impl JsonDoc {
    fn get_path_xstring(&self, id: &Id) -> Option<XString> {
        self.doc.paths.get(id).map(|p| join_path(&p.path))
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
    snap!(paths, @r#"
    [
        "integration                                        [Module]",
        "integration::ACONSTANT                             [Constant]",
        "integration::ASTATIC                               [Constant]",
        "integration::ATrait                                [Trait]",
        "integration::ATraitWithGAT                         [Trait]",
        "integration::AUnitStruct                           [Struct]",
        "integration::FieldsNamedStruct                     [Struct]",
        "integration::a_decl_macro                          [Macro]",
        "integration::func_dyn_trait                        [Function]",
        "integration::func_dyn_trait2                       [Function]",
        "integration::func_fn_pointer_impl_trait            [Function]",
        "integration::func_hrtb                             [Function]",
        "integration::func_lifetime_bounds                  [Function]",
        "integration::func_primitive                        [Function]",
        "integration::func_qualified_path                   [Function]",
        "integration::func_trait_bounds                     [Function]",
        "integration::func_tuple_array_slice                [Function]",
        "integration::func_with_1arg                        [Function]",
        "integration::func_with_1arg_and_ret                [Function]",
        "integration::func_with_const                       [Function]",
        "integration::func_with_no_args                     [Function]",
        "integration::no_synthetic                          [Function]",
        "integration::structs                               [Module]",
        "integration::structs::Named                        [Struct]",
        "integration::structs::NamedAllPrivateFields        [Struct]",
        "integration::structs::NamedAllPublicFields         [Struct]",
        "integration::structs::NamedGeneric                 [Struct]",
        "integration::structs::NamedGenericAllPrivate       [Struct]",
        "integration::structs::NamedGenericWithBound        [Struct]",
        "integration::structs::NamedGenericWithBoundAllPrivate [Struct]",
        "integration::structs::Tuple                        [Struct]",
        "integration::structs::TupleAllPrivate              [Struct]",
        "integration::structs::TupleGeneric                 [Struct]",
        "integration::structs::TupleGenericWithBound        [Struct]",
        "integration::structs::TupleWithBound               [Struct]",
        "integration::structs::Unit                         [Struct]",
        "integration::structs::UnitGeneric                  [Struct]",
        "integration::structs::UnitGenericWithBound         [Struct]",
        "integration::structs::UnitWithBound                [Struct]",
        "integration::submod1                               [Module]",
        "integration::submod1::AUnitEnum                    [Enum]",
        "integration::submod1::AUnitEnum::A                 [Variant]",
        "integration::submod1::AUnitEnum::B                 [Variant]",
        "integration::submod1::AUnitEnum::C                 [Variant]",
        "integration::submod1::submod2                      [Module]",
        "integration::submod1::submod2::ATraitNeverImplementedForTypes [Trait]",
        "integration::variadic                              [Function]",
        "integration::variadic_multiline                    [Function]",
    ]
    "#);

    // item counts
    shot!(doc.paths.len(), @"2362");
    shot!(js.local_path().count(), @"48");
    shot!(doc.index.len(), @"334");
    shot!(js.local_index().count(), @"325");

    // data sizes
    shot!(ByteSize(json.len() as _), @"463.3 KB");

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
    shot!(json_compression, @"[raw json text => xz] 463.3 KB => 34.3 KB (-93%)");

    let [bin_size, xz_size] = compress_bin(doc)?;
    let bin_compression = format!(
        "[raw json text => bb] {}\n\
         [binary bytes  => xz] {}\n\
         [raw json text => xz] {} ",
        reduced_size(json_size, bin_size),
        reduced_size(bin_size, xz_size),
        reduced_size(json_size, xz_size)
    );
    shot!(bin_compression, @r"
    [raw json text => bb] 463.3 KB => 120.9 KB (-74%)
    [binary bytes  => xz] 120.9 KB => 31.1 KB (-74%)
    [raw json text => xz] 463.3 KB => 31.1 KB (-93%)
    ");

    Ok(())
}

#[test]
fn stats() {
    let js @ JsonDoc { doc, .. } = &*INTEGRATION;
    let local_crate_name = js.get_path_xstring(&doc.root);
    let mut crates: FxHashMap<XString, u32> = FxHashMap::default();
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DebugItem {
    path: String,
    tag: Tag,
    id: u32,
}

impl DebugItem {
    fn new(id: u32, item: &ItemSummary) -> DebugItem {
        use ItemKind::*;
        let tag = match item.kind {
            Module => Tag::Module,
            ExternCrate => Tag::Unknown,
            Use => Tag::Unknown,
            Struct => Tag::Struct,
            StructField => Tag::Field,
            Union => Tag::Union,
            Enum => Tag::Enum,
            Variant => Tag::Variant,
            Function => Tag::Function,
            TypeAlias => Tag::TypeAlias,
            Constant => Tag::Constant,
            Trait => Tag::Trait,
            TraitAlias => Tag::Unknown,
            Impl => Tag::Implementations,
            Static => Tag::Static,
            ExternType => Tag::Unknown,
            Macro => Tag::MacroDecl,
            ProcAttribute => Tag::MacroAttr,
            ProcDerive => Tag::MacroDerv,
            AssocConst => Tag::AssocConst,
            AssocType => Tag::AssocType,
            Primitive => Tag::Unknown,
            Keyword => Tag::Unknown,
        };
        DebugItem {
            tag,
            path: item.path.join("::"),
            id,
        }
    }

    #[allow(clippy::inherent_to_string)]
    fn to_string(&self) -> String {
        let Self { tag, path, id, .. } = self;
        format!("({id:03}) {path:<60} [{tag:?}]")
    }
}

#[test]
fn parse_extract_local() {
    let js = &*INTEGRATION;
    let mut local_items = js
        .local_path()
        .map(|(Id(id), item)| DebugItem::new(*id, item))
        .collect::<Vec<_>>();
    local_items.sort_unstable();
    snap!(
        "local_items",
        local_items
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
    );
}
