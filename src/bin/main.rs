mod app;
mod event;
mod tui;
mod ui;
mod update;

use crate::update::update;
use color_eyre::eyre::Result;
use crossterm::event::{MouseButton, MouseEventKind};
use event::Event;

fn main() -> Result<()> {
    tui::install_hooks()?;

    let mut tui = tui::Tui::new(1000)?;
    let mut app = app::App::default();
    let outline = app.set_doc()?;

    let (len, mut outline) = app::cache_outline(&outline);
    let mut rows = 0u16;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app, Some(outline.clone()))?;
        // Handle events.
        match tui.events.next()? {
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::ScrollDown | MouseEventKind::Down(MouseButton::Left) => {
                    rows = (rows + 5).min(len);
                    outline = outline.scroll((rows, 0));
                }
                MouseEventKind::ScrollUp | MouseEventKind::Down(MouseButton::Right) => {
                    rows = rows.saturating_sub(5);
                    outline = outline.scroll((rows, 0));
                }
                _ => (),
            },
            Event::Resize(_, _) => {}
        };
    }

    Ok(())
}
