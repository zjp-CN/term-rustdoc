use crate::{
    app::App,
    ui::{Page, ScrollOffset},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut App, page: &mut Page, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Home => page.scroll_home_outline(),
        KeyCode::End => page.scroll_end_outline(),
        KeyCode::PageUp => page.scrollup_outline(ScrollOffset::HalfScreen),
        KeyCode::PageDown => page.scrolldown_outline(ScrollOffset::HalfScreen),
        _ => {}
    };
}
