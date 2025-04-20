use super::{
    code_block,
    element::{Element, FOOTNOTE},
    list::{self, parse_codeblock},
    Block, Blocks, MetaTag, Word,
};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use term_rustdoc::util::{xformat, XString};

pub fn parse(doc: &str) -> Blocks {
    if doc.is_empty() {
        return Blocks::default();
    }
    let mut blocks = Blocks::new();
    let mut iter = markdown_iter(doc);
    while let Some((event, range)) = iter.by_ref().next() {
        match event {
            Event::Start(Tag::Paragraph) => {
                let mut block = Block::default();
                let para = ele!(iter, Paragraph, range);
                Element::new(doc, &mut block, blocks.links(), para).parse_paragraph();
                blocks.push(block);
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => {
                let code_block = &doc[range.clone()];
                blocks.push(code_block::rust(code_block));
                let _ = ele!(iter, CodeBlock, range);
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(fence))) => {
                let fence = XString::from(&*fence);
                let mut block = Block::default();
                parse_codeblock(doc[range.clone()].trim(), fence, &mut block);
                blocks.push(block);
                // consume the codeblock iterator
                let _ = ele!(iter, CodeBlock, range);
            }
            Event::Start(Tag::Heading { level, .. }) => {
                let raw = &doc[range.clone()];
                let heading = ele!(#heading iter, level, range);
                let mut block = Block::default();
                let mut sharps = XString::default();
                let level = level as u8;
                (0..level).for_each(|_| sharps.push('#'));
                sharps.push(' ');
                block.push_a_word(Word {
                    word: sharps,
                    ..Default::default()
                });
                Element::new(doc, &mut block, blocks.links(), heading).parse_paragraph();
                let id = blocks.links().push_heading(level, raw);
                block.set_heading(id);
                blocks.push(block);
            }
            Event::Rule => blocks.push(Block::from_iter([Word {
                tag: MetaTag::Rule,
                ..Default::default()
            }])),
            Event::Start(Tag::Table(_)) => {
                // the table is rendered via original contents with syntect's highlights
                blocks.push(code_block::md_table(&doc[range.clone()]));
                let _ = ele!(iter, Table, range); // consume the whole table
            }
            Event::Start(Tag::BlockQuote(_)) => {
                if let Some((Event::Start(Tag::Paragraph), range)) = iter.next() {
                    let mut block = Block::default();
                    let para = ele!(iter, Paragraph, range);
                    Element::new(doc, &mut block, blocks.links(), para).parse_paragraph();
                    block.set_quote_block();
                    blocks.push(block);
                }
            }
            Event::Start(Tag::FootnoteDefinition(key)) => {
                if let Some((Event::Start(Tag::Paragraph), range)) = iter.next() {
                    let mut block = Block::default();
                    block.push_a_word(Word {
                        word: xformat!("[^{key}]: "),
                        style: FOOTNOTE,
                        tag: MetaTag::FootnoteSource,
                        trailling_whitespace: false,
                    });
                    let para = ele!(iter, Paragraph, range);
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

fn markdown_iter(
    doc: &str,
) -> pulldown_cmark::OffsetIter<'_, pulldown_cmark::DefaultBrokenLinkCallback> {
    Parser::new_ext(
        doc,
        Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TABLES
            | Options::ENABLE_TASKLISTS,
    )
    .into_offset_iter()
}

#[cfg(test)]
mod tests;
