use std::fs::File;
use term_rustdoc::tree::CrateDoc;

fn init_logger() {
    let file = File::create("./target/test.log").unwrap();
    tracing_subscriber::fmt()
        .with_writer(file)
        .with_ansi(false)
        .without_time()
        .init();
}

#[test]
fn compile_actix_0_13_0() {
    init_logger();
    let json_path = rustdoc_json::Builder::default()
        .toolchain("nightly")
        .target_dir("./target")
        .manifest_path(
            // "/root/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/tokio-1.35.1/Cargo.toml",
            "/root/.cargo/registry/src/rsproxy.cn-0dccff568467c15b/actix-0.13.0/Cargo.toml",
        )
        .build()
        .unwrap();
    dbg!(&json_path);
    let file = File::open(&json_path).unwrap();
    let json: rustdoc_types::Crate = serde_json::from_reader(file).unwrap();
    println!("parse done");
    let crate_doc = CrateDoc::new(json);
    dbg!(&crate_doc);
}
