use rustdoc_types::Id;
use term_rustdoc::util::XString;

/// Extra meaning not so relevant to style in current word.
#[derive(Default, Clone, Debug)]
pub enum MetaTag {
    #[default]
    Normal,
    Link(LinkTag),
    InlineCode,
    // InlineHTML,
    ListItem,
    ListItemN(u8),

    // separate block elements: directly rendered as a truncatable line
    Heading(usize),
    Image,
    Rule,
    FootnoteSource,

    CodeBlock(XString),
    QuoteBlock,
}

/// metadata/extra info in a chunk of text
///
/// StyledText { text: String, style: Style, meta: Option<TextMeta>, pos: (u16, u16) }
/// Heading(u8)
/// Line { inner: Vec<StyledText>, meta: Heading }
/// Paragraph { inner: Vec<Line>, externallinks: Vec<usize>,
///             hyperlinks: Vec<usize>, footnotes: Vec<usize> }
///
/// Links { inner: Vec<XString> }
/// the usize is used only in the doc to refer to the link position in vec to highlight it
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum LinkTag {
    /// local crate item can be referred by item ID
    LocalItemLink(Id),
    /// points to a external crate item path (may be supported once multi-crate docs are ready)
    ExternalItemLink(usize),
    /// Reference link
    ReferenceLink(usize),
    /// a link to styled text
    Footnote(XString),
    // /// Autolink or Email, both of which are in the form of `<xxx>`
    // ///
    // /// the URL content will be rendered directly and won't be cached in vec
    // Url(XString),
    /// broken link or invalid input tag
    Unknown,
}
