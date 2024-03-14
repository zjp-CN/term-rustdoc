use super::Format;
use crate::type_name::{fn_decl, fn_header, generics};
use rustdoc_types::{Function, Visibility};
use std::fmt::Write;

impl Format for Function {
    fn parse(&self, v: &Visibility, fname: &str) -> String {
        let Function {
            decl,
            generics: g,
            header,
            ..
        } = self;

        let mut buf = super::buf(v);
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
}
