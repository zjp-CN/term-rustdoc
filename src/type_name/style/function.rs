use super::{path::*, utils::write_comma, Punctuation, StyledType, Syntax, Tag};
use rustdoc_types::{Abi, FnDecl, FunctionPointer, Header, Type};

impl Format for FunctionPointer {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        let FunctionPointer {
            decl,
            generic_params, // HRTB
            header,
        } = self;
        header.format::<Kind>(buf);
        FnPointerDecl(decl).format::<Kind>(buf);
        if !generic_params.is_empty() {
            buf.write(Syntax::For);
            generic_params.format::<Kind>(buf);
        }
    }
}

/// Fn pointer contains a default `_` as argument name, but no need to show it.
/// Rust also allows named arguments in fn pointer, so if the name is not `_`, it's shown.
struct FnPointerDecl<'d>(&'d FnDecl);

impl Format for FnPointerDecl<'_> {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        let FnDecl {
            inputs,
            output,
            c_variadic,
        } = self.0;
        buf.write_in_parentheses(|buf| {
            buf.write_slice(
                inputs,
                |arg, buf| {
                    let (name, ty) = arg;
                    if name == "_" {
                        ty.format::<Kind>(buf);
                    } else {
                        arg.format::<Kind>(buf);
                    }
                },
                write_comma,
            );
            if *c_variadic {
                buf.write(Syntax::Variadic);
            }
        });
        if let Some(ty) = output {
            buf.write(Syntax::ReturnArrow);
            ty.format::<Kind>(buf);
        }
    }
}

impl Format for FnDecl {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        let FnDecl {
            inputs,
            output,
            c_variadic,
        } = self;
        buf.write_in_parentheses(|buf| {
            buf.write_slice(inputs, Format::format::<Kind>, write_comma);
            if *c_variadic {
                buf.write(Syntax::Variadic);
            }
        });
        if let Some(ty) = output {
            buf.write(Syntax::ReturnArrow);
            ty.format::<Kind>(buf);
        }
    }
}

impl Format for (String, Type) {
    /// Named function inputs for fn items.
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        let (name, ty) = self;
        buf.write(name);
        buf.write(Punctuation::Colon);
        ty.format::<Kind>(buf);
    }
}

impl Format for Header {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        use super::Function;
        let Header {
            const_,
            unsafe_,
            async_,
            abi,
        } = self;
        if *const_ {
            buf.write(Function::Const);
        }
        if *async_ {
            buf.write(Function::Const);
        }
        if *unsafe_ {
            buf.write(Function::Unsafe);
        }
        abi.format::<Kind>(buf);
    }
}

impl Format for Abi {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        use super::Abi as A;
        buf.write(match self {
            Abi::Rust => A::Rust,
            Abi::C { .. } => A::C,
            Abi::Cdecl { .. } => A::Cdecl,
            Abi::Stdcall { .. } => A::Stdcall,
            Abi::Fastcall { .. } => A::Fastcall,
            Abi::Aapcs { .. } => A::Aapcs,
            Abi::Win64 { .. } => A::Win64,
            Abi::SysV64 { .. } => A::SysV64,
            Abi::System { .. } => A::System,
            Abi::Other(abi) => {
                buf.write(A::Other);
                buf.write(Tag::UnusualAbi(abi.into()));
                return;
            }
        });
    }
}
