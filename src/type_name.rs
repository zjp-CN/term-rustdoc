use crate::util::{xformat, XString};
use itertools::intersperse;
use rustdoc_types::{GenericArg, GenericArgs, Path, Type};

const COMMA: XString = XString::new_inline(", ");

pub fn type_name(ty: &Type) -> Option<XString> {
    match ty {
        Type::ResolvedPath(p) => resolved_path_name(p),
        Type::Generic(t) => Some(t.as_str().into()),
        Type::BorrowedRef {
            lifetime,
            mutable,
            type_,
        } => type_name(type_).map(|ty| match (lifetime, mutable) {
            (None, false) => xformat!("&{ty}"),
            (None, true) => xformat!("&mut {ty}"),
            (Some(life), false) => xformat!("&'{life} {ty}"),
            (Some(life), true) => xformat!("&'{life} mut {ty}"),
        }),
        _ => None,
    }
}

pub fn resolved_path_name(p: &Path) -> Option<XString> {
    let name = p.name.as_str();
    match p.args.as_deref() {
        Some(GenericArgs::AngleBracketed { args, bindings: _ }) => {
            // FIXME: bindings without args
            if args.is_empty() {
                Some(name.into())
            } else {
                let arg: XString =
                    intersperse(args.iter().filter_map(generic_arg_name), COMMA).collect();
                Some(xformat!("{name}<{arg}>"))
            }
        }
        Some(GenericArgs::Parenthesized { inputs, output }) => {
            let args: XString = intersperse(inputs.iter().filter_map(type_name), COMMA).collect();
            let ret = output
                .as_ref()
                .and_then(|t| Some(xformat!(" -> {}", type_name(t)?)))
                .unwrap_or_default();
            Some(xformat!("{name}({args}){ret}"))
        }
        None => Some(name.into()),
    }
}

fn generic_arg_name(arg: &GenericArg) -> Option<XString> {
    match arg {
        GenericArg::Lifetime(life) => Some(life.as_str().into()),
        GenericArg::Type(ty) => type_name(ty),
        GenericArg::Const(_) => None,
        GenericArg::Infer => Some(XString::new_inline("_")),
    }
}

/// Only show the last name in path.
pub fn short_type_name(ty: &Type) -> Option<XString> {
    match ty {
        Type::ResolvedPath(p) => dbg!(short_resolved_path_name(p)),
        Type::Generic(t) => Some(t.as_str().into()),
        Type::BorrowedRef {
            lifetime,
            mutable,
            type_,
        } => short_type_name(type_).map(|ty| match (lifetime, mutable) {
            (None, false) => xformat!("&{ty}"),
            (None, true) => xformat!("&mut {ty}"),
            (Some(life), false) => xformat!("&'{life} {ty}"),
            (Some(life), true) => xformat!("&'{life} mut {ty}"),
        }),
        _ => None,
    }
}

pub fn short_resolved_path_name(p: &Path) -> Option<XString> {
    fn short_name(name: &str) -> &str {
        &name[name.rfind(':').map_or(0, |x| x + 1)..]
    }
    dbg!(&p.name);
    let name = short_name(&p.name);
    match p.args.as_deref() {
        Some(GenericArgs::AngleBracketed { args, bindings: _ }) => {
            // FIXME: bindings without args
            if args.is_empty() {
                Some(name.into())
            } else {
                let arg: XString =
                    intersperse(args.iter().filter_map(short_generic_arg_name), COMMA).collect();
                Some(xformat!("{name}<{arg}>"))
            }
        }
        Some(GenericArgs::Parenthesized { inputs, output }) => {
            let args: XString =
                intersperse(inputs.iter().filter_map(short_type_name), COMMA).collect();
            let ret = output
                .as_ref()
                .and_then(|t| Some(xformat!(" -> {}", short_type_name(t)?)))
                .unwrap_or_default();
            Some(xformat!("{name}({args}){ret}"))
        }
        None => Some(name.into()),
    }
}

fn short_generic_arg_name(arg: &GenericArg) -> Option<XString> {
    match arg {
        GenericArg::Lifetime(life) => Some(life.as_str().into()),
        GenericArg::Type(ty) => short_type_name(ty),
        GenericArg::Const(_) => None,
        GenericArg::Infer => Some(XString::new_inline("_")),
    }
}
