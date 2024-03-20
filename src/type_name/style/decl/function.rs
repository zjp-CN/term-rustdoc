use super::{Declaration, VisNameMap};
use crate::type_name::style::{
    path::{FindName, Format},
    utils::write_comma,
    Function as Func, Punctuation, StyledType, Syntax,
};
use rustdoc_types::{FnDecl, Function, Generics};

impl Declaration for Function {
    fn format<K: FindName>(&self, map: VisNameMap, buf: &mut StyledType) {
        map.vis.format::<K>(buf);
        let Function {
            decl,
            generics,
            header,
            has_body,
        } = self;
        header.format::<K>(buf);
        buf.write(Func::Fn);
        buf.write(map.name);
        let Generics {
            params,
            where_predicates,
        } = generics;
        if !params.is_empty() {
            params.format::<K>(buf);
        }
        decl.format::<K>(buf);
        where_predicates.format::<K>(buf);
        if !*has_body {
            // if a function has no body, it's likely an associated function in trait definition
            buf.write(Punctuation::SemiColon);
        }
    }
}

impl Format for FnDecl {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        let FnDecl {
            inputs,
            output,
            c_variadic,
        } = self;
        // Multiline for args if the count is more than 2.
        let multiline = (inputs.len() + *c_variadic as usize) > 2;
        buf.write_in_parentheses(|buf| {
            buf.write_slice(
                inputs,
                |arg, buf| {
                    if multiline {
                        buf.write(Punctuation::NewLine);
                        buf.write(Punctuation::Indent);
                    }
                    arg.format::<Kind>(buf);
                },
                write_comma,
            );
            if *c_variadic {
                write_comma(buf);
                if multiline {
                    buf.write(Punctuation::NewLine);
                    buf.write(Punctuation::Indent);
                }
                buf.write(Syntax::Variadic);
            }
            if multiline {
                buf.write(Punctuation::NewLine);
            }
        });
        if let Some(ty) = output {
            buf.write(Syntax::ReturnArrow);
            ty.format::<Kind>(buf);
        }
    }
}
