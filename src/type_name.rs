use crate::util::{xformat, XString};
use itertools::intersperse;
use rustdoc_types::{GenericArg, GenericArgs, Path, Type};

pub fn type_name(ty: &Type) -> Option<XString> {
    match ty {
        Type::ResolvedPath(p) => resolved_path_name(p),
        Type::Generic(t) => Some(t.as_str().into()),
        _ => None,
    }
}

const COMMA: XString = XString::new_inline(", ");

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
