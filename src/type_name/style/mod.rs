#![allow(unused)]
mod path;

use crate::{tree::ID, util::XString};

trait Format {
    fn format(&self, buf: &mut StyledType);
}

pub struct StyledType {
    inner: Vec<Tag>,
}

impl StyledType {
    fn write(&mut self, tag: Tag) {
        self.inner.push(tag);
    }

    fn write_format(&mut self, fmt: impl Format) {
        fmt.format(self);
    }

    #[allow(clippy::inherent_to_string)]
    fn to_string(&self) -> String {
        let cap = self.inner.iter().map(Tag::str_len).sum::<usize>();
        let mut buf = String::with_capacity(cap);
        for tag in &self.inner {
            tag.write_to_string(&mut buf)
        }
        buf
    }
}

/// Rendering tag which represents color, style and metadata that are used to jump.
#[derive(Clone, Debug)]
pub enum Tag {
    /// A path to an item that is usually carries an ID.
    /// We use the ID to jump to another item.
    /// The Path do not include generics.
    /// A Path ID tag is conjuction with its Name tag.
    Path(ID),
    /// A Name is a short path an ID can refer to or somthing not statically known.
    /// E.g. short path/type name, function argument, name for field, variant and generics etc.
    Name(XString),
    Symbol(Symbol),
    Decl(Decl),
    /// The scope id is in conjuction with a Name.
    /// `pub(scope)` is composed of [`Vis::PubScope`], [`Punctuation::ParenthesisStart`],
    /// [`Tag::Name`] and [`Punctuation::ParenthesisEnd`].
    PubScope(ID),
    /// In conjuction with [`Abi`].
    /// `extern "other_abi" ` is composed of [`Abi::Other`], [`Punctuation::Quote`],
    /// [`Tag::UnusualAbi`], [`Punctuation::Quote`] and [`Punctuation::WhiteSpace`].
    UnusualAbi(XString),
}

impl Tag {
    pub fn write_to_string(&self, buf: &mut String) {
        match self {
            Tag::Path(_) => (),
            Tag::Name(s) => buf.push_str(s),
            Tag::Symbol(s) => buf.push_str(s.to_str()),
            Tag::Decl(s) => buf.push_str(s.to_str()),
            Tag::PubScope(_) => (),
            Tag::UnusualAbi(s) => buf.push_str(s),
        }
    }
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
        }
    }
}

// macro MWE for interspersing fieldless variants with field-carrying ones:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=33ef54aa78ae2ac682c911440f8d576a

/// Implement to_str and str_len methods and basic Derive macros for a fieldless enum.
macro_rules! to_str {
    (
        $(#[$em:meta])*
        $vis:vis enum $e:ident { $($t:tt)+ }
    ) => {
        to_str!(@impl [def {$(#[$em])* $vis} $e {}] [to_str {}] [str_len {}] : $($t)+);
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

to_str!(
    pub enum Symbol {
        Syntax(Syntax),
        Punctuation(Punctuation),
    }
);

to_str!(
    /// Symbol as syntax component.
    ///
    /// NOTE: some syntax has already included whitespaces, because this saves pushing them.
    pub enum Syntax {
        Reference = "&",
        ReferenceMut = "&mut",
        /// lifetime may lie between `&` and `mut`
        Mut = "mut",
        Self_ = "Self",
        Where = "where ",
        Dyn = "dyn ",
        PathSep = "::",
        As = " as ",
        RawPointer = "*const ",
        RawPointerMut = "*mut ",
        Infer = "_",
        Impl = "impl ",
        For = "for",
        /// mainly for `?Sized`
        Maybe = "?",
        MaybeConst = "~const",
    }
);

to_str!(
    /// Punctuation symbol.
    ///
    /// NOTE: some Punctuations have included whitespaces for convenience.
    pub enum Punctuation {
        WhiteSpace = " ",
        NewLine = "\n",
        /// `, `
        Comma = ", ",
        /// `: `
        Colon = ": ",
        /// <code> = </code>
        Equal = " = ",
        /// <code> + </code>
        Plus = " + ",
        Apostrophe = "'",
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

to_str!(
    /// Components in declaration. A type doesn't need this, but type declaration need this.
    pub enum Decl {
        Vis(Vis),
        Function(Function),
        Struct(Struct),
    }
);

to_str!(
    pub enum Vis {
        Pub = "pub ",
        /// placeholder: not showing anything
        Default = "",
        PubCrate = "pub(crate) ",
        /// In conjuction with [`Tag::PubScope`].
        PubScope = "pub",
    }
);

to_str!(
    /// FunctionQualifiers order: const? async? unsafe? (extern Abi?)? fn
    pub enum Function {
        Const = "const ",
        Async = "async ",
        Unsafe = "unsafe ",
        Abi(Abi),
        Fn = "fn ",
    }
);

to_str!(
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

to_str!(
    pub enum Struct {
        Struct = "struct ",
    }
);
