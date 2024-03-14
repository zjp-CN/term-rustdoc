use super::Format;
use crate::{
    tree::IDMap,
    type_name::{generics, short},
};
use rustdoc_types::{Id, ItemEnum, Struct, StructKind, Visibility};
use std::fmt::Write;

const PRIVATE: &str = "/* private fields */";
fn private(b: bool) -> &'static str {
    if b {
        PRIVATE
    } else {
        ""
    }
}

impl Format for Struct {
    fn parse(&self, v: &Visibility, fname: &str, map: &IDMap) -> String {
        let Struct {
            kind, generics: g, ..
        } = self;

        let mut buf = super::buf(v);
        _ = write!(&mut buf, "struct {fname}");
        let (def, where_) = generics(g);
        let b = &mut buf;
        match (kind, &def, &where_) {
            (StructKind::Unit, None, None) => b.push(';'),
            (StructKind::Unit, None, Some(w)) => _ = write!(b, "\nwhere\n    {w};"),
            (StructKind::Unit, Some(d), None) => _ = write!(b, "<{d}>;"), // very unlikely
            (StructKind::Unit, Some(d), Some(w)) => _ = write!(b, "<{d}>\nwhere\n    {w};"),
            (StructKind::Tuple(t), None, None) => {
                b.push('(');
                push_tuple_fields(t, map, b);
                b.push(')');
            }
            (StructKind::Tuple(t), None, Some(w)) => {
                b.push('(');
                push_tuple_fields(t, map, b);
                _ = write!(b, ")\nwhere\n    {w};");
            }
            (StructKind::Tuple(t), Some(d), None) => {
                _ = write!(b, "<{d}>(");
                push_tuple_fields(t, map, b);
                b.push_str(");");
            }
            (StructKind::Tuple(t), Some(d), Some(w)) => {
                _ = write!(b, "<{d}>(");
                push_tuple_fields(t, map, b);
                _ = write!(b, ")\nwhere\n    {w};");
            }
            (
                StructKind::Plain {
                    fields,
                    fields_stripped,
                },
                None,
                None,
            ) => write_body(b, fields, map, *fields_stripped),
            (
                StructKind::Plain {
                    fields,
                    fields_stripped,
                },
                None,
                Some(w),
            ) => {
                _ = writeln!(b, "\nwhere\n    {w}");
                write_body(b, fields, map, *fields_stripped);
            }
            (
                StructKind::Plain {
                    fields,
                    fields_stripped,
                },
                Some(d),
                None,
            ) => {
                _ = writeln!(b, "<{d}>");
                write_body(b, fields, map, *fields_stripped);
            }
            (
                StructKind::Plain {
                    fields,
                    fields_stripped,
                },
                Some(d),
                Some(w),
            ) => {
                _ = writeln!(b, "<{d}>\nwhere\n    {w}");
                write_body(b, fields, map, *fields_stripped);
            }
        }
        buf
    }
}

fn write_body(b: &mut String, fields: &[Id], map: &IDMap, fields_stripped: bool) {
    b.push_str(" {\n");
    push_fields(fields, map, b);
    let has_public_fields = !fields.is_empty();
    let pr = private(fields_stripped);
    if has_public_fields {
        b.push_str("    ");
    }
    b.push_str(pr);
    if has_public_fields {
        b.push('\n');
    }
    b.push('}');
}

fn push_fields(ids: &[Id], map: &IDMap, buf: &mut String) {
    fn field(id: &str, map: &IDMap, buf: &mut String) {
        if let Some(fi) = map.get_item(id) {
            if let Some(name) = fi.name.as_deref() {
                if let ItemEnum::StructField(ty) = &fi.inner {
                    _ = writeln!(buf, "    {name}: {},", short(ty));
                }
            }
        }
    }

    for id in ids.iter().map(|id| &*id.0) {
        field(id, map, buf);
    }
}

fn push_tuple_fields(ids: &[Option<Id>], map: &IDMap, buf: &mut String) {
    fn field(id: &str, map: &IDMap, buf: &mut String) {
        if let Some(fi) = map.get_item(id) {
            if let ItemEnum::StructField(ty) = &fi.inner {
                _ = write!(buf, "\n    {},", short(ty));
            }
        }
    }

    let mut contains_private = false;
    for id in ids {
        match id {
            Some(id) => field(&id.0, map, buf),
            None => contains_private = true,
        }
    }
    if contains_private {
        buf.push_str("\n    ");
        buf.push_str(PRIVATE);
    }
    if !ids.is_empty() {
        buf.push('\n');
    }
}
