use crate::{
    tree::{IDMap, IdAsStr, ID},
    util::XString,
};
pub use termtree::{GlyphPalette, Tree};

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
pub type DocTree = Tree<XString>;

/// Display a node as a tree component in multiple forms.
pub trait Show {
    /// A plain form usually with basic info.
    fn show(&self) -> DocTree;

    /// A fancier form with more item tags/icons before subnodes and other improvements.
    fn show_prettier(&self, map: &IDMap) -> DocTree;
}

impl Show for str {
    fn show(&self) -> DocTree {
        DocTree::new(self.into())
    }

    /// Just as `<str as Show>::show` does.
    fn show_prettier(&self, _: &IDMap) -> DocTree {
        self.show()
    }
}

macro_rules! node {
    ($xstring:expr) => { $crate::tree::impls::show::DocTree::new($xstring.into()) };
    ($e:literal $(, $($t:tt)*)?) => {
        DocTree::new($crate::util::xformat!( $e $(, $($t)*)? ))
    };
}

pub fn show_names<'id, S: 'id + ?Sized + IdAsStr>(
    ids: impl 'id + IntoIterator<Item = &'id S>,
    glyph: GlyphPalette,
    map: &'id IDMap,
) -> impl 'id + Iterator<Item = DocTree> {
    ids.into_iter()
        .map(move |id| Tree::new(map.name(id)).with_glyphs(glyph))
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
