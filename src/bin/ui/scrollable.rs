use super::Selected;
use crate::{err, Result};
use ratatui::prelude::*;
use std::fmt;
use term_rustdoc::tree::TreeLine;

/// A text panel that can be scrolled and select texts when the cursor is inside of it.
#[derive(Default)]
pub struct Scrollable<Lines> {
    /// Styled texts on each line
    pub lines: Lines,
    /// The start of row to be displayed
    pub start: usize,
    /// The row position where cursor was last time
    pub cursor: u16,
    /// The maximum width among all lines
    pub max_windth: u16,
    /// The selected text across lines
    pub select: Option<Selected>,
    /// The widget area, usually not the full screen
    pub area: Rect,
}

impl<Line: AsRef<[TreeLine]>> Scrollable<Line> {
    pub fn lines(&self) -> &[TreeLine] {
        self.lines.as_ref()
    }

    pub fn len(&self) -> usize {
        self.lines().len()
    }
}

impl<Lines: Default + AsRef<[TreeLine]>> Scrollable<Lines> {
    pub fn new(lines: Lines, full: Rect) -> Result<Self> {
        let w = lines.as_ref().iter().map(TreeLine::width).max();
        let max_windth = w.ok_or_else(|| err!("The documentation is empty with no items."))?;
        if full.width < max_windth {
            warn!(
                full.width,
                max_windth, "Outline width exceeds the area width, so lines may be truncated."
            );
        }
        Ok(Self {
            lines,
            max_windth,
            area: full,
            ..Default::default()
        })
    }
}

impl<Lines> Scrollable<Lines> {
    /// The index the current cursor on screen points to.
    pub fn idx_of_current_cursor(&self) -> usize {
        self.cursor as usize + self.start
    }
}

impl<Lines: AsRef<[TreeLine]>> fmt::Debug for Scrollable<Lines> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Scrollable");
        s.field("lines.len", &self.len())
            .field("start", &self.start)
            .field("cursor", &self.cursor)
            .field("max_windth", &self.max_windth)
            .field("area", &self.area);
        if let Some(select) = &self.select {
            s.field("select", select);
        }
        s.finish()
    }
}

impl<Lines: AsRef<[TreeLine]>> Widget for &mut Scrollable<Lines> {
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
