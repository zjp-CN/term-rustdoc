use crate::app::App;
use ratatui::prelude::{Buffer, Frame, Rect, Widget};
use term_rustdoc::tree::{Text, TreeLine, TreeLines};

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
    pub fn new(outline: TreeLines) -> Self {
        Page {
            outline: Outline {
                display: Scrollable::new(outline),
            },
            ..Default::default()
        }
    }

    pub fn scrolldown_outline(&mut self) {
        let len = self.outline.display.len();
        let start = &mut self.outline.display.start;
        *start = (*start + 5).min(len);
    }

    pub fn scrollup_outline(&mut self) {
        let start = &mut self.outline.display.start;
        *start = start.saturating_sub(5);
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
    cursor: Option<u16>,
    /// The selected text across lines
    select: Option<Selected>,
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
    fn new(lines: Lines) -> Self {
        Self {
            lines,
            ..Default::default()
        }
    }
}

impl<Lines: AsRef<[TreeLine]>> Widget for &mut Scrollable<Lines> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        write_lines(self.lines.as_ref(), self.start, area, buf);
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
            let [(glyph, g_style), (name, n_style)] = line.glyph_name();
            let (x_name, _) = buf.set_stringn(x, y, glyph, width as usize, g_style);
            buf.set_stringn(x_name, y, name, width as usize, n_style);
            y += 1;
        }
    }
}
