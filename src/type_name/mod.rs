use crate::util::{xformat, XString};
use itertools::intersperse;
use rustdoc_types::{
    DynTrait, GenericArg, GenericBound, GenericParamDef, GenericParamDefKind, Path, PolyTrait,
    TraitBoundModifier, Type,
};

mod short_or_long;
pub use short_or_long::{long_path, short_path};

trait TypeName: Copy + Fn(&Type) -> Option<XString> {}
impl<F> TypeName for F where F: Copy + Fn(&Type) -> Option<XString> {}
trait ResolvePath: Copy + Fn(&Path) -> Option<XString> {}
impl<F> ResolvePath for F where F: Copy + Fn(&Path) -> Option<XString> {}

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

fn typename<Kind: FindName>(ty: &Type) -> Option<XString> {
    let resolve_path = Kind::resolve_path();
    match ty {
        Type::ResolvedPath(p) => resolve_path(p),
        Type::Generic(t) => Some(t.as_str().into()),
        Type::BorrowedRef {
            lifetime,
            mutable,
            type_,
        } => typename::<Kind>(type_).map(|ty| match (lifetime, mutable) {
            (None, false) => xformat!("&{ty}"),
            (None, true) => xformat!("&mut {ty}"),
            (Some(life), false) => xformat!("&'{life} {ty}"),
            (Some(life), true) => xformat!("&'{life} mut {ty}"),
        }),
        Type::DynTrait(poly) => dyn_trait::<Kind>(poly),
        _ => None,
        // Type::Primitive(_) => todo!(),
        // Type::FunctionPointer(_) => todo!(),
        // Type::Tuple(_) => todo!(),
        // Type::Slice(_) => todo!(),
        // Type::Array { type_, len } => todo!(),
        // Type::ImplTrait(_) => todo!(),
        // Type::Infer => todo!(),
        // Type::RawPointer { mutable, type_ } => todo!(),
        // Type::QualifiedPath {
        //     name,
        //     args,
        //     self_type,
        //     trait_,
        // } => todo!(),
    }
}

pub fn long(ty: &Type) -> Option<XString> {
    typename::<Long>(ty)
}

pub fn short(ty: &Type) -> Option<XString> {
    typename::<Short>(ty)
}

fn dyn_trait<Kind: FindName>(DynTrait { traits, lifetime }: &DynTrait) -> Option<XString> {
    dbg!(traits);
    let resolve_path = Kind::resolve_path();
    let iter = traits.iter().map(
        |PolyTrait {
             trait_,
             generic_params,
         }| {
            let iter = generic_params.iter().map(generic_param_def::<Kind>);
            let hrtb = XString::from_iter(intersperse(iter, COMMA));
            let ty = resolve_path(trait_).unwrap_or_default();
            xformat!("{hrtb} {ty}")
        },
    );
    let path = intersperse(iter, PLUS).collect::<XString>();
    Some(lifetime.as_deref().map_or_else(
        || xformat!("dyn {path}"),
        |life| xformat!("dyn '{life} + {path}"),
    ))
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
                    let path = resolve_path(trait_).unwrap_or_default();
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
                    .map(|ty| xformat!(" = {}", type_name(ty).unwrap_or_default()))
                    .unwrap_or_default()
            )
        }
        GenericParamDefKind::Const { type_, default } => xformat!(
            "{name}: {}{}",
            type_name(type_).unwrap_or_default(),
            default
                .as_deref()
                .map(|s| xformat!(" = {s}"))
                .unwrap_or_default()
        ),
    }
}

fn generic_arg_name<Kind: FindName>(arg: &GenericArg) -> Option<XString> {
    let type_name = Kind::type_name();
    match arg {
        GenericArg::Lifetime(life) => Some(life.as_str().into()),
        GenericArg::Type(ty) => type_name(ty),
        GenericArg::Const(_) => None,
        GenericArg::Infer => Some(XString::new_inline("_")),
    }
}
