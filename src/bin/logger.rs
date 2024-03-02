use crate::{err, Result};
use std::{
    fs::{self, File},
    path::PathBuf,
};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

pub fn init() -> Result<()> {
    #[cfg(debug_assertions)]
    let path = PathBuf::from_iter(["target", "term_rustdoc.log"]);
    #[cfg(not(debug_assertions))]
    let path = data_dir()?.join("term_rustdoc.log");
    let file = File::create(path)?;

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

pub fn data_dir() -> Result<PathBuf> {
    let mut dir = dirs::data_local_dir().ok_or_else(|| err!("Can't find the config_local_dir"))?;
    dir.push("term-rustdoc");
    if !dir.exists() {
        fs::create_dir(&dir)?;
    }
    Ok(dir)
}
