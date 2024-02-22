mod update;

use crate::{dashboard::DashBoard, ui::Page};
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
