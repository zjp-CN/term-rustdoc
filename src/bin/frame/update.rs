use super::Frame;
use crate::{
    app::App,
    dashboard::DashBoard,
    event::Event,
    ui::{Page, ScrollOffset},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEventKind};

impl Frame {
    pub fn consume_event(&mut self, event: Event, app: &mut App) {
        match event {
            Event::Key(key_event) => self.update(app, key_event),
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::ScrollDown => {
                    self.page.scrolldown(ScrollOffset::Fixed(5));
                }
                MouseEventKind::ScrollUp => {
                    self.page.scrollup(ScrollOffset::Fixed(5));
                }
                MouseEventKind::Down(MouseButton::Left) => {
                    let (x, y) = (mouse_event.column, mouse_event.row);
                    self.page.set_current_panel(y, x);
                }
                _ => (),
            },
            Event::Resize(_, _) => {}
            Event::MouseDoubleClick => self.page.double_click(),
        };
    }

    fn update(&mut self, app: &mut App, key_event: KeyEvent) {
        update_dash_board(&mut self.dash_board, app, &key_event);
        // update_page(&mut self.page, app, &key_event);
    }
}

fn update_dash_board(dash: &mut DashBoard, app: &mut App, key_event: &KeyEvent) {
    let dash = dash.ui();
    if key_event.modifiers == KeyModifiers::CONTROL {
        match key_event.code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') => dash.clear_input(),
            _ => (),
        }
        return;
    }
    match key_event.code {
        KeyCode::Char(ch) => dash.push_char(ch),
        KeyCode::Backspace => dash.pop_char(),
        KeyCode::PageUp => dash.scroll_up(),
        KeyCode::PageDown => dash.scroll_down(),
        _ => (),
    }
}

fn update_page(page: &mut Page, app: &mut App, key_event: &KeyEvent) {
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
        KeyCode::Char('1') => page.outline_fold_expand_to_first_level_modules(),
        KeyCode::Enter => page.outline_fold_expand_toggle(),
        KeyCode::Char('d') => page.toggle_sytect(),
        _ => {}
    };
}
