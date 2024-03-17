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
}

/// Rendering tag which represents color, style and metadata that are used to jump.
pub enum Tag {
    /// A path to an item that is usually carries an ID.
    /// We use the ID to jump to another item.
    /// The Path do not include generics.
    /// A Path ID tag is conjuction with with its Name tag.
    Path(ID),
    /// A Name is a short path an ID can refer to or somthing not statically known.
    /// E.g. short path/type name, function argument, name for field, variant and generics etc.
    Name(XString),
    Symbol(Symbol),
    Decl(Decl),
    PubScope(ID),
    UnusualAbi(XString),
}

pub enum Symbol {
    Syntax(Syntax),
    Punctuation(Punctuation),
}

/// Implement to_str and str_len methods and basic Derive macros for a fieldless enum.
macro_rules! to_str {
    (
        $(#[$m:meta])*
        $vis:vis enum $ename:ident { $(
            $(#[$fm:meta])*
            $variant:ident = $s:literal,
        )+ }
    ) => {
        $(#[$m])* #[derive(Clone, Copy, Debug)]
        $vis enum $ename { $(
            $(#[$fm])* $variant,
        )+ }
        impl $ename {
            pub const fn to_str(self) -> &'static str {
                match self {
                    $($ename::$variant => $s ,)+
                }
            }
            pub const fn str_len(self) -> usize {
                match self {
                    $($ename::$variant => $s.len() ,)+
                }
            }
        }
    };
    // It's tricky to intersperse fieldless variants with field-carrying ones,
    // thus use `@` to separate them.
    (
        $(#[$m:meta])*
        $vis:vis enum $ename:ident { $(
            $(#[$fm:meta])*
            $variant:ident = $s:literal,
        )* @ $(
            $(#[$vm:meta])*
            $var:ident($inner:ident),
        )* }
    ) => {
        $(#[$m])* #[derive(Clone, Copy, Debug)]
        $vis enum $ename { $(
            $(#[$fm])* $variant,
        )* $(
            $(#[$vm])* $var($inner),
        )*}
        impl $ename {
            pub const fn to_str(self) -> &'static str {
                match self {
                    $($ename::$variant => $s ,)*
                    $($ename::$var(val) => val.to_str() ,)*
                }
            }
            pub const fn str_len(self) -> usize {
                match self {
                    $($ename::$variant => $s.len() ,)*
                    $($ename::$var(val) => val.str_len() ,)*
                }
            }
        }
    };
}

to_str!(
    /// Symbol as syntax component.
    ///
    /// NOTE: some syntax has already included whitespaces, because this saves pushing them.
    pub enum Syntax {
        /// `&`
        Reference = "&",
        /// `&mut`
        ReferenceMut = "&mut",
        /// `mut`: lifetime may lie between `&` and `mut`
        Mut = "mut",
        /// `Self`
        Self_ = "Self",
        /// `dyn `
        Dyn = "dyn ",
        /// `::`
        PathSep = "::",
        /// ` as `
        As = " as ",
        /// `*const `
        RawPointer = "*const ",
        /// `*mut `
        RawPointerMut = "*mut ",
        /// `_`
        Infer = "_",
        /// `impl `
        Impl = "impl ",
        /// `for`
        For = "for",
        /// `?` (mainly ?Sized)
        Maybe = "?",
        /// `~const `
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
        /// ` = `
        Equal = " = ",
        /// ` + `
        Plus = " + ",
        Tick = "'",
        AngleBracketStart = "<",
        AngleBracketEnd = ">",
        SquareBracketStart = "[",
        SquareBracketEnd = "]",
        ParenthesisStart = "(",
        ParenthesisEnd = ")",
        BraceStart = "{",
        BraceEnd = "}",
    }
);

to_str!(
    /// Components in declaration. A type doesn't need this, but type declaration need this.
    pub enum Decl { @
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
        /// Placeholder, not showing anything. But in conjuction with [`Tag::PubScope`].
        PubScope = "",
    }
);

to_str!(
    /// FunctionQualifiers order: const? async? unsafe? (extern Abi?)? fn
    pub enum Function {
        Const = "const ",
        Async = "async ",
        Unsafe = "unsafe ",
        Fn = "fn ",
        @
        Abi(Abi),
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
        /// Placeholder, not showing anything. But in conjuction with [`Tag::UnusualAbi`].
        Other = "",
    }
);

to_str!(
    pub enum Struct {
        Struct = "struct ",
    }
);
