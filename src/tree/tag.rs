use super::DocTree;
use ratatui::style::{Color::*, Modifier, Style};
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
    Implementations,
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

// for each normal item/list element
fn fg(r: u8, g: u8, b: u8) -> Style {
    Style::default().fg(Rgb(r, g, b))
}

// for the title of a list of items
fn bfg(r: u8, g: u8, b: u8) -> Style {
    Style::default()
        .fg(Rgb(r, g, b))
        .add_modifier(Modifier::BOLD)
}

// for data structure name
fn bufg(r: u8, g: u8, b: u8) -> Style {
    Style::default()
        .fg(Rgb(r, g, b))
        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
}

impl Tag {
    pub fn style(self) -> Style {
        // fg(159, 234, 115), // #9FEA73
        // fg(Rgb(177, 84, 5)),     // #B15405
        match self {
            Tag::Module => bfg(213, 245, 85),     // #D5F555
            Tag::Structs => bfg(60, 148, 165),    // #3C94A5
            Tag::Struct => bufg(60, 148, 165),    // #3C94A5
            Tag::Fields => bfg(137, 199, 210),    // #89C7D2
            Tag::Field => fg(137, 199, 210),      // #89C7D2
            Tag::NoFields => fg(137, 199, 210),   // #89C7D2
            Tag::Unions => bufg(183, 64, 168),    // #B740A8
            Tag::Union => fg(183, 64, 168),       // #B740A8
            Tag::Enums => bfg(61, 176, 181),      // #3DB0B5
            Tag::Enum => bufg(61, 176, 181),      // #3DB0B5
            Tag::Variants => bfg(141, 218, 187),  // #8DDABB
            Tag::Variant => fg(141, 218, 187),    // #8DDABB
            Tag::NoVariants => fg(141, 218, 187), // #8DDABB
            Tag::Traits => bfg(255, 140, 41),     // #FF8C29
            Tag::Trait => bufg(255, 140, 41),     // #FF8C29
            Tag::Functions => bfg(214, 83, 76),   // #D6534C
            Tag::Function => fg(214, 83, 76),     // #D6534C
            Tag::Constants => bfg(232, 218, 104), // #E8DA68
            Tag::Constant => fg(232, 218, 104),   // #E8DA68
            Tag::Statics => bfg(43, 43, 175),     // #2B2BAF
            Tag::Static => bfg(43, 43, 175),      // #2B2BAF
            Tag::TypeAliass => bfg(144, 99, 200), // #9063C8
            Tag::TypeAlias => fg(144, 99, 200),   // #9063C8
            Tag::Import => fg(212, 61, 141),      // #D43D8D
            Tag::MacroDecls => bfg(15, 129, 29),  // #0F811D
            Tag::MacroDecl => fg(15, 129, 29),    // #0F811D
            Tag::MacroFuncs => bfg(96, 215, 117), // #60D775
            Tag::MacroFunc => fg(96, 215, 117),   // #60D775
            Tag::MacroAttrs => bfg(159, 233, 27), // #9FE91B
            Tag::MacroAttr => fg(159, 233, 27),   // #9FE91B
            Tag::MacroDervs => bfg(98, 152, 0),   // #629800
            Tag::MacroDerv => fg(98, 152, 0),     // #629800
            Tag::Unknown | Tag::FieldsPrivate => Style::default().fg(DarkGray),
            Tag::Implementations => Style::default().fg(White),
            Tag::InherentImpls => bfg(243, 101, 134), // #F36586
            Tag::ImplInherent => fg(243, 101, 134),   // #F36586
            Tag::TraitImpls => bfg(255, 195, 144),    // #FFC390
            Tag::ImplTrait => fg(255, 195, 144),      // #FFC390
            Tag::AutoImpls => bfg(255, 140, 41),      // #FF8C29
            Tag::ImplAuto => fg(255, 140, 41),        // #FF8C29
            Tag::BlanketImpls => bfg(222, 186, 0),    // #DEBA00
            Tag::ImplBlanket => fg(222, 186, 0),      // #DEBA00
            _ => Style::default(),
        }
    }

    pub fn glyph(self) -> GlyphPalette {
        // return Default::default();
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
            // Tag::ImplInherent => icon!("[inhrt]"),
            // Tag::ImplTrait => icon!("[trait]"),
            // Tag::ImplAuto => icon!("[auto]"),
            // Tag::ImplBlanket => icon!("[blkt]"),
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
            Tag::Implementations => "Implementations",
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
            Tag::TypeAliass => "Type Alias",
            Tag::MacroDecls => "Macros - Declarative",
            Tag::MacroFuncs => "Macros - Function",
            Tag::MacroAttrs => "Macros - Attribute",
            Tag::MacroDervs => "Macros - Derive",
            _ => {
                error!(
                    "Tag `{self:?}` should reply on contexts like \
                 name/path/type instead of plain text"
                );
                return DocTree::new(
                    "A Tag shouldn't be here. Check out the log.".into(),
                    Tag::Unknown,
                );
            }
        };
        DocTree::new(text.into(), self)
    }
}
