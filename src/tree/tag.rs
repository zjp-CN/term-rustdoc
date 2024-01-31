use super::DocTree;
use ratatui::style::{Color, Style};
use termtree::GlyphPalette;

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
