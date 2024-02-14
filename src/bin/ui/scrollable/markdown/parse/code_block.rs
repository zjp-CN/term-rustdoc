// fenced codeblock including the tags and snippet are special in rustdoc:
// * empty fence tag means Rust code
// * extra tags following `rust,` hint extra rendering
// * code snippet beginning with `# ` is hidden as default

use super::{convert_style, Block, Line, MetaTag, Word, SYNTHEME};
use ratatui::style::{Color, Style};
use syntect::{easy::HighlightLines, util::LinesWithEndings};

pub fn parse(fence: &mut str, code: &str) -> Block {
    fence.make_ascii_lowercase();
    match &*fence {
        "" | "rust" | "rs" => rust(code),
        _ => other(fence, code),
    }
}

fn word(text: &str, style: syntect::highlighting::Style) -> Word {
    Word {
        word: text.into(),
        style: convert_style(style),
        tag: MetaTag::CodeBlock("rust".into()),
        trailling_whitespace: false,
    }
}

/// If the lang is not in SyntaxSet, first fall back to Rust lang, then this one.
#[cold]
fn fallback(code: &str) -> Block {
    code.lines()
        .map(|line| Word {
            word: line.into(),
            style: Style {
                fg: Some(Color::LightRed),
                ..Default::default()
            },
            tag: MetaTag::CodeBlock("Unknown".into()),
            trailling_whitespace: false,
        })
        .collect()
}

pub fn rust(code: &str) -> Block {
    SYNTHEME.with(|(ps, ts)| {
        let Some(syntax) = ps.find_syntax_by_name("Rust") else {
            return fallback(code);
        };
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
        let mut lines = Vec::with_capacity(8);
        // filter out the lines starting `# ` used for hidden lines
        for line in code.lines().filter(|l| !l.starts_with("# ")) {
            let mut words = Vec::with_capacity(8);
            for (style, text) in h.highlight_line(line, ps).unwrap() {
                words.push(word(text, style));
            }
            lines.push(Line::from_iter(words));
        }
        let mut block = Block::from_iter(lines);
        block.shrink_to_fit();
        block
    })
}

macro_rules! gen_parse_code {
    ($( $fname:ident ),+) => { $(
        pub fn $fname(code: &str) -> Block {
            SYNTHEME.with(|(ps, ts)| {
                let Some(syntax) = ps.find_syntax_by_name(stringify!($fname)) else {
                    return rust(code);
                };
                gen_parse_code! { #inner code ps ts syntax }
            })
        }
        )+ };
    (#inner $code:ident $ps:ident $ts:ident $syntax:ident) => {
        let mut h = HighlightLines::new($syntax, &$ts.themes["base16-ocean.dark"]);
        let mut lines = Vec::with_capacity(8);
        for line in LinesWithEndings::from($code) {
            let mut words = Vec::with_capacity(8);
            for (style, text) in h.highlight_line(line, $ps).unwrap() {
                words.push(word(text, style));
            }
            lines.push(Line::from_iter(words));
        }
        let mut block = Block::from_iter(lines);
        block.shrink_to_fit();
        block
    };
}

/// If the lang is not found by file extention, use Rust as fallback.
pub fn other(lang: &str, code: &str) -> Block {
    SYNTHEME.with(|(ps, ts)| {
        let Some(syntax) = ps.find_syntax_by_extension(lang) else {
            return rust(code);
        };
        gen_parse_code! { #inner code ps ts syntax }
    })
}

gen_parse_code!(markdown);

pub fn md_table(table: &str) -> Block {
    markdown(table)
}
