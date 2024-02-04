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
        KeyCode::Home => page.scroll_home(),
        KeyCode::End => page.scroll_end(),
        KeyCode::PageUp => page.scrollup(ScrollOffset::HalfScreen),
        KeyCode::PageDown => page.scrolldown(ScrollOffset::HalfScreen),
        KeyCode::Up => page.move_backward_cursor(),
        KeyCode::Down => page.move_forward_cursor(),
        KeyCode::Char('L') => page.move_bottom_cursor(),
        KeyCode::Char('H') => page.move_top_cursor(),
        KeyCode::Char('M') => page.move_middle_cursor(),
        KeyCode::Char('m') => page.outline_fold_expand_current_module_only(),
        KeyCode::Char('/') => page.outline_fold_expand_all(),
        KeyCode::Char('0') => page.outline_fold_expand_zero_level(),
        KeyCode::Char('1') => page.outline_fold_expand_first_level_modules_only(),
        KeyCode::Enter => page.outline_fold_expand_toggle(),
        _ => {}
    };
}
