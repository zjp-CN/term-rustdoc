use super::{generic::generic_arg_name, long, short, Long, Short, COMMA};
use crate::util::{xformat, XString};
use itertools::intersperse;
use rustdoc_types::{GenericArgs, Path};

pub fn long_path(p: &Path) -> XString {
    let name = p.name.as_str();
    match p.args.as_deref() {
        Some(GenericArgs::AngleBracketed { args, bindings: _ }) => {
            // FIXME: bindings without args
            if args.is_empty() {
                name.into()
            } else {
                let arg: XString =
                    intersperse(args.iter().map(generic_arg_name::<Long>), COMMA).collect();
                xformat!("{name}<{arg}>")
            }
        }
        Some(GenericArgs::Parenthesized { inputs, output }) => {
            let args: XString = intersperse(inputs.iter().map(long), COMMA).collect();
            let ret = output
                .as_ref()
                .map(|t| xformat!(" -> {}", long(t)))
                .unwrap_or_default();
            xformat!("{name}({args}){ret}")
        }
        None => name.into(),
    }
}

/// Only show the last name in path.
pub fn short_path(p: &Path) -> XString {
    fn short_name(name: &str) -> &str {
        &name[name.rfind(':').map_or(0, |x| x + 1)..]
    }
    let name = short_name(&p.name);
    match p.args.as_deref() {
        Some(GenericArgs::AngleBracketed { args, bindings: _ }) => {
            // FIXME: bindings without args
            if args.is_empty() {
                name.into()
            } else {
                let arg: XString =
                    intersperse(args.iter().map(generic_arg_name::<Short>), COMMA).collect();
                xformat!("{name}<{arg}>")
            }
        }
        Some(GenericArgs::Parenthesized { inputs, output }) => {
            let args: XString = intersperse(inputs.iter().map(short), COMMA).collect();
            let ret = output
                .as_ref()
                .map(|t| xformat!(" -> {}", short(t)))
                .unwrap_or_default();
            xformat!("{name}({args}){ret}")
        }
        None => name.into(),
    }
}
