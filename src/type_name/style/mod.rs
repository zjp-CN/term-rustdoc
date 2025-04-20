#![allow(unused)]
mod decl;
mod function;
mod generics;
mod path;
mod type_;
mod utils;

use super::render::DeclarationLines;
use crate::util::XString;
use std::fmt;

pub use path::{long, long_path};
use rustdoc_types::Id;

#[derive(Default, Clone, Debug)]
pub struct StyledType {
    inner: Vec<Tag>,
}

impl fmt::Display for StyledType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for tag in &self.inner {
            _ = tag.fmt(f);
        }
        Ok(())
    }
}

impl StyledType {
    fn with_capacity(cap: usize) -> Self {
        StyledType {
            inner: Vec::with_capacity(cap),
        }
    }

    pub(crate) fn tags(&self) -> &[Tag] {
        &self.inner
    }

    fn write<T: Into<Tag>>(&mut self, tag: T) {
        self.inner.push(tag.into());
    }

    pub fn str_len(&self) -> usize {
        self.inner.iter().map(Tag::str_len).sum()
    }

    pub fn to_non_wrapped_string(&self) -> String {
        use fmt::Write;
        let mut buf = String::with_capacity(self.str_len());
        _ = write!(buf, "{self}");
        buf
    }

    fn write_id_name(&mut self, id: Id, name: &str) {
        self.write_span_path_name(|buf| {
            buf.write(Tag::Path(id));
            buf.write(Tag::Name(name.into()));
        });
    }

    fn write_vis_scope(&mut self, id: Id, path: &str) {
        self.write(Tag::Decl(Decl::Vis(Vis::PubScope)));
        self.write_in_parentheses(|s| {
            s.write(Tag::PubScope(id));
            s.write(Tag::Name(path.into()));
        });
    }

    /// Write `start_tag` `x` `end_tag` where x is written from the callback.
    /// (start_tag, end_tag) can be `()` `[]` `{}` `<>` [`Span`] etc.
    fn write_enclosing_tag(
        &mut self,
        start_tag: Tag,
        end_tag: Tag,
        f: impl FnOnce(&mut StyledType),
    ) {
        self.write(start_tag);
        f(self);
        self.write(end_tag);
    }
}

macro_rules! impl_write_tag {
    (@Punctuation $($fname:ident $s:literal: $start:ident, $end:ident),+ $(,)?) => {
        /// Write stuff in enclosing [`Punctuation`]s.
        impl StyledType { $(
            #[doc = concat!("Write `", $s, "` where x is written from the callback.")]
            fn $fname(&mut self, f: impl FnOnce(&mut StyledType)) {
                let start = Tag::Symbol(Symbol::Punctuation(Punctuation::$start));
                let end = Tag::Symbol(Symbol::Punctuation(Punctuation::$end));
                self.write_enclosing_tag(start, end, f);
            }
        )+ }
    };
    (@span $($fname:ident $s:literal $span:ident),+ $(,)?) => {
        /// Write stuff in enclosing [`Span`]s.
        impl StyledType { $(
            #[doc = concat!("Write `", $s, "` as [`Span::", stringify!($span),
              "`] between [`Tag::Start`] and [`Tag::End`] with the callback.")]
            fn $fname(&mut self, f: impl FnOnce(&mut StyledType)) {
                let start = Tag::Start(Span::$span);
                let end = Tag::End(Span::$span);
                self.write_enclosing_tag(start, end, f);
            }
        )+ }
    };
}

impl_write_tag!(@Punctuation
    write_in_brace "{x}": BraceStart, BraceEnd ,
    write_in_parentheses "(x)": ParenthesisStart, ParenthesisEnd ,
    write_in_angle_bracket "<x>": AngleBracketStart, AngleBracketEnd ,
    write_in_squre_bracket "[x]": SquareBracketStart, SquareBracketEnd ,
);

impl_write_tag!(@span
    write_span_path_name "path_without_generics" PathName ,
    write_span_type_name "..." TypeName ,
    write_span_where_bound "where ..." WhereBound ,
    write_span_generics_def "< ... >" GenericsDef ,
    write_span_function_name "fn ..." FunctionName ,
);

/// Rendering tag which represents color, style and metadata that are used to jump.
#[derive(Clone, Debug)]
pub enum Tag {
    /// A path to an item that is usually carries an ID.
    /// We use the ID to jump to another item.
    /// The Path do not include generics.
    /// A Path ID tag is conjuction with its Name tag.
    Path(Id),
    /// A Name is a short path an ID can refer to or somthing not statically known.
    /// E.g. short path/type name, function argument, name for field, variant and generics etc.
    Name(XString),
    Symbol(Symbol),
    Decl(Decl),
    /// The scope id is in conjuction with a Name.
    /// `pub(scope)` is composed of [`Vis::PubScope`], [`Punctuation::ParenthesisStart`],
    /// [`Tag::Name`] and [`Punctuation::ParenthesisEnd`].
    PubScope(Id),
    /// In conjuction with [`Abi`].
    /// `extern "other_abi" ` is composed of [`Abi::Other`], [`Punctuation::Quote`],
    /// [`Tag::UnusualAbi`], [`Punctuation::Quote`] and [`Punctuation::WhiteSpace`].
    UnusualAbi(XString),
    Start(Span),
    End(Span),
}

