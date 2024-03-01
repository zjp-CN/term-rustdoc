use std::{fs::File, path::PathBuf};
use term_rustdoc::tree::{CrateDoc, Show};

fn init_logger() {
    let file = File::create("./target/test.log").unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(file)
        .with_ansi(false)
        .without_time()
        .init();
}

// replace this path on your machine
fn compile(path: &str) -> PathBuf {
    rustdoc_json::Builder::default()
        .toolchain("nightly")
        .target_dir("./target")
        .manifest_path(path)
        .build()
        .unwrap()
}

#[test]
#[ignore = "stack overflows caused by recursive reexported modules has been solved"]
fn compile_actix_0_13_0() {
    init_logger();
    let json_path =
        compile("/root/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/actix-0.13.0/Cargo.toml");
    dbg!(&json_path);
    let file = File::open(&json_path).unwrap();
    let json: rustdoc_types::Crate = serde_json::from_reader(file).unwrap();
    println!("parse done");
    let crate_doc = CrateDoc::new(json);
    dbg!(&crate_doc);
}

#[test]
#[ignore = "TODO reexport external crate items"]
fn compile_nucleo_0_3_0() {
    init_logger();
    let json_path =
        compile("/root/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/nucleo-0.3.0/Cargo.toml");
    dbg!(&json_path);
    let file = File::open(&json_path).unwrap();
    let json: rustdoc_types::Crate = serde_json::from_reader(file).unwrap();
    println!("parse done");
    let crate_doc = CrateDoc::new(json);
    println!("{}", crate_doc.dmodule().show_prettier(&crate_doc));
}
