use super::ScrollTreeLines;
use ratatui::prelude::{Buffer, Color, Rect};
use term_rustdoc::tree::TreeLine;

impl ScrollTreeLines {
    pub fn render(&self, buf: &mut Buffer) {
        // if no visible lines, we won't render anything
        let Some(visible) = self.visible_lines() else {
            return;
        };

        let area = self.area;

        // render tree by each line
        write_lines(visible, area, buf);

        // render the current row
        if let Some(current_line) = self.get_line_of_current_cursor() {
            let Rect { x, y, width, .. } = area;
            render_current_line(current_line, buf, x, y + self.cursor.y, width);
        }
    }
}

fn write_lines(lines: &[TreeLine], rect: Rect, buf: &mut Buffer) {
    let Rect {
        x, mut y, width, ..
    } = rect;
    for line in lines {
        render_line(line, buf, x, y, width);
        y += 1;
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
