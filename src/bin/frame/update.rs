use super::{help::Help, Focus, Frame};
use crate::{
    dashboard::DashBoard,
    event::Event,
    ui::{Page, ScrollOffset},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

impl Frame {
    pub fn consume_event(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => self.update_for_key(key_event),
            Event::Mouse(mouse_event) => self.update_for_mouse(mouse_event),
            Event::Resize(_, _) => {}
            Event::MouseDoubleClick(x, y) => self.update_for_double_click((x, y)),
            Event::DocCompiled(info) => self.dash_board.ui().receive_compiled_doc(*info),
            Event::CrateDoc(pkg_key) => {
                let ui = &self.dash_board.ui();
                if let Some(doc) = ui.get_loaded_doc(&pkg_key) {
                    match Page::new(*pkg_key, doc, ui.get_full_area()) {
                        Ok(page) => {
                            self.page = page;
                            self.switch_to_page();
                        }
                        Err(err) => error!("Failed to construct a Page:\n{err}"),
                    }
                }
            }
            Event::Downgraded(pkg_key) => self.page.drop(&pkg_key),
        };
    }

    fn update_for_key(&mut self, key_event: KeyEvent) {
        if key_event.modifiers == KeyModifiers::CONTROL {
            #[allow(clippy::single_match)]
            match key_event.code {
                KeyCode::Char('w') => {
                    self.switch_focus();
                    return;
                }
                KeyCode::Char('q') => {
                    self.quit();
                    return;
                }
                _ => (),
            }
        }

        if let KeyCode::F(1) = key_event.code {
            if !matches!(self.focus, Focus::Help) {
                self.get_help();
            } else {
                self.switch_focus();
            }
            return;
        }

        match self.focus {
            Focus::DashBoard => update_dash_board(&mut self.dash_board, &key_event),
            Focus::Page => update_page(&mut self.page, &key_event),
            Focus::Help => update_help(self.get_help(), &key_event),
        };
    }

    fn update_for_mouse(&mut self, event: MouseEvent) {
        match self.focus {
            Focus::DashBoard => {
                if self.dash_board.ui().update_for_mouse(event) && !self.page.is_empty() {
                    self.switch_to_page();
                }
            }
            Focus::Page => match event.kind {
                MouseEventKind::ScrollDown => {
                    self.page.scrolldown(ScrollOffset::Fixed(5));
                }
                MouseEventKind::ScrollUp => {
                    self.page.scrollup(ScrollOffset::Fixed(5));
                }
                MouseEventKind::Down(MouseButton::Left) => {
                    let (x, y) = (event.column, event.row);
                    self.page.set_current_panel(y, x);
                }
                _ => (),
            },
            Focus::Help => {
                let popup = self.get_help();
                let help = popup.scroll_text();
                match event.kind {
                    MouseEventKind::ScrollDown => help.scrolldown(ScrollOffset::Fixed(5)),
                    MouseEventKind::ScrollUp => help.scrollup(ScrollOffset::Fixed(5)),
                    MouseEventKind::Down(MouseButton::Left) => {
                        let position = (event.column, event.row);
                        if popup.heading_jump(position) {
                            return;
                        }
                        if !popup.contains(position) {
                            self.switch_focus();
                        }
                    }
                    _ => (),
                }
            }
        };
    }

    fn update_for_double_click(&mut self, position: (u16, u16)) {
        match self.focus {
            Focus::DashBoard if self.dash_board.ui().contains(position) => {
                self.dash_board.ui().compile_or_load_doc(Some(position.1))
            }
            Focus::Page => self.page.double_click(),
            _ => (),
        }
    }
}

fn update_dash_board(dash: &mut DashBoard, key_event: &KeyEvent) {
    let ui = dash.ui();
    if key_event.modifiers == KeyModifiers::CONTROL {
        match key_event.code {
            KeyCode::Char('c') => ui.clear_input(),
            KeyCode::Char('s') => ui.switch_sort(),
            KeyCode::Char('f') => ui.switch_search_source(),
            _ => (),
        }
        return;
    }
    match key_event.code {
        KeyCode::Char(ch) => ui.push_char(ch),
        KeyCode::Backspace => ui.pop_char(),
        KeyCode::Up => ui.move_backward_cursor(),
        KeyCode::Down => ui.move_forward_cursor(),
        KeyCode::Home => ui.scroll_home(),
        KeyCode::End => ui.scroll_end(),
        KeyCode::PageUp => ui.scroll_up(),
        KeyCode::PageDown => ui.scroll_down(),
        KeyCode::Enter => ui.compile_or_load_doc(None),
        KeyCode::Tab => ui.switch_panel(),
        KeyCode::Delete => ui.downgrade(None),
        _ => (),
    }
}

fn update_page(page: &mut Page, key_event: &KeyEvent) {
    match key_event.code {
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

fn update_help(help: &mut Help, event: &KeyEvent) {
    let help = help.scroll_text();
    match event.code {
        KeyCode::Up => help.scrollup(ScrollOffset::Fixed(1)),
        KeyCode::Down => help.scrolldown(ScrollOffset::Fixed(1)),
        KeyCode::Home => help.scroll_home(),
        KeyCode::End => help.scroll_end(),
        KeyCode::PageUp => help.scrollup(ScrollOffset::Fixed(5)),
        KeyCode::PageDown => help.scrolldown(ScrollOffset::Fixed(5)),
        _ => (),
    }
}
