use super::generic::generic_param_def_for_slice;
use super::{short, Short};
use crate::util::XString;
use itertools::Itertools;
use rustdoc_types::{Abi, FnDecl, FunctionPointer, Header, Type};
use std::fmt::Write;
use std::format_args as f;

pub fn fn_header(h: &Header, mut buf: impl Write) {
    let Header {
        const_,
        unsafe_,
        async_,
        abi,
    } = h;
    if *const_ {
        _ = buf.write_str("const ");
    }
    if *unsafe_ {
        _ = buf.write_str("unsafe ");
    }
    if *async_ {
        _ = buf.write_str("async ");
    }
    _ = match abi {
        Abi::Rust => Ok(()),
        Abi::C { .. } => buf.write_str("extern \"C\" "),
        Abi::Cdecl { .. } => buf.write_str("extern \"cdecl\" "),
        Abi::Stdcall { .. } => buf.write_str("extern \"stdcall\" "),
        Abi::Fastcall { .. } => buf.write_str("extern \"fastcall\" "),
        Abi::Aapcs { .. } => buf.write_str("extern \"aapcs\" "),
        Abi::Win64 { .. } => buf.write_str("extern \"win64\" "),
        Abi::SysV64 { .. } => buf.write_str("extern \"sysv64\" "),
        Abi::System { .. } => buf.write_str("extern \"system\" "),
        Abi::Other(s) => buf.write_str(s),
    };
}

pub fn fn_decl(
    FnDecl {
        inputs,
        output,
        c_variadic,
    }: &FnDecl,
    mut buf: impl Write,
) {
    _ = buf.write_char('(');
    let multiline = (inputs.len() + *c_variadic as usize) > 2;
    if multiline {
        _ = buf.write_char('\n');
    }
    let ident = if multiline { "    " } else { "" };
    let sep = if multiline { ",\n" } else { ", " };
    let dot = (*c_variadic).then(|| (String::from("_"), Type::Primitive(String::from("..."))));
    let iter = inputs.iter().chain(dot.as_ref()).enumerate();
    let args = iter.format_with(sep, |(idx, (name, ty)), f| {
        if idx == 0 && name == "self" {
            match ty {
                Type::BorrowedRef {
                    lifetime,
                    mutable,
                    type_,
                } => {
                    let mut ty = short(type_);
                    if ty == "Self" {
                        ty.clear();
                        _ = ty.write_str("self");
                    }
                    return match (lifetime, mutable) {
                        (None, false) => f(&f!("{ident}&{ty}")),
                        (None, true) => f(&f!("{ident}&mut {ty}")),
                        (Some(life), false) => f(&f!("{ident}&{life} {ty}")),
                        (Some(life), true) => f(&f!("{ident}&{life} mut {ty}")),
                    };
                }
                Type::Generic(s) if s == "Self" => return f(&f!("{ident}self")),
                _ => (),
            }
        }
        let ty = short(ty);
        f(&f!("{ident}{name}: {ty}"))
    });
    write!(buf, "{args}").unwrap();
    if multiline {
        _ = buf.write_char('\n');
    }
    _ = buf.write_char(')');
    if let Some(ty) = output {
        _ = buf.write_str(" -> ");
        _ = buf.write_str(&short(ty));
    }
}

pub fn fn_pointer(
    FunctionPointer {
        decl,
        generic_params,
        header,
    }: &FunctionPointer,
) -> XString {
    let mut buf = XString::new_inline("");
    fn_header(header, &mut buf);
    if let Some(param) = generic_param_def_for_slice::<Short>(generic_params) {
        buf.push_str(&param);
        buf.push(' ');
    }
    buf.push_str("fn");
    fn_decl_for_fn_pointer(decl, &mut buf);
    buf
}

fn fn_decl_for_fn_pointer(f: &FnDecl, mut buf: impl Write) {
    _ = buf.write_char('(');
    let iter = f.inputs.iter().map(|input| &input.1);
    let args = iter.format_with(", ", |ty, f| {
        let ty = short(ty);
        f(&f!("{ty}"))
    });
    write!(buf, "{args}").unwrap();
    _ = buf.write_char(')');
    if let Some(ty) = &f.output {
        _ = buf.write_str(" -> ");
        _ = buf.write_str(&short(ty));
    }
}
