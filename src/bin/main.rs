mod app;
mod event;
mod logger;
mod tui;
mod ui;
mod update;

#[macro_use]
extern crate tracing;

use crate::update::update;
use color_eyre::eyre::{eyre as err, Result};
use crossterm::event::{MouseButton, MouseEventKind};
use event::Event;
use ui::ScrollOffset;

fn main() -> Result<()> {
    tui::install_hooks()?;
    logger::init()?;

    let mut tui = tui::Tui::new(1000)?;
    let mut app = app::App::default();

    let outline = app.set_doc()?;
    let mut page = ui::Page::new(outline, app.rustdoc(), tui.size()?)?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app, &mut page)?;
        // Handle events.
        match tui.events.next()? {
            Event::Key(key_event) => update(&mut app, &mut page, key_event),
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::ScrollDown => {
                    page.scrolldown(ScrollOffset::Fixed(5));
                }
                MouseEventKind::ScrollUp => {
                    page.scrollup(ScrollOffset::Fixed(5));
                }
                MouseEventKind::Down(MouseButton::Left) => {
                    let (x, y) = (mouse_event.column, mouse_event.row);
                    tui.events.left_click()?;
                    page.set_current_component(y, x);
                }
                _ => (),
            },
            Event::Resize(_, _) => {}
            Event::MouseDoubleClick => page.double_click(),
        };
    }

    Ok(())
}
