use crate::util::{xformat, XString};
use itertools::intersperse;
use rustdoc_types::{
    DynTrait, GenericArg, GenericBound, GenericParamDef, GenericParamDefKind, Path, PolyTrait,
    TraitBoundModifier, Type,
};
use std::fmt::Write;

mod short_or_long;
pub use short_or_long::{long_path, short_path};

trait TypeName: Copy + FnOnce(&Type) -> XString {}
impl<F> TypeName for F where F: Copy + FnOnce(&Type) -> XString {}
trait ResolvePath: Copy + FnOnce(&Path) -> XString {}
impl<F> ResolvePath for F where F: Copy + FnOnce(&Path) -> XString {}

trait FindName {
    fn type_name() -> impl TypeName;
    fn resolve_path() -> impl ResolvePath;
    fn type_and_path() -> (impl TypeName, impl ResolvePath) {
        (Self::type_name(), Self::resolve_path())
    }
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
        Type::Infer => XString::new_inline("_"),
        _ => XString::new_inline(""),
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
            let iter = generic_params.iter().map(generic_param_def::<Kind>);
            let hrtb = XString::from_iter(intersperse(iter, COMMA));
            let sep = if generic_params.is_empty() { "" } else { " " };
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

fn generic_param_def<Kind: FindName>(GenericParamDef { name, kind }: &GenericParamDef) -> XString {
    let (type_name, resolve_path) = Kind::type_and_path();
    match kind {
        GenericParamDefKind::Lifetime { outlives } => {
            let outlives = outlives.iter().map(XString::from);
            xformat!(
                "{name}: {}",
                XString::from_iter(intersperse(outlives, PLUS))
            )
        }
        GenericParamDefKind::Type {
            bounds, default, ..
        } => {
            let iter = bounds.iter().map(|b| match b {
                GenericBound::TraitBound {
                    trait_,
                    generic_params,
                    modifier,
                } => {
                    let path = resolve_path(trait_);
                    let args = XString::from_iter(intersperse(
                        generic_params.iter().map(generic_param_def::<Kind>),
                        PLUS,
                    ));
                    match modifier {
                        TraitBoundModifier::None => xformat!("{path}<{args}>"),
                        TraitBoundModifier::Maybe => xformat!("?{path}<{args}>"),
                        TraitBoundModifier::MaybeConst => xformat!("~const {path}<{args}>"),
                    }
                }
                GenericBound::Outlives(life) => XString::from(life.as_str()),
            });
            xformat!(
                "{name}: {}{}",
                XString::from_iter(intersperse(iter, PLUS)),
                default
                    .as_ref()
                    .map(|ty| xformat!(" = {}", type_name(ty)))
                    .unwrap_or_default()
            )
        }
        GenericParamDefKind::Const { type_, default } => xformat!(
            "{name}: {}{}",
            type_name(type_),
            default
                .as_deref()
                .map(|s| xformat!(" = {s}"))
                .unwrap_or_default()
        ),
    }
}

fn generic_arg_name<Kind: FindName>(arg: &GenericArg) -> XString {
    let type_name = Kind::type_name();
    match arg {
        GenericArg::Lifetime(life) => life.as_str().into(),
        GenericArg::Type(ty) => type_name(ty),
        GenericArg::Const(_) => todo!(),
        GenericArg::Infer => XString::new_inline("_"),
    }
}
