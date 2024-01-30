use crate::{
    tree::{IDMap, IdAsStr, ID},
    util::XString,
};
use ratatui::style::{Color, Style};
use std::{fmt, rc::Rc};
use termtree::{GlyphPalette, Tree};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tag {
    Module,
    Struct,
    Union,
    Enum,
    Trait,
    Function,
    Constant,
    Static,
    TypeAlias,
    Import,
    MacroDecl,
    MacroFunc,
    MacroAttr,
    MacroDerv,
    Unknown,
    NoImpls,
    ImplInherent,
    InherentImpls,
    ImplTrait,
    TraitImpls,
    ImplAuto,
    AutoImpls,
    ImplBlanket,
    BlanketImpls,
    NoVariants, // Head for no variants
    Variants,   // Head for variants
    Variant,
    NoFields,
    Fields,
    Field,
    FieldsPrivate,
    NoAssocOrImpls,
    AssocTypes,
    AssocType,
    AssocConsts,
    AssocConst,
    AssocFns,
    AssocFn,
    Implementors,
    Implementor,
    Structs,
    Unions,
    Enums,
    Traits,
    Functions,
    Constants,
    Statics,
    TypeAliass,
    MacroDecls,
    MacroFuncs,
    MacroAttrs,
    MacroDervs,
}

impl Tag {
    pub fn style(self) -> Style {
        let style = Style::default();
        match self {
            Tag::Module => style.fg(Color::LightRed),
            Tag::Struct => style.fg(Color::Green),
            Tag::Union => style.fg(Color::LightCyan),
            Tag::Enum => style.fg(Color::Cyan),
            Tag::Trait => style.fg(Color::Magenta),
            Tag::Function => style.fg(Color::Rgb(60, 194, 200)),
            Tag::Constant => style.fg(Color::Rgb(255, 138, 37)),
            Tag::Static => style.fg(Color::Rgb(147, 72, 209)),
            Tag::TypeAlias => style.fg(Color::Rgb(36, 14, 156)),
            Tag::Import => style.fg(Color::Rgb(37, 143, 195)),
            Tag::MacroDecl => style.fg(Color::Rgb(0, 180, 0)),
            Tag::MacroFunc => style.fg(Color::Rgb(67, 227, 67)),
            Tag::MacroAttr => style.fg(Color::Rgb(106, 233, 106)),
            Tag::MacroDerv => style.fg(Color::Rgb(33, 222, 33)),
            Tag::Unknown => style.fg(Color::DarkGray),
            Tag::InherentImpls => style,
            Tag::TraitImpls => style,
            Tag::AutoImpls => style,
            Tag::BlanketImpls => style,
            _ => style,
        }
    }

    pub fn glyph(self) -> GlyphPalette {
        match self {
            Tag::Module => icon!("[Mod]"),
            Tag::Struct => icon!("[Struct]"),
            Tag::Union => icon!("[Union]"),
            Tag::Enum => icon!("[Enum]"),
            Tag::Trait => icon!("[Trait]"),
            Tag::Function => icon!("[Fn]"),
            Tag::Constant => icon!("[Const]"),
            Tag::Static => icon!("[Static]"),
            Tag::TypeAlias => icon!("[type alias]"),
            Tag::MacroDecl => icon!("[macro decl]"),
            Tag::MacroFunc => icon!("[macro func]"),
            Tag::MacroAttr => icon!("[macro attr]"),
            Tag::MacroDerv => icon!("[macro derv]"),
            Tag::ImplInherent => icon!("[inhrt]"),
            Tag::ImplTrait => icon!("[trait]"),
            Tag::ImplAuto => icon!("[auto]"),
            Tag::ImplBlanket => icon!("[blkt]"),
            Tag::Field => icon!("[field]"),
            Tag::Variant => icon!("[variant]"),
            Tag::AssocType => icon!("[assoc type]"),
            Tag::AssocConst => icon!("[assoc constant]"),
            Tag::AssocFn => icon!("[assoc fn]"),
            _ => GlyphPalette::default(),
        }
    }

    /// Show as a simple heading node with no need for contexts.
    pub fn show(self) -> DocTree {
        let text = match self {
            Tag::NoImpls => "No Implementations!",
            Tag::InherentImpls => "Inherent Impls",
            Tag::TraitImpls => "Trait Impls",
            Tag::AutoImpls => "Auto Impls",
            Tag::BlanketImpls => "Blanket Impls",
            Tag::NoVariants => "No Variants!",
            Tag::Variants => "Variants",
            Tag::NoFields => "No Fields!",
            Tag::Fields => "Fields",
            Tag::FieldsPrivate => "/* private fields */",
            Tag::NoAssocOrImpls => "No Associated Items Or Implementors!",
            Tag::AssocTypes => "Associated Types",
            Tag::AssocConsts => "Associated Constants",
            Tag::AssocFns => "Associated Functions",
            Tag::Implementors => "Implementors",
            Tag::Structs => "Structs",
            Tag::Unions => "Unions",
            Tag::Enums => "Enums",
            Tag::Traits => "Traits",
            Tag::Functions => "Functions",
            Tag::Constants => "Constants",
            Tag::Statics => "Statics",
            Tag::TypeAliass => "Statics",
            Tag::MacroDecls => "Macros - Declarative",
            Tag::MacroFuncs => "Macros - Function",
            Tag::MacroAttrs => "Macros - Attribute",
            Tag::MacroDervs => "Macros - Derive",
            // _ => format!("error for Tag `{self:?}`").leak(),
            _ => unimplemented!(
                "Tag `{self:?}` should reply on contexts like \
                 name/path/type instead of plain text"
            ),
        };
        DocTree::new(text.into(), self)
    }
}

/// Tagged text including headings and nodes.
///
/// This type is displayed as empty because it's used
/// as an empty node in Tree display to get the tree stripe only.
pub struct TextTag {
    text: XString,
    tag: Tag,
}

impl fmt::Display for TextTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
