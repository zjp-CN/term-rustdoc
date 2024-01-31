use crate::{
    tree::{IDMap, IdAsStr, Tag, ID},
    util::XString,
};
use ratatui::style::Style;
use std::{
    fmt::{self, Write},
    rc::Rc,
};
use termtree::Tree;

macro_rules! icon {
    () => { ::termtree::GlyphPalette::new() };
    ("") => { ::termtree::GlyphPalette::new() };
    ($s:literal) => {
        ::termtree::GlyphPalette {
            item_indent: ::constcat::concat!("── ", $s, " "),
            middle_item: "├",
            last_item: "└",
            middle_skip: "│",
            last_skip: " ",
            skip_indent: "   ",
        }
    };
    ($( $name:ident = $s:literal ),+ $(,)?) => {
        $(
            pub const $name: ::termtree::GlyphPalette = icon!($s);
        )+
    };
}

/// Doc node in a display tree.
pub struct DocTree {
    tree: Tree<TextTag>,
}

impl fmt::Display for DocTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tree)
    }
}

impl DocTree {
    pub fn new(text: XString, tag: Tag) -> Self {
        Self {
            tree: Tree::new(TextTag { text, tag }).with_glyphs(tag.glyph()),
        }
    }
    pub fn with_leaves(mut self, leaves: impl IntoIterator<Item = Self>) -> Self {
        self.tree = self.tree.with_leaves(leaves.into_iter().map(|t| t.tree));
        self
    }
    pub fn push(&mut self, node: Self) {
        self.tree.push(node.tree);
    }
    pub fn into_treelines(self) -> TreeLines {
        TreeLines::new(self).0
    }
}

/// Display a node as a tree component in multiple forms.
pub trait Show {
    /// A plain form usually with basic info.
    fn show(&self) -> DocTree;

    /// A fancier form with more item tags/icons before subnodes and other improvements.
    fn show_prettier(&self, map: &IDMap) -> DocTree;
}

impl Show for str {
    fn show(&self) -> DocTree {
        DocTree::new(self.into(), Tag::Unknown)
    }

    /// Just as `<str as Show>::show` does.
    fn show_prettier(&self, _: &IDMap) -> DocTree {
        self.show()
    }
}

macro_rules! node {
    ($tag:ident : $xstring:expr) => {
        $crate::tree::DocTree::new($xstring, $crate::tree::Tag::$tag)
    };
}

pub fn show_names<'id, S: 'id + ?Sized + IdAsStr>(
    ids: impl 'id + IntoIterator<Item = &'id S>,
    tag: Tag,
    map: &'id IDMap,
) -> impl 'id + Iterator<Item = DocTree> {
    ids.into_iter()
        .map(move |id| DocTree::new(map.name(id), tag))
}

/// ### Usage 1
///
/// ````rust,ignore
/// let node = "No Implementations!".show();
/// let leaves = names_node!(self map node,
///     "Inherent Impls" inherent "[inhrt]",
///     "Trait Impls"    trait_   "[trait]",
///     "Auto Impls"     auto     "[auto]",
///     "Blanket Impls"  blanket  "[blkt]",
/// );
/// node.with_leaves(leaves)
/// ````
///
/// ### Usage 2
///
/// ````rust,ignore
/// let root = node!("[union] {}", map.path(&self.id, ItemKind::Union));
/// let fields = names_node!(@single
///     self map root.with_leaves(["No Fields!".show()]),
///     "Fields" fields "[field]"
/// );
/// root.with_leaves([fields, self.impls.show_prettier(map)])
/// ````
macro_rules! names_node {
    (
        $self:ident $map:ident $root:expr ,
        $( $node:ident $field:ident $tag:ident , )+ $(,)?
    ) => {{
        if $( $self.$field.is_empty() )&&+ { return $root }
        ::std::iter::empty()
            $(
                .chain(names_node!(@chain $node $field $tag $self $map))
            )+
    }};
    (@chain $node:ident $field:ident $tag:ident $self:ident $map:ident) => {
        (!$self.$field.is_empty()).then(|| {
            $crate::tree::Tag::$node.show().with_leaves($crate::tree::impls::show::show_names(
                &*$self.$field, $crate::tree::Tag::$tag, $map
            ))
        })
    };
    (@single $self:ident $map:ident $root:ident , $node:ident $field:ident $tag:ident) => {
        if $self.$field.is_empty() {
            $crate::tree::Tag::$root.show()
        } else {
            $crate::tree::Tag::$node.show().with_leaves($crate::tree::impls::show::show_names(
                &*$self.$field, $crate::tree::Tag::$tag, $map
            ))
        }
    };
}

// pub fn show_paths<'id, S: 'id + ?Sized + IdAsStr>(
//     ids: impl 'id + IntoIterator<Item = &'id S>,
//     kind: ItemKind,
//     glyph: GlyphPalette,
//     map: &'id IDMap,
// ) -> impl 'id + Iterator<Item = DocTree> {
//     ids.into_iter()
//         .map(move |id| Tree::new(map.path(id, &kind)).with_glyphs(glyph))
// }

pub fn show_ids(ids: &[ID]) -> impl '_ + Iterator<Item = DocTree> {
    ids.iter().map(|id| id.as_str().show())
}

/// Tagged text including headings and nodes.
pub struct TextTag {
    text: XString,
    tag: Tag,
}

impl fmt::Display for TextTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

/// Onwed [`ratatui::text::Span`], mainly being a small styled string.
pub struct Text {
    pub text: XString,
    pub style: Style,
}

pub struct TreeLine {
    pub glyph: Text,
    pub tag: Tag,
    /// Identation level with range of 0..=u8::MAX
    pub level: u8,
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
            name,
        }
    }

    fn set_glyph(&mut self, glyph: XString) {
        self.glyph.text = glyph;
    }

    pub fn glyph_name(&self) -> [(&str, Style); 2] {
        [
            (&self.glyph.text, self.glyph.style),
            (&self.name.text, self.name.style),
        ]
    }
}

pub struct TreeLines {
    tree: Rc<[TreeLine]>,
    collapse: Option<usize>,
}

impl TreeLines {
    pub fn new(tree: DocTree) -> (Self, Tree<Empty>) {
        let (mut lines, layout) = TreeLine::flatten(tree);
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
                tree: lines.into(),
                collapse: None,
            },
            layout,
        )
    }

    pub fn lines(&self) -> &[TreeLine] {
        &self.tree
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.tree.len()
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
        for l in &*self.tree {
            let plain = DisplayPlain {
                glyph: &l.glyph.text,
                name: &l.name.text,
            };
            write!(&mut buf, "{}", plain).unwrap();
        }
        buf.shrink_to_fit();
        buf
    }
}

impl AsRef<[TreeLine]> for TreeLines {
    fn as_ref(&self) -> &[TreeLine] {
        self.lines()
    }
}

impl Default for TreeLines {
    fn default() -> Self {
        TreeLines {
            tree: Rc::new([]),
            collapse: None,
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
