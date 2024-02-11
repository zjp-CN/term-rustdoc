use self::{
    meta_tag::{LinkTag, MetaTag},
    word::{Block, Blocks, Line, Links, Word},
};
use super::fallback::StyledLine;
use icu_segmenter::LineSegmenter;
use itertools::Itertools;
use ratatui::style::{Color, Modifier, Style};
use syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};
use term_rustdoc::{tree::Text as StyledText, util::XString};

mod code_block;
#[macro_use]
mod element;
mod entry_point;
mod list;
mod meta_tag;
mod word;

thread_local! {
    static SYNTHEME: (SyntaxSet, ThemeSet) = (
        SyntaxSet::load_defaults_newlines(),
        ThemeSet::load_defaults(),
    );
    static SEGMENTER: LineSegmenter = LineSegmenter::new_auto();
}

/// Split a `&str` into segmented words without considering trailling whitespaces.
///
/// This is used in as-is words like intra-codes.
fn segment_str(text: &str, mut f: impl FnMut(&str)) {
    SEGMENTER.with(|seg| {
        seg.segment_str(text)
            .tuple_windows()
            .for_each(|(start, end)| f(&text[start..end]));
    })
}

/// Split a `&str` into segmented and trailling-whitespace-aware words.
///
/// This is used in context where text wrapping is applied like in normal texts.
pub fn segment_words(text: &str, mut f: impl FnMut(&str, bool)) {
    SEGMENTER.with(|seg| {
        for (start, end) in seg.segment_str(text).tuple_windows() {
            let word_with_potential_trail_whitespace = &text[start..end];
            let word = word_with_potential_trail_whitespace.trim_end_matches(' ');
            let trailling_whitespace = word_with_potential_trail_whitespace.len() != word.len();
            f(word, trailling_whitespace);
        }
    });
}

pub fn md(doc: &str) -> Vec<StyledLine> {
    let mut lines = Vec::with_capacity(128);
    SYNTHEME.with(|(ps, ts)| {
        let syntax = ps.find_syntax_by_extension("md").unwrap();
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
        for line in LinesWithEndings::from(doc) {
            let mut one_line = Vec::with_capacity(4);
            for (style, text) in h.highlight_line(line, ps).unwrap() {
                one_line.push(StyledText::new(text.into(), convert_style(style)));
            }
            one_line.push(StyledText::new_text(XString::new_inline("\n")));
            one_line.shrink_to_fit();
            lines.push(StyledLine { line: one_line });
        }
    });
    lines.shrink_to_fit();
    lines
}

fn convert_style(style: syntect::highlighting::Style) -> Style {
    let fg = style.foreground;
    // let bg = style.background;
    let fg = Some(Color::Rgb(fg.r, fg.g, fg.b));
    let add_modifier = match style.font_style {
        FontStyle::BOLD => Modifier::BOLD,
        FontStyle::UNDERLINE => Modifier::UNDERLINED,
        FontStyle::ITALIC => Modifier::ITALIC,
        _ => Modifier::empty(),
    };
    // FIXME: Don't set underline_color, because it will conflict
    // with underline style on outline.
    // FIXME: bg seems needless
    Style {
        fg,
        // bg: Some(Color::Rgb(bg.r, bg.g, bg.b)),
        add_modifier,
        ..Default::default()
    }
}
