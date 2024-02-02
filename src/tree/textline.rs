use self::fold::Fold;
use crate::{
    tree::{DModule, DocTree, IDMap, Show, Tag},
    util::XString,
};
use ratatui::style::{Color, Style};
use std::{
    fmt::{self, Write},
    rc::Rc,
};
use termtree::Tree;
use unicode_width::UnicodeWidthStr;

mod fold;

/// Tagged text including headings and nodes.
#[derive(Clone)]
pub struct TextTag {
    pub text: XString,
    pub tag: Tag,
    pub id: Option<XString>,
}

/// Show text only, which is used as a plain text Tree display.
impl fmt::Display for TextTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

/// Onwed [`ratatui::text::Span`], mainly being a small styled string.
#[derive(Debug, Clone)]
pub struct Text {
    pub text: XString,
    pub style: Style,
}

impl Text {
    pub fn new(text: XString, style: Style) -> Self {
        Self { text, style }
    }

    pub fn new_text(text: XString) -> Self {
        Text {
            text,
            style: Style::default(),
        }
    }
}

#[derive(Clone)]
pub struct TreeLine {
    pub glyph: Text,
    pub tag: Tag,
    /// Identation level with range of 0..=u8::MAX
    pub level: u8,
    /// Node/Item id from Crate
    pub id: Option<XString>,
    pub name: Text,
}

impl fmt::Debug for TreeLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TreeLine")
            .field("tag", &self.tag)
            .field("level", &self.level)
            .field("name.text", &self.name.text)
            .finish()
    }
}

impl TreeLine {
    fn flatten(tree: DocTree) -> (Vec<TreeLine>, Tree<Empty>) {
        let mut level = 0u8;
        let mut emptytree = Tree::new(Empty).with_glyphs(tree.tree.root.tag.glyph());
        let root = TreeLine::new(tree.tree.root, level);
        let mut flatten = Vec::<TreeLine>::with_capacity(512);
        flatten.push(root);
        empty_tree_and_flatten(tree.tree.leaves, &mut level, &mut flatten, &mut emptytree);
        (flatten, emptytree)
    }

    pub fn new(tt: TextTag, level: u8) -> Self {
        let text = tt.text;
        let tag = tt.tag;
        let id = tt.id;
        let style = tag.style();
        let name = Text { text, style };

        // stripe placeholder
        let glyph = Text {
            text: Default::default(),
            style: Default::default(),
        };
        Self {
            glyph,
            tag,
            level,
            id,
            name,
        }
    }

    fn set_glyph(&mut self, glyph: XString) {
        self.glyph.text = glyph;
        self.glyph.style = Style::default().fg(Color::Gray);
    }

    /// texts and styles of glyph and name
    pub fn glyph_name(&self) -> [(&str, Style); 2] {
        [
            (&self.glyph.text, self.glyph.style),
            (&self.name.text, self.name.style),
        ]
    }

    /// non-cjk unicode width including glyph and name
    ///
    /// reason for non-cjk:
    /// * path or name usually doesn't contain CJK
    /// * CJK width counts glyph width more, leading to wasteful space in outline
    pub fn width(&self) -> u16 {
        let (g, n) = (&*self.glyph.text, &*self.name.text);
        (g.width() + n.width())
            .try_into()
            .unwrap_or_else(|_| panic!("The total width exceeds u16::MAX in `{g}{n}`"))
    }
}

pub struct TreeLines {
    lines: Rc<[TreeLine]>,
    tree: Rc<DModule>,
    fold: Option<Fold>,
}

impl TreeLines {
    pub fn new(dmod: DModule, map: &IDMap) -> (Self, Tree<Empty>) {
        let doc_tree = dmod.show_prettier(map);
        let dmod_tree = Rc::new(dmod);
        let (mut lines, layout) = TreeLine::flatten(doc_tree);
        let tree_glyph = glyph(&layout);

        let (len_nodes, len_glyph) = (lines.len(), tree_glyph.len());
        assert_eq!(
            len_nodes, len_glyph,
            "the amount of nodes is {len_nodes}, but that of glyph is {len_glyph}"
        );

        lines
            .iter_mut()
            .zip(tree_glyph)
            .for_each(|(l, g)| l.set_glyph(g));

        (
            TreeLines {
                lines: lines.into(),
                tree: dmod_tree,
                fold: None,
            },
            layout,
        )
    }

    pub fn all_lines(&self) -> &[TreeLine] {
        &self.lines
    }

    /// This should exactly be the same as DocTree's display.
    pub fn display_as_plain_text(&self) -> String {
        struct DisplayPlain<'s> {
            glyph: &'s str,
            name: &'s str,
        }
        impl fmt::Display for DisplayPlain<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                writeln!(f, "{}{}", self.glyph, self.name)
            }
        }
        let mut buf = String::with_capacity(self.len() * 50);
        for l in &*self.lines {
            let plain = DisplayPlain {
                glyph: &l.glyph.text,
                name: &l.name.text,
            };
            write!(&mut buf, "{}", plain).unwrap();
        }
        buf.shrink_to_fit();
        buf
    }

    pub fn modules_tree(&self) -> Rc<DModule> {
        self.tree.clone()
    }
}

/// NOTE: `TreeLines` derefs to slices to be shown, so
/// if folding is set, derefs to incomplete `&[TreeLine]`;
/// if no item is satisfied in fold filtering, derefs to complete `&[TreeLine]`;
/// otherwise, derefs to complete `&[TreeLine]`
impl std::ops::Deref for TreeLines {
    type Target = [TreeLine];

    fn deref(&self) -> &Self::Target {
        self.fold
            .as_ref()
            .and_then(Fold::lines)
            .unwrap_or(&self.lines)
    }
}

impl Default for TreeLines {
    fn default() -> Self {
        TreeLines {
            lines: Rc::new([]),
            tree: Rc::new(DModule::default()),
            fold: None,
        }
    }
}

/// This type is displayed as empty because it's used
/// as an empty node in Tree display to get the tree stripe glyph only.
pub struct Empty;

impl fmt::Display for Empty {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

fn empty_tree_and_flatten(
    leaves: Vec<Tree<TextTag>>,
    level: &mut u8,
    flatten: &mut Vec<TreeLine>,
    empty: &mut Tree<Empty>,
) {
    *level += 1;
    // each tree node must have its root, so only looks into its leaves
    for tree in leaves {
        let mut current = *level;
        let glyph = tree.root.tag.glyph();

        // append the root of subtree
        flatten.push(TreeLine::new(tree.root, current));

        // construct new empty tree
        let mut root = Tree::new(Empty).with_glyphs(glyph);
        empty_tree_and_flatten(tree.leaves, &mut current, flatten, &mut root);
        empty.push(root);
    }
}

fn glyph(layout: &Tree<Empty>) -> Vec<XString> {
    let mut buf = String::with_capacity(1024);
    write!(&mut buf, "{layout}").expect("can't write Tree<Empty> to the string buf");
    buf.lines().map(XString::from).collect()
}
