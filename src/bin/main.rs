mod app;
mod dashboard;
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
use color_eyre::eyre::{eyre as err, Result};

fn main() -> Result<()> {
    tui::install_hooks()?;
    logger::init()?;

    let mut tui = tui::Tui::new(1000)?;
    let mut app = app::App::init()?;
    let fuzz = fuzzy::Fuzzy::new();

    // let outline = app.set_doc()?;
    let full = tui.size()?;
    // let page = ui::Page::new(outline, app.rustdoc(), full)?;
    let page = Default::default();
    let dash_board = dashboard::DashBoard::new(full, fuzz)?;
    let mut frame = Frame { dash_board, page };

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app, &mut frame)?;
        // Handle events.
        frame.consume_event(tui.events.next()?, &mut app);
    }

    Ok(())
}
