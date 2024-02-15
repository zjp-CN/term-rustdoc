use super::fallback::{ScrollText, StyledLine};
use crate::ui::scrollable::generics::render_line;
use ratatui::prelude::{Buffer, Rect};

impl ScrollText {
    pub fn render(&mut self, buf: &mut Buffer) {
        write_lines(&self.lines, self.start, self.area, buf);
    }
}

fn write_lines(lines: &[StyledLine], row_start: usize, rect: Rect, buf: &mut Buffer) {
    if lines.is_empty() {
        return;
    }
    let Rect {
        x,
        mut y,
        width,
        height,
    } = rect;
    let row_end = (row_start + height as usize).min(lines.len());
    let width = width as usize;
    if let Some(lines) = lines.get(row_start..row_end) {
        for line in lines {
            render_line(line.iter_text_style(), buf, x, y, width);
            y += 1;
        }
    }
}
