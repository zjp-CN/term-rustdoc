use super::Parse;
use crate::{
    tree::IDMap,
    type_name::{fn_decl, fn_header, generics},
};
use rustdoc_types::{Function, ItemEnum, Visibility};
use std::fmt::Write;

impl Parse for Function {
    fn parse(&self, v: &Visibility, fname: &str) -> String {
        let Function {
            decl,
            generics: g,
            header,
            ..
        } = self;

        let mut buf = Self::buf(v);
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
    fn item(item: &ItemEnum) -> Option<&Self> {
        if let ItemEnum::Function(f) = item {
            Some(f)
        } else {
            None
        }
    }
}

pub fn fn_item(id: &str, map: &IDMap) -> String {
    if let Some(item) = map.get_item(id) {
        if let ItemEnum::Function(f) = &item.inner {
            let fname = item.name.as_deref().unwrap_or("");
            return f.parse(&item.visibility, fname);
        }
    }
    String::new()
}
