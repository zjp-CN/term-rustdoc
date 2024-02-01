use super::Scrollable;
use ratatui::prelude::{Buffer, Color, Rect, Widget};
use std::ops::Deref;
use term_rustdoc::tree::TreeLine;

impl<Lines: Deref<Target = [TreeLine]>> Widget for &mut Scrollable<Lines> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // render tree by each line
        write_lines(self.lines.as_ref(), self.start, area, buf);

        // render the current row
        if let Some(cur) = self.lines().get(self.idx_of_current_cursor()) {
            let Rect { x, width, .. } = area;
            render_current_line(cur, buf, x, self.cursor, width);
        }
    }
}

fn write_lines(lines: &[TreeLine], row_start: usize, rect: Rect, buf: &mut Buffer) {
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
    if let Some(lines) = lines.get(row_start..row_end) {
        for line in lines {
            render_line(line, buf, x, y, width);
            y += 1;
        }
    }
}

fn render_line(line: &TreeLine, buf: &mut Buffer, x: u16, y: u16, width: u16) {
    let [(glyph, g_style), (name, n_style)] = line.glyph_name();
    let (x_name, _) = buf.set_stringn(x, y, glyph, width as usize, g_style);
    buf.set_stringn(x_name, y, name, width as usize, n_style);
}

// Usually the line doesn't contain bg, thus highlight it by adding DarkGray bg on glyph
// and inversing bg the name with Black fg.
fn render_current_line(line: &TreeLine, buf: &mut Buffer, x: u16, y: u16, width: u16) {
    let [(glyph, g_style), (name, mut n_style)] = line.glyph_name();
    let (x_name, _) = buf.set_stringn(x, y, glyph, width as usize, g_style.bg(Color::DarkGray));
    n_style.bg = n_style.fg;
    n_style.fg = Some(Color::Black);
    buf.set_stringn(x_name, y, name, width as usize, n_style);
}
