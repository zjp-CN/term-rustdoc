use super::{generics::hrtb, path::*, utils::write_comma, Punctuation, StyledType, Syntax, Tag};
use rustdoc_types::{Abi, FunctionHeader, FunctionPointer, FunctionSignature, Type};

impl Format for FunctionPointer {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        let FunctionPointer {
            sig,
            generic_params, // HRTB
            header,
        } = self;
        hrtb::<Kind>(generic_params, buf);
        header.format::<Kind>(buf);
        buf.write(Syntax::FnPointer);
        FnPointerDecl(sig).format::<Kind>(buf);
    }
}

/// Fn pointer contains a default `_` as argument name, but no need to show it.
/// Rust also allows named arguments in fn pointer, so if the name is not `_`, it's shown.
/// Besides, the arguments in a fnpointer are in one line, or rather the whole fnpointer is one-line,
/// whereas lines for arguments in a function item depend.
struct FnPointerDecl<'d>(&'d FunctionSignature);

impl Format for FnPointerDecl<'_> {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        let FunctionSignature {
            inputs,
            output,
            is_c_variadic,
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
            if *is_c_variadic {
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
    ///
    /// NOTE: usually some more checks should be performed before calling this:
    /// * fn pointers don't need `_` name
    /// * fn items don't need `self` name
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        let (name, ty) = self;
        buf.write(name);
        buf.write(Punctuation::Colon);
        ty.format::<Kind>(buf);
    }
}

impl Format for FunctionHeader {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        use super::Function;
        let FunctionHeader {
            is_const,
            is_unsafe,
            is_async,
            abi,
        } = self;
        if *is_const {
            buf.write(Function::Const);
        }
        if *is_async {
            buf.write(Function::Const);
        }
        if *is_unsafe {
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
