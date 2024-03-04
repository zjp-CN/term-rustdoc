use crate::ui::scrollable::render_line;
use ratatui::{
    prelude::{Buffer, Rect, Style, Widget},
    widgets::Block,
};
use unicode_width::UnicodeWidthStr;

#[derive(Default, Debug)]
pub struct Surround {
    block: Block<'static>,
    area: Rect,
}

impl Surround {
    pub fn new(block: Block<'static>, area: Rect) -> Self {
        Surround { block, area }
    }

    pub fn inner(&self) -> Rect {
        self.block.inner(self.area)
    }

    pub fn block_mut(&mut self) -> &mut Block<'static> {
        &mut self.block
    }

    pub fn render(&self, buf: &mut Buffer) {
        (&self.block).render(self.area, buf);
    }

    pub fn area(&self) -> Rect {
        self.area
    }

    /// Update the border area and then return inner area only when the outer areas differ.
    pub fn update_area(&mut self, area: Rect) -> Option<Rect> {
        if self.area == area {
            return None;
        }
        self.area = area;
        Some(self.inner())
    }

    pub fn render_only_bottom_right_text(&self, buf: &mut Buffer, text: &str) -> usize {
        let area = self.area;
        let text_width = text.width();
        if let Some(offset) = (area.width as usize).checked_sub(2 + text_width) {
            let x = area.x + offset as u16;
            let y = area.y + area.height - 1;
            render_line(Some((text, Style::new())), buf, x, y, text_width);
            return text_width + 2;
        }
        0
    }

    pub fn render_only_bottom_left_text(&self, buf: &mut Buffer, text: &str, used: usize) {
        let area = self.area;
        if let Some(rest) = (area.width as usize).checked_sub(2 + used) {
            if rest < text.width() {
                // not enought space to show
                return;
            }
            let x = area.x + 2;
            let y = area.y + area.height - 1;
            render_line(Some((text, Style::new())), buf, x, y, rest);
        }
    }

    pub fn render_only_top_left_text(&self, buf: &mut Buffer, text: &str, used: usize) {
        let area = self.area;
        if let Some(rest) = (area.width as usize).checked_sub(2 + used) {
            if rest < text.width() {
                // not enought space to show
                return;
            }
            let x = area.x + 2;
            let y = area.y;
            render_line(Some((text, Style::new())), buf, x, y, rest);
        }
    }
}
