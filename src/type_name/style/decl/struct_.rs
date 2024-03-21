use super::{Declaration, VisNameMap};
use crate::{
    tree::IDMap,
    type_name::style::{
        path::{FindName, Format},
        utils::{write_comma, write_comma_without_whitespace},
        Decl, Punctuation, StyledType, Syntax,
    },
};
use rustdoc_types::{Generics, Id, ItemEnum, Struct, StructKind};

impl Declaration for Struct {
    fn format<K: FindName>(&self, map: VisNameMap, buf: &mut StyledType) {
        let Struct {
            kind,
            generics:
                Generics {
                    params,
                    where_predicates,
                },
            ..
        } = self;
        let VisNameMap { vis, id, name, map } = map;
        vis.format::<K>(buf);
        buf.write(Decl::Struct);
        buf.write_id_name(id, name);
        params.format::<K>(buf);
        match kind {
            StructKind::Unit => {
                where_predicates.format::<K>(buf);
                buf.write(Punctuation::SemiColon);
            }
            StructKind::Tuple(fields) => {
                tuple::<K>(fields, map, buf);
                where_predicates.format::<K>(buf);
                buf.write(Punctuation::SemiColon);
            }
            StructKind::Plain {
                fields,
                fields_stripped,
            } => {
                where_predicates.format::<K>(buf);
                buf.write(if where_predicates.is_empty() {
                    Punctuation::WhiteSpace
                } else {
                    Punctuation::NewLine
                });
                plain::<K>(fields, *fields_stripped, map, buf);
            }
        };
    }
}

fn tuple<K: FindName>(fields: &[Option<Id>], map: &IDMap, buf: &mut StyledType) {
    if fields.iter().all(Option::is_none) {
        // all fields are private, thus no need to be multiline
        buf.write_in_parentheses(|buf| {
            buf.write_slice(
                itertools::repeat_n(Syntax::Infer, fields.len()),
                |t, buf| buf.write(t),
                write_comma,
            )
        });
        return;
    }
    buf.write_in_parentheses(|buf| {
        // if a tuple has more than one fields, make it multiline.
        let multiline = fields.len() > 1;
        buf.write_slice(
            fields,
            |f, buf| {
                if multiline {
                    buf.write(Punctuation::NewLine);
                    buf.write(Punctuation::Indent);
                }
                let id = f.as_ref().map(|id| &*id.0);
                let field = id.and_then(|id| map.get_item(id).map(|x| &x.inner));
                if let Some(ItemEnum::StructField(f)) = field {
                    f.format::<K>(buf);
                } else {
                    buf.write(Syntax::Infer);
                }
            },
            |buf| {
                if multiline {
                    write_comma_without_whitespace(buf)
                } else {
                    write_comma(buf)
                }
            },
        );
        if multiline {
            buf.write(Punctuation::NewLine);
        }
    });
}

fn plain<K: FindName>(fields: &[Id], has_private_field: bool, map: &IDMap, buf: &mut StyledType) {
    buf.write_in_brace(|buf| {
        buf.write_slice(
            fields,
            |id, buf| {
                buf.write(Punctuation::NewLine);
                buf.write(Punctuation::Indent);
                let Some(field) = map.get_item(&id.0) else {
                    error!(?id, "field item is not found");
                    return;
                };
                buf.write(field.name.as_deref().unwrap_or("???"));
                buf.write(Punctuation::Colon);
                if let ItemEnum::StructField(f) = &field.inner {
                    f.format::<K>(buf);
                } else {
                    error!(?field, "not a StructField in a struct");
                    buf.write(Syntax::Infer);
                }
            },
            write_comma_without_whitespace,
        );
        match (fields.is_empty(), has_private_field) {
            (true, true) => {
                // no public field + PrivateFields = all PrivateFields
                // oneline
                buf.write(Punctuation::WhiteSpace);
                buf.write(Decl::PrivateFields);
                buf.write(Punctuation::WhiteSpace);
            }
            (false, true) => {
                // multiline + some public fields + PrivateFields
                buf.write(Punctuation::Comma);
                buf.write(Punctuation::NewLine);
                buf.write(Punctuation::Indent);
                buf.write(Decl::PrivateFields);
                buf.write(Punctuation::NewLine);
            }
            (false, false) => {
                // multiline + all public fields
                buf.write(Punctuation::NewLine);
            }
            (true, false) => (), // no public and no PrivateFields = empty fields
        }
    });
}
