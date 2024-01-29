mod app;
mod event;
mod tui;
mod ui;
mod update;

use crate::update::update;
use color_eyre::eyre::Result;
use event::Event;

fn main() -> Result<()> {
    tui::install_hooks()?;

    let mut tui = tui::Tui::new(1000)?;
    let mut app = app::App::default();

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    Ok(())
}
