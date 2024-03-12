use crate::type_name::generics;
use crate::{tree::IDMap, type_name::short};
use itertools::Itertools;
use rustdoc_types::{Abi, FnDecl, Function, Header, ItemEnum, Type, Visibility};
use std::fmt::Write;
use std::format_args as f;

fn vis(v: &Visibility, buf: &mut String) {
    match v {
        Visibility::Public => buf.push_str("pub "),
        Visibility::Default => (),
        Visibility::Crate => buf.push_str("pub(crate) "),
        Visibility::Restricted { path, .. } => write!(buf, "pub({path}) ").unwrap(),
    };
}

fn fn_header(h: &Header, buf: &mut String) {
    let Header {
        const_,
        unsafe_,
        async_,
        abi,
    } = h;
    if *const_ {
        buf.push_str("const ");
    }
    if *unsafe_ {
        buf.push_str("unsafe ");
    }
    if *async_ {
        buf.push_str("async ");
    }
    match abi {
        Abi::Rust => (),
        Abi::C { .. } => buf.push_str("extern \"C\""),
        Abi::Cdecl { .. } => buf.push_str("extern \"cdecl\""),
        Abi::Stdcall { .. } => buf.push_str("extern \"stdcall\""),
        Abi::Fastcall { .. } => buf.push_str("extern \"fastcall\""),
        Abi::Aapcs { .. } => buf.push_str("extern \"aapcs\""),
        Abi::Win64 { .. } => buf.push_str("extern \"win64\""),
        Abi::SysV64 { .. } => buf.push_str("extern \"sysv64\""),
        Abi::System { .. } => buf.push_str("extern \"system\""),
        Abi::Other(s) => buf.push_str(s),
    };
}

fn fn_decl(f: &FnDecl, buf: &mut String) {
    buf.push('(');
    let iter = f.inputs.iter().enumerate();
    let args = iter.format_with(", ", |(idx, (name, ty)), f| {
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
                        ty.push_str("self");
                    }
                    return match (lifetime, mutable) {
                        (None, false) => f(&f!("&{ty}")),
                        (None, true) => f(&f!("&mut {ty}")),
                        (Some(life), false) => f(&f!("&{life} {ty}")),
                        (Some(life), true) => f(&f!("&{life} mut {ty}")),
                    };
                }
                Type::Generic(s) if s == "Self" => return f(&"self"),
                _ => (),
            }
        }
        let ty = short(ty);
        f(&f!("{name}: {ty}"))
    });
    write!(buf, "{args}").unwrap();
    buf.push(')');
    if let Some(ty) = &f.output {
        buf.push_str(" -> ");
        buf.push_str(&short(ty));
    }
}

fn parse_fn(
    v: &Visibility,
    fname: &str,
    Function {
        decl,
        generics: g,
        header,
        ..
    }: &Function,
) -> String {
    let mut buf = String::with_capacity(128);
    vis(v, &mut buf);
    buf.push_str(fname);
    let (def, where_) = generics(g);
    if !def.is_empty() {
        write!(buf, "<{def}>").unwrap();
    }
    fn_header(header, &mut buf);
    fn_decl(decl, &mut buf);
    if !where_.is_empty() {
        write!(buf, " where {where_}").unwrap();
    }
    buf
}

pub fn fn_item(id: &str, map: &IDMap) -> String {
    if let Some(item) = map.get_item(id) {
        if let ItemEnum::Function(f) = &item.inner {
            let fname = item.name.as_deref().unwrap_or("");
            return parse_fn(&item.visibility, fname, f);
        }
    }
    String::new()
}
