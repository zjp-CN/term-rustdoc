mod update;

use crate::{dashboard::DashBoard, ui::Page};
use ratatui::prelude::Widget;

pub struct Frame {
    pub dash_board: DashBoard,
    pub page: Page,
}

impl Widget for &mut Frame {
    /// entry point for all rendering
    fn render(self, full: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.dash_board.ui().render(full, buf);
        // self.page.render(full, buf);
    }
}
