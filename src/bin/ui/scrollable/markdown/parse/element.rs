use super::{segment_str, Block, Color, LinkTag, Links, MetaTag, Modifier, Style, Word};
use pulldown_cmark::{Event, Tag};
use std::ops::Range;
use term_rustdoc::util::{ToCompactString, XString};

macro_rules! ele {
    ($iter:ident, $tag:ident, $range:ident) => {
        $iter
            .by_ref()
            .take_while(|(e, r)| {
                !(*e == ::pulldown_cmark::Event::End(::pulldown_cmark::TagEnd::$tag)
                    && *r == $range)
            })
            .collect::<Vec<_>>()
            .into_iter()
    };
    (#heading $iter:ident, $level:ident, $range:ident) => {
        $iter
            .by_ref()
            .take_while(|(e, r)| {
                !(*e == ::pulldown_cmark::Event::End(::pulldown_cmark::TagEnd::Heading($level))
                    && *r == $range)
            })
            .collect::<Vec<_>>()
            .into_iter()
    };
    (#list $iter:ident, $ordered:expr, $range:ident) => {
        $iter
            .by_ref()
            .take_while(|(e, r)| {
                !(*e == ::pulldown_cmark::Event::End(::pulldown_cmark::TagEnd::List($ordered))
                    && *r == $range)
            })
            .collect::<Vec<_>>()
            .into_iter()
    };
}

pub type EventRange<'doc> = (Event<'doc>, Range<usize>);

pub struct Element<'doc, 'block, 'links, I> {
    doc: &'doc str,
    iter: I,
    block: &'block mut Block,
    links: &'links mut Links,
}

