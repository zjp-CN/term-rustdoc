use crate::app::App;
use ratatui::{
    prelude::{Buffer, Frame, Rect, Widget},
    style::Color,
};
use term_rustdoc::tree::{Text, TreeLine, TreeLines};

/// scroll up/down behavior and with what offset
mod page_scroll;

pub use page_scroll::ScrollOffset;

pub fn render(app: &mut App, page: &mut Page, f: &mut Frame) {
    f.render_widget(page, f.size());
}

#[derive(Default)]
pub struct Page {
    outline: Outline,
    content: Content,
    navi: Navigation,
}

impl Page {
    pub fn new(outline: TreeLines, full: Rect) -> Self {
        Page {
            outline: Outline {
                display: Scrollable::new(outline, full),
            },
            ..Default::default()
        }
    }
}

impl Widget for &mut Page {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.outline.display.render(area, buf);
    }
}

/// A text panel that can be scrolled and select texts when the cursor is inside of it.
#[derive(Default)]
struct Scrollable<Lines> {
    /// Styled texts on each line
    lines: Lines,
    /// The start of row to be displayed
    start: usize,
    /// The row position where cursor was last time
    cursor: u16,
    /// The selected text across lines
    select: Option<Selected>,
    area: Rect,
}

impl<Line: AsRef<[TreeLine]>> Scrollable<Line> {
    pub fn lines(&self) -> &[TreeLine] {
        self.lines.as_ref()
    }

    pub fn len(&self) -> usize {
        self.lines().len()
    }
}

impl<Lines: Default> Scrollable<Lines> {
    fn new(lines: Lines, full: Rect) -> Self {
        Self {
            lines,
            area: full,
            ..Default::default()
        }
    }
}

impl<Lines> Scrollable<Lines> {
    /// The index the current cursor on screen points to.
    pub fn idx_of_current_cursor(&self) -> usize {
        self.cursor as usize + self.start
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

/// The selected texts will be rendered with original fg but grey bg.
#[derive(Default)]
struct Selected {
    row_start: u16,
    row_end: u16,
    col_start: u16,
    col_end: u16,
}

#[derive(Default)]
struct Outline {
    display: Scrollable<TreeLines>,
}

#[derive(Default)]
struct Content {
    display: Scrollable<Vec<Text>>,
}

#[derive(Default)]
struct Navigation {
    display: Scrollable<TreeLines>,
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
