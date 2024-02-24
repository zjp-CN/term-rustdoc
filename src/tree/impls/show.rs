use crate::{
    tree::{IDMap, IdAsStr, Tag, TextTag, ID},
    util::XString,
};
use std::fmt;
use termtree::Tree;

/// Construct a glyph possibly with custom ident text.
/// This is a macro because GlyphPalette needs &'static str.
macro_rules! icon {
    () => {
        ::termtree::GlyphPalette::new()
    };
    ("") => {
        ::termtree::GlyphPalette::new()
    };
    ($s:literal) => {
        ::termtree::GlyphPalette {
            item_indent: ::constcat::concat!("── ", $s, " "),
            ..Default::default()
        }
    };
    (@fold $s:literal) => {
        ::termtree::GlyphPalette {
            item_indent: ::constcat::concat!("─➤ ", $s, " "),
            last_item: "╰",
            ..Default::default()
        }
    };
}

/// Doc node in a display tree.
#[derive(Clone)]
pub struct DocTree {
    pub tree: Tree<TextTag>,
}

impl fmt::Display for DocTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tree)
    }
}

impl std::iter::Extend<DocTree> for DocTree {
    fn extend<T: IntoIterator<Item = DocTree>>(&mut self, iter: T) {
        self.tree.extend(iter.into_iter().map(|t| t.tree));
    }
}

impl DocTree {
    pub fn new(text: XString, tag: Tag, id: Option<XString>) -> Self {
        Self {
            tree: Tree::new(TextTag { text, tag, id }).with_glyphs(tag.glyph()),
        }
    }
    pub fn with_leaves(mut self, leaves: impl IntoIterator<Item = Self>) -> Self {
        self.tree = self.tree.with_leaves(leaves.into_iter().map(|t| t.tree));
        self
    }
    pub fn push(&mut self, node: Self) {
        self.tree.push(node.tree);
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
        DocTree::new(self.into(), Tag::Unknown, None)
    }

    /// Just as `<str as Show>::show` does.
    fn show_prettier(&self, _: &IDMap) -> DocTree {
        self.show()
    }
}

/// @name is for short name; no @ is for name with absolute path
macro_rules! node {
    // map.path(&self.id, ItemKind::Struct)
    ($tag:ident : $map:ident, $id:expr) => {
        $crate::tree::DocTree::new($map.path($id), $crate::tree::Tag::$tag, Some($id.into()))
    };
    ($tag:ident : $map:ident, $kind:ident, $id:expr) => {
        $crate::tree::DocTree::new($map.path($id), $crate::tree::Tag::$tag, Some($id.into()))
    };
    (@name $tag:ident : $map:ident, $id:expr) => {
        $crate::tree::DocTree::new($map.name($id), $crate::tree::Tag::$tag, Some($id.into()))
    };
}

pub fn show_names<'id, S: 'id + ?Sized + IdAsStr>(
    ids: impl 'id + IntoIterator<Item = &'id S>,
    tag: Tag,
    map: &'id IDMap,
) -> impl 'id + Iterator<Item = DocTree> {
    ids.into_iter()
        .map(move |id| DocTree::new(map.name(id), tag, Some(id.id_str().into())))
}

/// ### Usage 1
///
/// ````rust,ignore
/// let node = Tag::Implementations.show();
/// let leaves = names_node!(self map node,
///     InherentImpls inherent ImplInherent,
///     TraitImpls    trait_   ImplTrait,
///     AutoImpls     auto     ImplAuto,
///     BlanketImpls  blanket  ImplBlanket,
/// );
/// node.with_leaves(leaves)
/// ````
///
/// ### Usage 2
///
/// ````rust,ignore
/// let root = node!(Union: map.path(&self.id, ItemKind::Union));
/// let fields = names_node!(@single
///     self map root.with_leaves([Tag::NoFields.show()]),
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
