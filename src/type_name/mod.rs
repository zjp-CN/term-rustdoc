use crate::util::{xformat, XString};
use itertools::intersperse;
use rustdoc_types::{DynTrait, Path, PolyTrait, Type};
use std::fmt::Write;

mod short_or_long;
pub use short_or_long::{long_path, short_path};

mod generic;
pub use generic::generics;

mod funcion;
pub(crate) use funcion::{fn_decl, fn_header};

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
const COLON: &str = ": ";

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
        Type::RawPointer { mutable, type_ } => xformat!(
            "*{} {}",
            if *mutable { "mut" } else { "const" },
            typename::<Kind>(type_)
        ),
        Type::QualifiedPath {
            name,
            args,
            self_type,
            trait_,
        } => {
            let self_ = typename::<Kind>(self_type);
            if let Some(trait_) = trait_.as_ref().map(resolve_path).filter(|s| !s.is_empty()) {
                if let Some(args) = generic::generic_args::<Kind>(args) {
                    xformat!("<{self_} as {trait_}>::{name}{args}")
                } else {
                    xformat!("<{self_} as {trait_}>::{name}")
                }
            } else if let Some(args) = generic::generic_args::<Kind>(args) {
                xformat!("{self_}::{name}{args}")
            } else {
                xformat!("{self_}::{name}")
            }
        }
        Type::Slice(ty) => typename::<Kind>(ty),
        Type::DynTrait(poly) => dyn_trait::<Kind>(poly),
        Type::ImplTrait(b) => xformat!(
            "impl {}",
            generic::generic_bound_for_slice::<Kind>(b).unwrap_or_default()
        ),
        Type::Tuple(v) => {
            let iter = v.iter().map(|ty| typename::<Kind>(ty));
            let ty = XString::from_iter(intersperse(iter, COMMA));
            xformat!("({ty})")
        }
        Type::Array { type_, len } => {
            let ty = typename::<Kind>(type_);
            xformat!("[{ty}; {len}]")
        }
        Type::FunctionPointer(f) => funcion::fn_pointer(f),
        Type::Infer => INFER,
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

/// Format a [`rustdoc_types::Type`] with long Path inside.
pub fn long(ty: &Type) -> XString {
    typename::<Long>(ty)
}

/// Format a [`rustdoc_types::Type`] with short Path inside.
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
            let hrtb = generic::generic_param_def_for_slice::<Kind>(generic_params);
            let [sep, hrtb] = if let Some(b) = &hrtb {
                [" ", b]
            } else {
                [""; 2]
            };
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
