mod help;
mod update;
mod util;

pub use self::util::centered_rect;

use self::help::Help;
use crate::{dashboard::DashBoard, ui::Page};
use ratatui::prelude::{Buffer, Rect, Widget};

pub struct Frame {
    pub dash_board: DashBoard,
    pub page: Page,
    focus: Focus,
    /// Initialize this when needed the first time.
    help: Option<Box<Help>>,
}

#[derive(Default, Debug, Clone, Copy)]
enum Focus {
    #[default]
    DashBoard,
    Page,
}

impl Frame {
    pub fn new(dash_board: DashBoard) -> Frame {
        let (page, focus, help) = Default::default();
        Frame {
            dash_board,
            page,
            focus,
            help,
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

    fn get_help(&mut self) -> Option<&mut Help> {
        self.help.as_deref_mut().filter(|h| h.show())
    }

    fn get_help_anyway(&mut self) -> &mut Help {
        self.help.get_or_insert_with(|| {
            let help = Help::new(self.dash_board.ui().get_full_area());
            info!("Initialize Help");
            Box::new(help)
        })
    }
}

impl Widget for &mut Frame {
    /// entry point for all rendering
    fn render(self, full: Rect, buf: &mut Buffer) {
        if let Some(help) = self.get_help() {
            help.update_area(full);
            help.render(buf);
            return;
        }
        match self.focus {
            Focus::DashBoard => self.dash_board.ui().render(full, buf),
            Focus::Page => self.page.render(full, buf),
        };
    }
}
