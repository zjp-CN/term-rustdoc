use super::{Declaration, VisNameMap};
use crate::type_name::style::{
    path::{FindName, Format},
    utils::write_comma,
    Function as Func, Punctuation, StyledType, Syntax,
};
use rustdoc_types::{FnDecl, Function, Generics, Type};

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
                    fn_argument::<Kind>(arg, buf);
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

// Special check for self receiver:
// self and Self are strict keywords, meaning they are only allowed to be used
// as receiver the first arguement in methods, so they will never be seen in incorrect context.
// We could check the receiver case in arg slice, but to keep things simple,
// only check self/Self in functions for all arguements.
fn fn_argument<Kind: FindName>(arg @ (name, ty): &(String, Type), buf: &mut StyledType) {
    if name == "self" {
        match ty {
            Type::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } if matches!(&**type_, Type::Generic(s) if s == "Self") => {
                match (lifetime, mutable) {
                    (None, false) => {
                        // &self
                        buf.write(Syntax::Reference);
                        buf.write(Syntax::self_);
                    }
                    (None, true) => {
                        // &mut self
                        buf.write(Syntax::ReferenceMut);
                        buf.write(Syntax::self_);
                    }
                    (Some(life), false) => {
                        // &'life self
                        buf.write(Syntax::Reference);
                        buf.write(life);
                        buf.write(Punctuation::WhiteSpace);
                        buf.write(Syntax::self_);
                    }
                    (Some(life), true) => {
                        // &'life mut self
                        buf.write(Syntax::Reference);
                        buf.write(life);
                        buf.write(Punctuation::WhiteSpace);
                        buf.write(Syntax::Mut);
                        buf.write(Syntax::self_);
                    }
                }
            }
            Type::Generic(s) if s == "Self" => buf.write(Syntax::self_), // self
            _ => arg.format::<Kind>(buf), // self: Type (Box<Self>/Rc<Self>/Arc<Self>/...)
        }
    } else {
        arg.format::<Kind>(buf)
    }
}
