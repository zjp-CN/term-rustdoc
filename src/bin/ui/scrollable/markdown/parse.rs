use super::{StyledLine, StyledLines, StyledText};
use ratatui::style::{Color, Modifier, Style};
use syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};
use term_rustdoc::util::XString;

thread_local! {
    static SYNTHEME: (SyntaxSet, ThemeSet) = (
        SyntaxSet::load_defaults_newlines(),
        ThemeSet::load_defaults(),
    );
}

pub fn md(doc: &str) -> StyledLines {
    let mut lines = Vec::with_capacity(128);
    SYNTHEME.with(|(ps, ts)| {
        let syntax = ps.find_syntax_by_extension("md").unwrap();
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
        for line in LinesWithEndings::from(doc) {
            let mut one_line = Vec::with_capacity(4);
            for (style, text) in h.highlight_line(line, &ps).unwrap() {
                one_line.push(StyledText::new(text.into(), convert_style(style)));
            }
            one_line.push(StyledText::new_text(XString::new_inline("\n")));
            one_line.shrink_to_fit();
            lines.push(StyledLine { line: one_line });
        }
    });
    lines.shrink_to_fit();
    StyledLines { lines }
}

fn convert_style(style: syntect::highlighting::Style) -> Style {
    let fg = style.foreground;
    let bg = style.background;
    let fg = Some(Color::Rgb(fg.r, fg.g, fg.b));
    let add_modifier = match style.font_style {
        FontStyle::BOLD => Modifier::BOLD,
        FontStyle::UNDERLINE => Modifier::UNDERLINED,
        FontStyle::ITALIC => Modifier::ITALIC,
        _ => Modifier::empty(),
    };
    Style {
        fg,
        bg: Some(Color::Rgb(bg.r, bg.g, bg.b)),
        underline_color: fg,
        add_modifier,
        sub_modifier: Modifier::empty(),
    }
}
