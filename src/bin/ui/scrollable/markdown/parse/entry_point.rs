use super::{
    code_block, element::Element, list, segment_str, Block, Blocks, Line, LinkTag, Links, MetaTag,
    Word,
};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use ratatui::style::{Color, Modifier, Style};
use std::ops::Range;
use term_rustdoc::util::XString;

pub fn parse(doc: &str) -> Blocks {
    if doc.is_empty() {
        return Blocks::default();
    }
    let mut blocks = Blocks::new();
    let mut iter = Parser::new_ext(
        doc,
        Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TABLES
            | Options::ENABLE_TASKLISTS,
    )
    .into_offset_iter();
    while let Some((event, range)) = iter.by_ref().next() {
        match event {
            Event::Start(Tag::Paragraph) => {
                let mut block = Block::default();
                let mut para = ele!(iter, Paragraph, range);
                Element::new(doc, &mut block, blocks.links(), para).parse_paragraph();
                blocks.push(block);
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => {
                if let Some((Event::Text(code_block), _)) = iter.next() {
                    blocks.push(code_block::rust(&code_block));
                }
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(fence))) => {
                if let Some((Event::Text(code_block), _)) = iter.next() {
                    blocks.push(code_block::parse(&mut XString::from(&*fence), &code_block));
                }
            }
            Event::Start(Tag::Heading { level, .. }) => {
                while let Some(event) = ele!(#heading iter, level, range).next() {
                    if let (Event::Text(text), _) = event {
                        blocks.push(Block::heading(level as u8, &text));
                    }
                }
            }
            Event::Rule => blocks.push(Block::from_iter([Word {
                tag: MetaTag::Rule,
                ..Default::default()
            }])),
            Event::Start(Tag::Table(_)) => {
                // the table is rendered via original contents with syntect's highlights
                blocks.push(code_block::md_table(&doc[range.clone()]));
                ele!(iter, Table, range).last(); // consume the whole table
            }
            Event::Start(Tag::BlockQuote) => {
                if let Some((Event::Start(Tag::Paragraph), range)) = iter.next() {
                    let mut block = Block::default();
                    let mut para = ele!(iter, Paragraph, range);
                    Element::new(doc, &mut block, blocks.links(), para).parse_paragraph();
                    block.set_quote_block();
                    blocks.push(block);
                }
            }
            Event::Start(Tag::FootnoteDefinition(key)) => {
                if let Some((Event::Start(Tag::Paragraph), range)) = iter.next() {
                    let mut block = Block::default();
                    let mut para = ele!(iter, Paragraph, range);
                    Element::new(doc, &mut block, blocks.links(), para).parse_paragraph();
                    block.set_foot_note();
                    blocks.links().push_footnote(&key, block);
                }
            }
            Event::Start(Tag::List(kind)) => {
                let iter = ele!(#list iter, kind.is_some(), range);
                let mut block = Block::default();
                list::parse(&mut 0, kind, iter, &mut block, doc, blocks.links());
                blocks.push(block);
            }
            _ => (),
        }
    }
    blocks.shrink_to_fit();
    blocks
}

#[test]
fn parse_markdown() {
    let doc = r#"
aaa b *c* d **e**. ~xxx~ z.

1 *c **ss** d sadsad xxx* `yyyy`

```
let a = 1;
```

> rrr sss 
> tt

- [x] done!
    - nested list
- [ ] undone
    1. *a*
    2. `b`
"#;
    parse(doc);
    insta::assert_display_snapshot!(parse(doc), @r###"
    aaa b c d e. xxx z.

    1 c ss d sadsad xxx `yyyy`

    let a = 1;

    rrr sss tt

    * [x] done!
      * nested list
    * [ ] undone
      1. a
      2. `b`

    "###);
}
