use crate::{
    tree::IDMap,
    type_name::{fn_decl, fn_header, generics},
};
use rustdoc_types::{Function, ItemEnum, Visibility};
use std::fmt::Write;

fn vis(v: &Visibility, buf: &mut String) {
    match v {
        Visibility::Public => buf.push_str("pub "),
        Visibility::Default => (),
        Visibility::Crate => buf.push_str("pub(crate) "),
        Visibility::Restricted { path, .. } => write!(buf, "pub({path}) ").unwrap(),
    };
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
    fn_header(header, &mut buf);
    buf.push_str("fn ");
    buf.push_str(fname);
    let (def, where_) = generics(g);
    if let Some(def) = &def {
        write!(buf, "<{def}>").unwrap();
    }
    fn_decl(decl, &mut buf);
    if let Some(where_) = &where_ {
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
