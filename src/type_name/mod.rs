use crate::util::{xformat, XString};
use itertools::intersperse;
use rustdoc_types::{DynTrait, Path, PolyTrait, Type};
use std::fmt::Write;

mod short_or_long;
pub use short_or_long::{long_path, short_path};

mod generic;
pub use generic::generics;

trait TypeName: Copy + FnOnce(&Type) -> XString {}
impl<F> TypeName for F where F: Copy + FnOnce(&Type) -> XString {}
trait ResolvePath: Copy + FnOnce(&Path) -> XString {}
impl<F> ResolvePath for F where F: Copy + FnOnce(&Path) -> XString {}

trait FindName {
    fn type_name() -> impl TypeName;
    fn resolve_path() -> impl ResolvePath;
    // fn type_and_path() -> (impl TypeName, impl ResolvePath) {
    //     (Self::type_name(), Self::resolve_path())
    // }
}

struct Short;

impl FindName for Short {
    fn type_name() -> impl TypeName {
        short
    }
    fn resolve_path() -> impl ResolvePath {
        short_path
    }
}

struct Long;

impl FindName for Long {
    fn type_name() -> impl TypeName {
        long
    }
    fn resolve_path() -> impl ResolvePath {
        long_path
    }
}

const COMMA: XString = XString::new_inline(", ");
const PLUS: XString = XString::new_inline(" + ");
const INFER: XString = XString::new_inline("_");
const EMPTY: XString = XString::new_inline("");
const COLON: XString = XString::new_inline(": ");

fn typename<Kind: FindName>(ty: &Type) -> XString {
    let resolve_path = Kind::resolve_path();
    match ty {
        Type::ResolvedPath(p) => resolve_path(p),
        Type::Generic(t) | Type::Primitive(t) => t.as_str().into(),
        Type::BorrowedRef {
            lifetime,
            mutable,
            type_,
        } => borrow_ref::<Kind>(type_, lifetime, mutable),
        Type::Tuple(v) => {
            let iter = v.iter().map(|ty| typename::<Kind>(ty));
            let ty = XString::from_iter(intersperse(iter, COMMA));
            xformat!("({ty})")
        }
        Type::Slice(ty) => typename::<Kind>(ty),
        Type::Array { type_, len } => {
            let ty = typename::<Kind>(type_);
            xformat!("[{ty}; {len}]")
        }
        Type::DynTrait(poly) => dyn_trait::<Kind>(poly),
        Type::Infer => INFER,
        _ => EMPTY,
        // Type::FunctionPointer(_) => todo!(),
        // Type::ImplTrait(_) => todo!(),
        // Type::RawPointer { mutable, type_ } => todo!(),
        // Type::QualifiedPath {
        //     name,
        //     args,
        //     self_type,
        //     trait_,
        // } => todo!(),
    }
}

fn borrow_ref<Kind: FindName>(type_: &Type, lifetime: &Option<String>, mutable: &bool) -> XString {
    let mut buf = match (lifetime, mutable) {
        (None, false) => xformat!("&"),
        (None, true) => xformat!("&mut "),
        (Some(life), false) => xformat!("&{life} "),
        (Some(life), true) => xformat!("&{life} mut "),
    };
    if let Type::DynTrait(d) = type_ {
        let (ty, add) = parenthesized_type::<Kind>(d);
        if add {
            write!(buf, "({ty})").unwrap();
        } else {
            buf.push_str(&ty);
        }
    } else {
        buf.push_str(&typename::<Kind>(type_));
    }
    buf
}

pub fn long(ty: &Type) -> XString {
    typename::<Long>(ty)
}

pub fn short(ty: &Type) -> XString {
    typename::<Short>(ty)
}

fn dyn_trait<Kind: FindName>(DynTrait { traits, lifetime }: &DynTrait) -> XString {
    let resolve_path = Kind::resolve_path();
    let iter = traits.iter().map(
        |PolyTrait {
             trait_,
             generic_params,
         }| {
            let [sep, hrtb] = generic::generic_param_def_for_slice::<Kind>(generic_params);
            let sep = if sep.is_empty() { "" } else { " " };
            let ty = resolve_path(trait_);
            xformat!("{hrtb}{sep}{ty}")
        },
    );
    let path = intersperse(iter, PLUS).collect::<XString>();
    lifetime.as_deref().map_or_else(
        || xformat!("dyn {path}"),
        |life| xformat!("dyn {life} + {path}"),
    )
}

/// Ref: <https://doc.rust-lang.org/reference/types.html#parenthesized-types>
///
/// dyn multi-Traits behind a reference or raw pointer type needs `()` disambiguation.
///
/// bool means whether the XString should be added `()`.
fn parenthesized_type<Kind: FindName>(d: &DynTrait) -> (XString, bool) {
    let s = dyn_trait::<Kind>(d);
    (s, d.traits.len() + d.lifetime.is_some() as usize > 1)
}
