use super::{generic_arg_name, long, short, COMMA};
use crate::util::{xformat, XString};
use itertools::intersperse;
use rustdoc_types::{GenericArgs, Path};

pub fn long_path(p: &Path) -> Option<XString> {
    let name = p.name.as_str();
    match p.args.as_deref() {
        Some(GenericArgs::AngleBracketed { args, bindings: _ }) => {
            // FIXME: bindings without args
            if args.is_empty() {
                Some(name.into())
            } else {
                let arg: XString = intersperse(
                    args.iter()
                        .map(|a| generic_arg_name(a, long).unwrap_or_default()),
                    COMMA,
                )
                .collect();
                Some(xformat!("{name}<{arg}>"))
            }
        }
        Some(GenericArgs::Parenthesized { inputs, output }) => {
            let args: XString =
                intersperse(inputs.iter().map(|a| long(a).unwrap_or_default()), COMMA).collect();
            let ret = output
                .as_ref()
                .and_then(|t| Some(xformat!(" -> {}", long(t)?)))
                .unwrap_or_default();
            Some(xformat!("{name}({args}){ret}"))
        }
        None => Some(name.into()),
    }
}

/// Only show the last name in path.
pub fn short_path(p: &Path) -> Option<XString> {
    fn short_name(name: &str) -> &str {
        &name[name.rfind(':').map_or(0, |x| x + 1)..]
    }
    let name = short_name(&p.name);
    match p.args.as_deref() {
        Some(GenericArgs::AngleBracketed { args, bindings: _ }) => {
            // FIXME: bindings without args
            if args.is_empty() {
                Some(name.into())
            } else {
                let arg: XString = intersperse(
                    args.iter()
                        .map(|a| generic_arg_name(a, short).unwrap_or_default()),
                    COMMA,
                )
                .collect();
                Some(xformat!("{name}<{arg}>"))
            }
        }
        Some(GenericArgs::Parenthesized { inputs, output }) => {
            let args: XString =
                intersperse(inputs.iter().map(|a| short(a).unwrap_or_default()), COMMA).collect();
            let ret = output
                .as_ref()
                .and_then(|t| Some(xformat!(" -> {}", short(t)?)))
                .unwrap_or_default();
            Some(xformat!("{name}({args}){ret}"))
        }
        None => Some(name.into()),
    }
}