impl From<&str> for Tag {
    fn from(val: &str) -> Self {
        Tag::Name(val.into())
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tag::Path(_) => (),
            Tag::Name(s) => _ = f.write_str(s),
            Tag::Symbol(s) => _ = f.write_str(s.to_str()),
            Tag::Decl(s) => _ = f.write_str(s.to_str()),
            Tag::PubScope(_) => (),
            Tag::UnusualAbi(s) => _ = f.write_str(s),
            Tag::Start(_) | Tag::End(_) => (),
        }
        Ok(())
    }
}

impl Tag {
    pub fn str_len(&self) -> usize {
        match self {
            // a path uses name len instead of id len
            Tag::Path(_) => 0,
            Tag::Name(s) => s.len(),
            Tag::Symbol(s) => s.str_len(),
            Tag::Decl(s) => s.str_len(),
            // `pub(scope)`: scope use name len rather than id len, but we should count `pub()` here
            Tag::PubScope(_) => 0,
            Tag::UnusualAbi(s) => s.len(),
            Tag::Start(_) | Tag::End(_) => 0,
        }
    }
}

// macro MWE for interspersing fieldless variants with field-carrying ones:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=33ef54aa78ae2ac682c911440f8d576a

/// Implement to_str and str_len methods and basic Derive macros for a fieldless enum.
macro_rules! to_str {
    ($({$val:ident $from:expr})?
        $(#[$em:meta])*
        $vis:vis enum $e:ident { $($t:tt)+ }
    ) => {
        to_str!(@impl [def {$(#[$em])* $vis} $e {}] [to_str {}] [str_len {}] : $($t)+);
        $(impl From<$e> for Tag {
            fn from($val: $e) -> Tag { $from }
        })?
    };
    // expand token trees
    (@impl
     [def {$(#[$em:meta])* $vis:vis} $e:ident { $($vars:tt)* }]
     [to_str  { $($b1:tt)* }]
     [str_len { $($b2:tt)* }] :
    ) => {
        #[derive(Clone, Copy, Debug)] $(#[$em])*
        $vis enum $e { $($vars)* }
        impl $e {
            pub const fn to_str(self) -> &'static str {
                match self { $($b1)* }
            }
            pub const fn str_len(self) -> usize {
                match self { $($b2)* }
            }
        }
    };
    (@impl
     [def {$(#[$em:meta])* $vis:vis} $e:ident { $($vars:tt)* }]
     [to_str  { $($b1:tt)* }]
     [str_len { $($b2:tt)* }] :
     $(#[$vm:meta])* $var:ident = $s:literal ,
    ) => {
        to_str!(@impl
            [def {$(#[$em])* $vis} $e { $($vars)* $(#[$vm])* $var , } ]
            [to_str  { $($b1)* $e::$var => $s,       } ]
            [str_len { $($b2)* $e::$var => $s.len(), } ] :
        );
    };
    (@impl
     [def {$(#[$em:meta])* $vis:vis} $e:ident { $($vars:tt)* }]
     [to_str  { $($b1:tt)* }]
     [str_len { $($b2:tt)* }] :
     $(#[$vm:meta])* $var:ident = $s:literal , $($t:tt)+
    ) => {
        to_str!(@impl
            [def {$(#[$em])* $vis} $e { $($vars)* $(#[$vm])* $var , } ]
            [to_str  { $($b1)* $e::$var => $s,       } ]
            [str_len { $($b2)* $e::$var => $s.len(), } ] :
            $($t)+
        );
    };
    (@impl
     [def {$(#[$em:meta])* $vis:vis} $e:ident { $($vars:tt)* }]
     [to_str  { $($b1:tt)* }]
     [str_len { $($b2:tt)* }] :
     $(#[$vm:meta])* $var:ident($inner:ident) ,
    ) => {
        to_str!(@impl
            [def {$(#[$em])* $vis} $e { $($vars)* $(#[$vm])* $var($inner) , } ]
            [to_str  { $($b1)* $e::$var(val) => val.to_str(),  } ]
            [str_len { $($b2)* $e::$var(val) => val.str_len(), } ] :
        );
    };
    (@impl
     [def {$(#[$em:meta])* $vis:vis} $e:ident { $($vars:tt)* }]
     [to_str  { $($b1:tt)* }]
     [str_len { $($b2:tt)* }] :
     $(#[$vm:meta])* $var:ident($inner:ident) , $($t:tt)+
    ) => {
        to_str!(@impl
            [def {$(#[$em])* $vis} $e { $($vars)* $(#[$vm])* $var($inner) , } ]
            [to_str  { $($b1)* $e::$var(val) => val.to_str(),  } ]
            [str_len { $($b2)* $e::$var(val) => val.str_len(), } ] :
            $($t)+
        );
    };
}

to_str!({val Tag::Symbol(val)}
    pub enum Symbol {
        Syntax(Syntax),
        Punctuation(Punctuation),
    }
);

to_str!({val Tag::Symbol(Symbol::Syntax(val))}
    /// Symbol as syntax component.
    ///
    /// NOTE: some syntax has already included whitespaces, because this saves pushing them.
    #[allow(non_camel_case_types)]
    pub enum Syntax {
        Reference = "&",
        ReferenceMut = "&mut ",
        /// lifetime may lie between `&` and `mut`
        Mut = "mut ",
        ReturnArrow = " -> ",
        self_ = "self",
        Self_ = "Self",
        Where = "where",
        Dyn = "dyn ",
        PathSep = "::",
        As = " as ",
        RawPointer = "*const ",
        RawPointerMut = "*mut ",
        Infer = "_",
        Impl = "impl ",
        For = "for",
        Const = "const ",
        /// mainly for `?Sized`
        Maybe = "?",
        MaybeConst = "~const",
        FnPointer = "fn",
        Variadic = "...",
    }
);

to_str!({val Tag::Symbol(Symbol::Punctuation(val))}
    /// Punctuation symbol.
    ///
    /// NOTE: some Punctuations have included whitespaces for convenience.
    pub enum Punctuation {
        WhiteSpace = " ",
        NewLine = "\n",
        Indent = "    ",
        Comma = ",",
        /// `: `
        Colon = ": ",
        /// <code> = </code>
        Equal = " = ",
        /// <code> + </code>
        Plus = " + ",
        Apostrophe = "'",
        SemiColon = ";",
        AngleBracketStart = "<",
        AngleBracketEnd = ">",
        SquareBracketStart = "[",
        SquareBracketEnd = "]",
        ParenthesisStart = "(",
        ParenthesisEnd = ")",
        BraceStart = "{",
        BraceEnd = "}",
        Quote = "\"",
    }
);

to_str!({val Tag::Decl(val)}
    /// Components in declaration. A type doesn't need this, but type declaration need this.
    pub enum Decl {
        Vis(Vis),
        Function(Function),
        Struct = "struct ",
        PrivateFields = "/* private fields */",
    }
);

to_str!({val Tag::Decl(Decl::Vis(val))}
    pub enum Vis {
        Pub = "pub ",
        /// placeholder: not showing anything
        Default = "",
        PubCrate = "pub(crate) ",
        /// In conjuction with [`Tag::PubScope`].
        PubScope = "pub",
    }
);

to_str!({val Tag::Decl(Decl::Function(val))}
    /// FunctionQualifiers order: const? async? unsafe? (extern Abi?)? fn
    pub enum Function {
        Const = "const ",
        Async = "async ",
        Unsafe = "unsafe ",
        Abi(Abi),
        Fn = "fn ",
    }
);

to_str!({val Tag::Decl(Decl::Function(Function::Abi(val)))}
    pub enum Abi {
        /// `extern "Rust"` is valid though, but for simplicity, no need to show it.
        Rust = "",
        C = "extern \"C\" ",
        Cdecl = "extern \"cdecl\" ",
        Stdcall = "extern \"stdcall\" ",
        Fastcall = "extern \"fastcall\" ",
        Aapcs = "extern \"aapcs\" ",
        Win64 = "extern \"win64\" ",
        SysV64 = "extern \"sysv64\" ",
        System = "extern \"system\" ",
        /// In conjuction with [`Tag::UnusualAbi`].
        Other = "extern ",
    }
);

// to_str!({val Tag::Decl(Decl::Struct(val))}
//     pub enum Struct {
//         Struct = "struct ",
//     }
// );

to_str!(
    /// Used to recognize the enclosing span for a component that needs styles.
    /// We want generics definitions, where bounds, names for structs/enums/fns,
    /// types for fields/enums/fn arguments etc to be colored.
    ///
    /// E.g. `Start(Function) - Path(ID) - Name(string) - End(Function)`
    ///
    /// All variants are zero str_len, and won't display anything.
    /// In conjuction with [`Tag::Start`] and [`Tag::End`].
    pub enum Span {
        PathName = "",
        TypeName = "",
        GenericsDef = "",
        WhereBound = "",
        FunctionName = "",
        FunctionArg = "",
        StructName = "",
        Field = "",
        EnumName = "",
        Variant = "",
    }
);
