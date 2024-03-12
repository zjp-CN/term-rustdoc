use crate::util::{xformat, XString};
use itertools::intersperse;
use rustdoc_types::{
    DynTrait, GenericArg, GenericBound, GenericParamDef, GenericParamDefKind, Path, PolyTrait,
    TraitBoundModifier, Type,
};

mod short_or_long;
pub use short_or_long::{long_path, short_path};

pub trait TypeName: Copy + Fn(&Type) -> Option<XString> {}
impl<F> TypeName for F where F: Copy + Fn(&Type) -> Option<XString> {}
pub trait ResolvePath: Copy + Fn(&Path) -> Option<XString> {}
impl<F> ResolvePath for F where F: Copy + Fn(&Path) -> Option<XString> {}

const COMMA: XString = XString::new_inline(", ");
const PLUS: XString = XString::new_inline(" + ");

fn typename(
    ty: &Type,
    resolve_path: impl ResolvePath,
    type_name: impl TypeName,
) -> Option<XString> {
    match ty {
        Type::ResolvedPath(p) => resolve_path(p),
        Type::Generic(t) => Some(t.as_str().into()),
        Type::BorrowedRef {
            lifetime,
            mutable,
            type_,
        } => typename(type_, resolve_path, type_name).map(|ty| match (lifetime, mutable) {
            (None, false) => xformat!("&{ty}"),
            (None, true) => xformat!("&mut {ty}"),
            (Some(life), false) => xformat!("&'{life} {ty}"),
            (Some(life), true) => xformat!("&'{life} mut {ty}"),
        }),
        Type::DynTrait(poly) => dyn_trait(poly, resolve_path, type_name),
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
    typename(ty, long_path, long)
}

pub fn short(ty: &Type) -> Option<XString> {
    typename(ty, short_path, short)
}

fn dyn_trait(
    DynTrait { traits, lifetime }: &DynTrait,
    resolve_path: impl ResolvePath,
    type_name: impl TypeName,
) -> Option<XString> {
    dbg!(traits);
    let iter = traits.iter().map(
        |PolyTrait {
             trait_,
             generic_params,
         }| {
            let iter = generic_params
                .iter()
                .map(|a| generic_param_def(a, resolve_path, type_name));
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

fn generic_param_def(
    GenericParamDef { name, kind }: &GenericParamDef,
    resolve_path: impl ResolvePath,
    type_name: impl TypeName,
) -> XString {
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
                        generic_params
                            .iter()
                            .map(|a| generic_param_def(a, resolve_path, type_name)),
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

fn generic_arg_name(arg: &GenericArg, type_name: impl TypeName) -> Option<XString> {
    match arg {
        GenericArg::Lifetime(life) => Some(life.as_str().into()),
        GenericArg::Type(ty) => type_name(ty),
        GenericArg::Const(_) => None,
        GenericArg::Infer => Some(XString::new_inline("_")),
    }
}
