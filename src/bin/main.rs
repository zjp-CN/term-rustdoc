mod color;
mod dashboard;
mod database;
mod event;
mod frame;
mod fuzzy;
mod local_registry;
mod logger;
mod tui;
mod ui;

#[macro_use]
extern crate tracing;

use self::frame::Frame;
use color_eyre::eyre::{eyre as err, Result, WrapErr};

fn main() -> Result<()> {
    tui::install_hooks()?;
    logger::init()?;

    let mut tui = tui::Tui::new(1000)?;
    let fuzz = fuzzy::Fuzzy::new();

    let full = tui.size()?;
    let sender = tui.events.get_sender();
    let dash_board = dashboard::DashBoard::new(full, fuzz, sender)?;
    let mut frame = Frame::new(dash_board);

    // Start the main loop.
    while !frame.should_quit {
        // Render the user interface.
        tui.draw(&mut frame)?;
        // Handle events.
        frame.consume_event(tui.events.next()?);
    }

    Ok(())
}
