use super::{
    fallback::{ScrollText, StyledLine},
    StyledText,
};
use ratatui::prelude::{Buffer, Rect};
use unicode_width::UnicodeWidthStr;

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
            render_line(line, buf, x, y, width);
            y += 1;
        }
    }
}

fn render_line(line: &[StyledText], buf: &mut Buffer, mut x: u16, y: u16, width: usize) {
    let mut used_width = 0usize;
    for stext in line {
        let text = stext.as_str();
        used_width += text.width_cjk();
        // stop rendering once it hits the end of width
        if used_width >= width {
            return;
        }
        let (x_pos, _) = buf.set_stringn(x, y, text, width, stext.style());
        x = x_pos;
    }
}
