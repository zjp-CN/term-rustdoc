use ratatui::{buffer::Buffer, style::Style};
use rustdoc_types::Id;
use std::ops::Deref;
use term_rustdoc::{tree::TreeLine, util::XString};
use unicode_width::UnicodeWidthStr;

pub trait Lines: Deref<Target = [Self::Line]> {
    type Line: LineState;
}

impl<L: LineState, Ls: Deref<Target = [L]>> Lines for Ls {
    type Line = L;
}

pub trait LineState {
    type State: PartialEq + Default;
    fn state(&self) -> Self::State;
    fn is_identical(&self, state: &Self::State) -> bool;
}

impl LineState for TreeLine {
    type State = Option<Id>;

    fn state(&self) -> Self::State {
        self.id
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        self.id == *state
    }
}

// Render a line as much as possible. Stop when the width is not enough, but still try to
// write the stoping text in the remaining width.
pub fn render_line<'t, I>(line: I, buf: &mut Buffer, mut x: u16, y: u16, width: usize) -> usize
where
    I: IntoIterator<Item = (&'t str, Style)>,
{
    let mut used_width = 0usize;
    for (text, style) in line {
        // stop rendering once it hits the end of width
        let text_width = text.width();
        if used_width + text_width > width {
            if let Some(rest) = width.checked_sub(used_width).filter(|w| *w > 0) {
                let (succeed, fail) = if let Some(text) = text.get(..rest) {
                    let (x_pos, _) = buf.set_stringn(x, y, text, width, style);
                    used_width += x_pos.saturating_sub(x) as usize;
                    x = x_pos;
                    (rest, text_width.saturating_sub(rest))
                } else {
                    (0, text_width)
                };
                warn!(
                    "{text:?} truncated in row {y} col {x} and possibly partially written. \
                     (used {used_width}, the next text_width {text_width} \
                     with {succeed} written {fail} not written, maximum {width})",
                );
            }
            return used_width;
        }
        let (x_pos, _) = buf.set_stringn(x, y, text, width, style);
        used_width += x_pos.saturating_sub(x) as usize;
        x = x_pos;
    }
    used_width
}

pub fn render_line_fill_gap<'t, I>(
    line: I,
    style: Style,
    buf: &mut Buffer,
    mut x: u16,
    y: u16,
    width: usize,
    gap_str: &mut XString,
) where
    I: IntoIterator<Item = &'t str>,
{
    let mut used_width = 0usize;
    for text in line {
        // stop rendering once it hits the end of width
        let text_width = text.width();
        if used_width + text_width > width {
            if let Some(rest) = width.checked_sub(used_width).filter(|w| *w > 0) {
                let (succeed, fail) = if let Some(text) = text.get(..rest) {
                    let (x_pos, _) = buf.set_stringn(x, y, text, width, style);
                    used_width += x_pos.saturating_sub(x) as usize;
                    x = x_pos;
                    (rest, text_width.saturating_sub(rest))
                } else {
                    (0, text_width)
                };
                warn!(
                    "{text:?} truncated in row {y} col {x} and possibly partially written. \
                     (used {used_width}, the next text_width {text_width} \
                     with {succeed} written {fail} not written, maximum {width})",
                );
            }
        }
        let (x_pos, _) = buf.set_stringn(x, y, text, width, style);
        used_width += x_pos.saturating_sub(x) as usize;
        x = x_pos;
    }
    if let Some(gap) = width.checked_sub(used_width).filter(|gap| *gap > 0) {
        gap_str.clear();
        (0..gap).for_each(|_| gap_str.push(' '));
        buf.set_stringn(x, y, gap_str, gap, style);
    }
}
