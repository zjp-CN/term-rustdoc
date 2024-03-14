use super::Format;
use crate::type_name::generics;
use rustdoc_types::{ItemEnum, Struct, StructKind, Visibility};
use std::fmt::Write;

fn private(b: bool) -> &'static str {
    const PRIVATE: &str = "/* private fields */";
    if b {
        PRIVATE
    } else {
        ""
    }
}

impl Format for Struct {
    fn parse(&self, v: &Visibility, fname: &str) -> String {
        let Struct {
            kind, generics: g, ..
        } = self;

        let mut buf = super::buf(v);
        _ = write!(&mut buf, "struct {fname}");
        let (def, where_) = generics(g);
        let b = &mut buf;
        match (kind, &def, &where_) {
            (StructKind::Unit, None, None) => b.push(';'),
            (StructKind::Unit, None, Some(w)) => _ = write!(b, "where {w};"),
            (StructKind::Unit, Some(d), None) => _ = write!(b, "<{d}>;"), // very unlikely
            (StructKind::Unit, Some(d), Some(w)) => _ = write!(b, "<{d}> where {w};"),
            // TODO: need IDMap to know Path
            (StructKind::Tuple(t), None, None) => _ = write!(b, "({t:?})"),
            (StructKind::Tuple(t), None, Some(w)) => _ = write!(b, "({t:?}) where {w};"),
            (StructKind::Tuple(t), Some(d), None) => _ = write!(b, "<{d}>({t:?});"),
            (StructKind::Tuple(t), Some(d), Some(w)) => _ = write!(b, "<{d}>({t:?}) where {w};"),
            (
                StructKind::Plain {
                    fields,
                    fields_stripped,
                },
                None,
                None,
            ) => _ = write!(b, " {{{fields:?}{}}}", private(*fields_stripped)),
            (
                StructKind::Plain {
                    fields,
                    fields_stripped,
                },
                None,
                Some(w),
            ) => _ = write!(b, " where {w}\n{{{fields:?}{}}}", private(*fields_stripped)),
            (
                StructKind::Plain {
                    fields,
                    fields_stripped,
                },
                Some(d),
                None,
            ) => _ = write!(b, "<{d}>\n{{{fields:?}{}}}", private(*fields_stripped)),
            (
                StructKind::Plain {
                    fields,
                    fields_stripped,
                },
                Some(d),
                Some(w),
            ) => {
                _ = write!(
                    b,
                    "<{d}>\nwhere {w}\n{{{fields:?}{}}}",
                    private(*fields_stripped)
                )
            }
        }
        buf
    }

    fn item(item: &ItemEnum) -> Option<&Self> {
        if let ItemEnum::Struct(s) = item {
            Some(s)
        } else {
            None
        }
    }
}
