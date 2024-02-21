use crate::{err, Result};
use std::fs::File;

pub fn init() -> Result<()> {
    let file = File::create("./target/term_rustdoc.log")?;
    tracing_subscriber::fmt()
        .with_writer(file)
        // .with_ansi(false)
        .try_init()
        .map_err(|_| err!("logger init failed: maybe there is another initializer?"))?;
    info!("logging initialized and the program starts");
    Ok(())
}
