use itertools::Itertools;
use rustdoc_types::{Abi, FnDecl, Function, Header, ItemEnum, Visibility};
use std::fmt::Write;

use crate::{tree::IDMap, type_name::type_name};

fn vis(v: &Visibility, buf: &mut String) {
    match v {
        Visibility::Public => buf.push_str("pub "),
        Visibility::Default => (),
        Visibility::Crate => buf.push_str("pub(crate) "),
        Visibility::Restricted { path, .. } => write!(buf, "pub({path}) ").unwrap(),
    };
}

fn header(h: &Header, buf: &mut String) {
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

fn fndecl(f: &FnDecl, buf: &mut String) {
    buf.push('(');
    let args = f.inputs.iter().format_with(", ", |(name, ty), f| {
        let ty = type_name(ty).unwrap_or_default();
        f(&format_args!("{name}: {ty}"))
    });
    write!(buf, "{args}").unwrap();
    buf.push(')');
    if let Some(ty) = &f.output {
        buf.push_str(" -> ");
        if let Some(output) = type_name(ty) {
            buf.push_str(&output);
        }
    }
}

pub fn parse_fn(v: &Visibility, fname: &str, fn_item: &Function) -> String {
    let mut buf = String::with_capacity(128);
    vis(v, &mut buf);
    buf.push_str(fname);
    header(&fn_item.header, &mut buf);
    fndecl(&fn_item.decl, &mut buf);
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
