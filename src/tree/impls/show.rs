use crate::{tree::ID, util::XString};
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
    fn show_prettier(&self) -> DocTree;
}

impl Show for str {
    fn show(&self) -> DocTree {
        DocTree::new(self.into())
    }

    /// Just as [`Show::show`] does.
    fn show_prettier(&self) -> DocTree {
        self.show()
    }
}

impl Show for ID {
    fn show(&self) -> DocTree {
        self.as_str().show()
    }

    /// Just as [`Show::show`] does.
    fn show_prettier(&self) -> DocTree {
        self.show()
    }
}

pub fn show_ids_with(ids: &[ID], glyph: GlyphPalette) -> impl '_ + Iterator<Item = DocTree> {
    show_ids(ids).map(move |tree| tree.with_glyphs(glyph))
}
pub fn show_ids(ids: &[ID]) -> impl '_ + Iterator<Item = DocTree> {
    ids.iter().map(ID::show)
}