impl<'doc, 'block, 'links, I> Element<'doc, 'block, 'links, I>
where
    I: Iterator<Item = EventRange<'doc>>,
{
    pub fn new(
        doc: &'doc str,
        block: &'block mut Block,
        links: &'links mut Links,
        iter: I,
    ) -> Self {
        Element {
            doc,
            iter,
            block,
            links,
        }
    }

    pub fn parse_paragraph(self) {
        let Element {
            doc,
            iter,
            block,
            links,
        } = self;
        // FIXME: We can remove some branches on List/Item. But for now, let's keep it simple.
        super::list::parse(&mut 0, None, iter, block, doc, links);
        // while let Some((event, range)) = para.next() {
        //     match event {
        //         Event::Text(text) => block.push_normal_words(&text),
        //         Event::Start(Tag::Link { dest_url, .. }) => {
        //             Element::new(doc, block, links, ele!(para, Link, range)).parse_link(&dest_url);
        //         }
        //         Event::Start(Tag::Emphasis) => {
        //             Element::new(doc, block, links, ele!(para, Emphasis, range)).parse_emphasis();
        //         }
        //         Event::Start(Tag::Strong) => {
        //             Element::new(doc, block, links, ele!(para, Strong, range)).parse_strong();
        //         }
        //         Event::Start(Tag::Strikethrough) => {
        //             Element::new(doc, block, links, ele!(para, Strikethrough, range))
        //                 .parse_strike_through();
        //         }
        //         Event::Code(intra_code) => parse_intra_code(&intra_code, block),
        //         Event::SoftBreak | Event::HardBreak => block.push_a_word(Word {
        //             // To indicate there is a whitespace in case the last word in this line
        //             // and word in next line are on the same line with whitespace separator after wrapping.
        //             trailling_whitespace: true,
        //             ..Default::default()
        //         }),
        //         Event::FootnoteReference(key) => block.push_a_word(Word {
        //             word: "[^_]".into(),
        //             style: Style {
        //                 fg: Some(Color::LightMagenta),
        //                 ..Default::default()
        //             },
        //             tag: MetaTag::Link(LinkTag::Footnote((&*key).into())),
        //             trailling_whitespace: false,
        //         }),
        //         Event::Start(Tag::Image { dest_url, .. }) => {
        //             Element::new(doc, block, links, ele!(para, Image, range))
        //                 .parse_image(&dest_url);
        //         }
        //         Event::TaskListMarker(done) => task_maker(done, block),
        //         _ => (),
        //     }
        // }
    }

    /// TODO: support local/external crate item links
    pub fn parse_link(self, link: &str) {
        let Element {
            mut iter,
            block,
            links,
            ..
        } = self;
        let idx = links.push_link(link.into());
        block.push_link(idx);
        let tag = MetaTag::Link(LinkTag::ReferenceLink(idx));
        let style = LINK;
        let alink = |word| Word {
            word,
            style,
            tag: tag.clone(),
            trailling_whitespace: false,
        };
        block.push_a_word(alink(XString::new_inline("[")));
        while let Some((event, range)) = iter.next() {
            match event {
                Event::Text(words) => {
                    block.push_words(&words, style, tag.clone());
                }
                Event::Code(code) => {
                    parse_intra_code_in_link(&code, block);
                }
                Event::Start(Tag::Emphasis) => {
                    let style = style.add_modifier(Modifier::ITALIC);
                    // we use for-loop here to discard further nested styles
                    for (event, _) in ele!(iter, Emphasis, range) {
                        if let Event::Text(words) = event {
                            block.push_words(&words, style, tag.clone());
                        }
                    }
                }
                Event::Start(Tag::Strong) => {
                    let style = style.add_modifier(Modifier::BOLD);
                    for (event, _) in ele!(iter, Strong, range) {
                        if let Event::Text(words) = event {
                            block.push_words(&words, style, tag.clone());
                        }
                    }
                }
                Event::Start(Tag::Strikethrough) => {
                    let style = style.add_modifier(Modifier::CROSSED_OUT);
                    for (event, _) in ele!(iter, Strikethrough, range) {
                        if let Event::Text(words) = event {
                            block.push_words(&words, style, tag.clone());
                        }
                    }
                }
                _ => (),
            }
        }
        block.push_a_word(alink(XString::new_inline("]")));
        block.push_a_word(alink(XString::new_inline("[")));
        block.push_a_word(alink(idx.to_compact_string()));
        block.push_a_word(alink(XString::new_inline("]")));
    }

    /// Images are like links, e.g. `![ref]` are valid syntax, or `![styled text](...)`.
    /// But when parsing them, don't show further styles and just truncate the img line if too long.
    pub fn parse_image(self, link: &str) {
        let Element { iter, block, .. } = self;
        let tag = MetaTag::Image;
        let style = Style {
            fg: Some(Color::Rgb(192, 192, 192)), // #C0C0C0
            ..Default::default()
        };
        let mut img = String::with_capacity(32);
        img.push_str("![");
        for (event, _) in iter {
            if let Event::Text(words) = event {
                img.push_str(&words)
            }
        }
        img.push_str("](");
        img.push_str(link);
        img.push(')');
        img.shrink_to_fit();
        block.push_a_word(Word {
            word: img.into(),
            style,
            tag,
            trailling_whitespace: false,
        });
    }

    /// an emphasis element can contain nested styles or other elements, but we only extract texts and
    /// apply italic style for them no matter what other styles are
    pub fn parse_emphasis(self) {
        for (event, _) in self.iter {
            if let Event::Text(text) = event {
                self.block.push_words(
                    &text,
                    Style {
                        add_modifier: Modifier::ITALIC,
                        ..Default::default()
                    },
                    MetaTag::Normal,
                );
            }
        }
    }

    pub fn parse_strong(self) {
        for (event, _) in self.iter {
            if let Event::Text(text) = event {
                self.block.push_words(
                    &text,
                    Style {
                        add_modifier: Modifier::BOLD,
                        ..Default::default()
                    },
                    MetaTag::Normal,
                );
            }
        }
    }

    pub fn parse_strike_through(self) {
        for (event, _) in self.iter {
            if let Event::Text(text) = event {
                self.block.push_words(
                    &text,
                    Style {
                        add_modifier: Modifier::CROSSED_OUT,
                        ..Default::default()
                    },
                    MetaTag::Normal,
                );
            }
        }
    }
}

const LINK: Style = Style {
    fg: Some(Color::Rgb(30, 144, 255)), // #1E90FF
    add_modifier: Modifier::empty(),
    bg: None,
    underline_color: None,
    sub_modifier: Modifier::empty(),
};

const INTRA_CODE: Style = Style {
    fg: Some(Color::Rgb(255, 184, 162)), // #FFB8A2
    bg: None,
    underline_color: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

pub fn parse_intra_code(code: &str, block: &mut Block) {
    fn word(s: &str) -> Word {
        Word {
            word: s.into(),
            style: INTRA_CODE,
            tag: MetaTag::InlineCode,
            trailling_whitespace: false,
        }
    }
    block.push_a_word(word("`"));
    segment_str(code, |s| {
        block.push_a_word(word(s));
    });
    let end = word("`");
    block.push_a_word(end);
}

pub fn parse_intra_code_in_link(code: &str, block: &mut Block) {
    fn word(s: &str, style: Style) -> Word {
        Word {
            word: s.into(),
            style,
            tag: MetaTag::InlineCode,
            trailling_whitespace: false,
        }
    }
    let tick = word("`", INTRA_CODE);
    block.push_a_word(tick.clone());
    segment_str(code, |s| {
        let word = word(s, LINK);
        block.push_a_word(word);
    });
    block.push_a_word(tick);
}
