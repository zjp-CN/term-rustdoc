use crate::app::App;
use ratatui::{prelude::Frame, text::Line, widgets::Paragraph};

pub fn render(app: &mut App, para: Option<Paragraph>, f: &mut Frame) {
    if let Some(para) = para {
        f.render_widget(para, f.size());
    }
}

pub struct Page<'doc> {
    outline: Outline<'doc>,
    content: Content<'doc>,
    navi: Navigation<'doc>,
}

/// A text panel that can be scrolled and select texts when the cursor is inside of it.
struct Scrollable<'text> {
    /// Styled texts on each line
    lines: Vec<Line<'text>>,
    /// The row position where cursor was last time
    cursor: Option<u16>,
    /// The selected text across lines
    select: Option<Selected>,
}

/// The selected texts will be rendered with original fg but grey bg.
struct Selected {
    row_start: u16,
    row_end: u16,
    col_start: u16,
    col_end: u16,
}

struct Outline<'doc> {
    display: Scrollable<'doc>,
}
struct Content<'doc> {
    display: Scrollable<'doc>,
}
struct Navigation<'doc> {
    display: Scrollable<'doc>,
}
