use crate::{err, Result};
use std::fs::File;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

pub fn init() -> Result<()> {
    let file = File::create("./target/term_rustdoc.log")?;

    // RUST_LOG="debug" or RUST_LOG="module_path=debug" environment variable
    // https://docs.rs/tracing-subscriber/0.3.18/tracing_subscriber/filter/struct.EnvFilter.html#example-syntax
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(file)
        // .with_ansi(false)
        .try_init()
        .map_err(|_| err!("logger init failed: maybe there is another initializer?"))?;
    info!("logging initialized and the program starts");
    Ok(())
}
