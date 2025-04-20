use super::{
    code_block,
    element::{parse_intra_code, Element, EventRange, FOOTNOTE},
    meta_tag::{LinkTag, MetaTag},
    Block, Line, Links, Word,
};
use pulldown_cmark::{CodeBlockKind, Event, Tag};
use ratatui::style::{Color, Style};
use term_rustdoc::util::{xformat, XString};

pub fn parse<'doc, I>(
    level: &mut u8,
    mut kind: Option<u64>,
    mut iter: I,
    block: &mut Block,
    doc: &'doc str,
    links: &mut Links,
) where
    I: Iterator<Item = EventRange<'doc>>,
{
    while let Some((event, range)) = iter.next() {
        match event {
            Event::Text(text) => block.push_normal_words(&text),
            Event::Start(Tag::Link { dest_url, .. }) => {
                Element::new(doc, block, links, ele!(iter, Link, range)).parse_link(&dest_url);
            }
            Event::Start(Tag::Emphasis) => {
                Element::new(doc, block, links, ele!(iter, Emphasis, range)).parse_emphasis();
            }
            Event::Start(Tag::Strong) => {
                Element::new(doc, block, links, ele!(iter, Strong, range)).parse_strong();
            }
            Event::Start(Tag::Strikethrough) => {
                Element::new(doc, block, links, ele!(iter, Strikethrough, range))
                    .parse_strike_through();
            }
            Event::Code(intra_code) => parse_intra_code(&intra_code, block),
            Event::SoftBreak | Event::HardBreak => block.push_a_word(Word {
                // To indicate there is a whitespace in case the last word in this line
                // and word in next line are on the same line with whitespace separator after wrapping.
                trailling_whitespace: true,
                ..Default::default()
            }),
            Event::FootnoteReference(key) => {
                let key = XString::new(&key);
                let tag = MetaTag::Link(LinkTag::Footnote(key.clone()));
                let footnote = |word| Word {
                    word,
                    style: FOOTNOTE,
                    tag: tag.clone(),
                    trailling_whitespace: false,
                };
                // push [^key] where each word can be wrapped
                block.push_a_word(footnote("[".into()));
                block.push_a_word(footnote("^".into()));
                block.push_a_word(footnote(key.clone()));
                block.push_a_word(footnote("]".into()));
                block.push_footnote(key);
            }
            Event::Start(Tag::Image { dest_url, .. }) => {
                Element::new(doc, block, links, ele!(iter, Image, range)).parse_image(&dest_url);
            }

            // List and Item
            Event::Start(Tag::Item) => {
                let words = item_prefix(level, kind.as_mut());
                block.extend([Line::from_iter(words)]); // each item occupies a new line
                let item_iter = ele!(iter, Item, range);
                let mut level = *level; // start on a new level
                parse(&mut level, kind, item_iter, block, doc, links);
            }
            Event::TaskListMarker(done) => task_maker(done, block),
            // Item doesn't contain a Paragraph, but we can reuse the parsing though
            Event::Start(Tag::List(kind)) => {
                *level += 1;
                let list = ele!(#list iter, kind.is_some(), range);
                parse(level, kind, list, block, doc, links);
            }

            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => {
                let code_block = &doc[range.clone()];
                block.push_code_block(code_block::rust(code_block));
                let _ = ele!(iter, CodeBlock, range);
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(fence))) => {
                let fence = XString::from(&*fence);
                parse_codeblock(doc[range.clone()].trim(), fence, block);
                // consume the codeblock iterator
                let _ = ele!(iter, CodeBlock, range);
            }

            // This case is less likely due to Start List -> Start Item -> TaskListMarker -> Text
            Event::Start(Tag::Paragraph) => {
                let para = ele!(iter, Paragraph, range);
                Element::new(doc, block, links, para).parse_paragraph();
            }
            _ => (),
        }
    }
}

/// Codebock in nest lists are parsed into multiple lines by pulldown_cmark.
/// So here we use the raw text and parse the backticks instead.
/// The content code will be rendered by syntect directly.
pub fn parse_codeblock(mut codeblock: &str, mut fence: XString, block: &mut Block) {
    // NOTE: the starting backticks in range doesn't contain leading whitespaces,
    // thus use the ending backticks istead.
    let mut backticks = "```";
    if let Some(backtick) = codeblock.rfind("```") {
        if let Some(last_line) = codeblock[..backtick].rfind('\n') {
            backticks = &codeblock[last_line + 1..];
            if let Some(first_line) = codeblock.find('\n') {
                codeblock = &codeblock[first_line + 1..last_line];
            }
        }
    }
    let [start, end] = Line::backtick(backticks, fence.clone());
    block.extend([start]);
    block.push_code_block(code_block::parse(&mut fence, codeblock));
    block.extend([end, Line::default()]);
}

fn task_maker(done: bool, block: &mut Block) {
    let task = if done {
        Word {
            word: "[x]".into(),
            style: Style {
                fg: Some(Color::LightYellow),
                ..Default::default()
            },
            ..Default::default()
        }
    } else {
        Word {
            word: "[ ]".into(),
            ..Default::default()
        }
    };
    block.extend([
        task,
        Word {
            trailling_whitespace: true,
            ..Default::default()
        },
    ]);
}

fn item_prefix(level: &mut u8, kind: Option<&mut u64>) -> [Word; 2] {
    let (word, tag) = match kind {
        Some(num) => {
            let word_tag = (xformat!("{num}. "), MetaTag::ListItemN(*num as u8));
            *num += 1;
            word_tag
        }
        None => ("* ".into(), MetaTag::ListItem),
    };
    [
        Word {
            word: {
                let mut ident = XString::const_new("");
                (0..*level).for_each(|_| ident.push_str("  "));
                ident
            },
            ..Default::default()
        },
        Word {
            word,
            style: Style {
                fg: Some(Color::Green),
                ..Default::default()
            },
            tag,
            ..Default::default()
        },
    ]
}
