mod update;

use crate::{app::App, dashboard::DashBoard, ui::Page};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::{Buffer, Rect, Widget};

pub struct Frame {
    pub dash_board: DashBoard,
    pub page: Page,
    focus: Focus,
}

#[derive(Default, Debug, Clone, Copy)]
enum Focus {
    #[default]
    DashBoard,
    Page,
}

impl Frame {
    pub fn new(dash_board: DashBoard) -> Frame {
        let (page, focus) = Default::default();
        Frame {
            dash_board,
            page,
            focus,
        }
    }

    fn switch_to_page(&mut self) {
        self.focus = Focus::Page;
    }

    fn switch_focus(&mut self) {
        let before = self.focus;
        self.focus = match self.focus {
            Focus::DashBoard if !self.page.is_empty() => Focus::Page,
            _ => Focus::DashBoard,
        };
        info!("Frame: swicth from {before:?} to {:?}", self.focus);
    }

    fn update_for_key(&mut self, app: &mut App, key_event: KeyEvent) {
        if key_event.modifiers == KeyModifiers::CONTROL {
            #[allow(clippy::single_match)]
            match key_event.code {
                KeyCode::Char('w') => {
                    self.switch_focus();
                    return;
                }
                _ => (),
            }
        }
        match self.focus {
            Focus::DashBoard => update::update_dash_board(&mut self.dash_board, app, &key_event),
            Focus::Page => update::update_page(&mut self.page, app, &key_event),
        };
    }
}

impl Widget for &mut Frame {
    /// entry point for all rendering
    fn render(self, full: Rect, buf: &mut Buffer) {
        match self.focus {
            Focus::DashBoard => self.dash_board.ui().render(full, buf),
            Focus::Page => self.page.render(full, buf),
        };
    }
}
