use super::ScrollTreeLines;
use ratatui::prelude::{Buffer, Color, Rect};
use term_rustdoc::tree::TreeLine;

impl ScrollTreeLines {
    pub fn render(&self, buf: &mut Buffer) {
        // if no visible lines, we won't render anything
        let Some(visible) = self.visible_lines() else {
            return;
        };

        let Rect { x, y, .. } = self.area;
        let width = self.area.width as usize;

        // render tree by each line
        write_lines(visible, buf, x, y, width);

        // render the current row
        if let Some(current_line) = self.get_line_of_current_cursor() {
            render_current_line(current_line, buf, x, y + self.cursor.y, width);
        }
    }
}

fn write_lines(lines: &[TreeLine], buf: &mut Buffer, x: u16, mut y: u16, width: usize) {
    for line in lines {
        render_line(line, buf, x, y, width);
        y += 1;
    }
}

fn render_line(line: &TreeLine, buf: &mut Buffer, x: u16, y: u16, width: usize) {
    let [(glyph, g_style), (name, n_style)] = line.glyph_name();
    let (x_name, _) = buf.set_stringn(x, y, glyph, width, g_style);
    if let Some(remain) = width.checked_sub((x_name - x) as usize) {
        buf.set_stringn(x_name, y, name, remain, n_style);
    }
}

// Usually the line doesn't contain bg, thus highlight it by adding DarkGray bg on glyph
// and inversing bg the name with Black fg.
fn render_current_line(line: &TreeLine, buf: &mut Buffer, x: u16, y: u16, width: usize) {
    let [(glyph, g_style), (name, mut n_style)] = line.glyph_name();
    let (x_name, _) = buf.set_stringn(x, y, glyph, width, g_style.bg(Color::DarkGray));
    n_style.bg = n_style.fg;
    n_style.fg = Some(Color::Black);
    if let Some(remain) = width.checked_sub((x_name - x) as usize) {
        buf.set_stringn(x_name, y, name, remain, n_style);
    }
}
