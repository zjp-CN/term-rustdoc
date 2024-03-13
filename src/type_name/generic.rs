use super::{short, typename, FindName, Short, COLON, COMMA, INFER, PLUS};
use crate::util::{xformat, XString};
use itertools::intersperse;
use rustdoc_types::{
    Constant, GenericArg, GenericArgs, GenericBound, GenericParamDef, GenericParamDefKind,
    Generics, Term, TraitBoundModifier, TypeBinding, TypeBindingKind, WherePredicate,
};

pub fn generic_param_def_for_slice<Kind: FindName>(
    generic_params: &[GenericParamDef],
) -> Option<XString> {
    if generic_params.is_empty() {
        return None;
    }
    Some(XString::from_iter(intersperse(
        generic_params.iter().map(generic_param_def::<Kind>),
        COMMA,
    )))
}

fn generic_param_def<Kind: FindName>(GenericParamDef { name, kind }: &GenericParamDef) -> XString {
    let type_name = Kind::type_name();
    match kind {
        GenericParamDefKind::Lifetime { outlives } => {
            if outlives.is_empty() {
                name.as_str().into()
            } else {
                let outlives = outlives.iter().map(XString::from);
                xformat!(
                    "{name}: {}",
                    XString::from_iter(intersperse(outlives, PLUS))
                )
            }
        }
        GenericParamDefKind::Type {
            bounds, default, ..
        } => {
            let bound = generic_bound_for_slice::<Kind>(bounds);
            let [sep, bound] = if let Some(b) = &bound {
                [COLON, b]
            } else {
                [""; 2]
            };
            xformat!(
                "{name}{sep}{bound}{}",
                default
                    .as_ref()
                    .map(|ty| xformat!(" = {}", type_name(ty)))
                    .unwrap_or_default()
            )
        }
        GenericParamDefKind::Const { type_, default } => xformat!(
            "const {name}: {}{}",
            type_name(type_),
            default
                .as_deref()
                .map(|s| xformat!(" = {s}"))
                .unwrap_or_default()
        ),
    }
}

pub fn generic_bound_for_slice<Kind: FindName>(b: &[GenericBound]) -> Option<XString> {
    if b.is_empty() {
        return None;
    }

    Some(XString::from_iter(intersperse(
        b.iter().map(generic_bound::<Kind>),
        PLUS,
    )))
}

fn generic_bound<Kind: FindName>(b: &GenericBound) -> XString {
    match b {
        GenericBound::TraitBound {
            trait_,
            generic_params,
            modifier,
        } => {
            let path = (Kind::resolve_path())(trait_);
            let args = generic_param_def_for_slice::<Kind>(generic_params);
            if let Some(args) = args {
                match modifier {
                    TraitBoundModifier::None => xformat!("{path}<{args}>"),
                    TraitBoundModifier::Maybe => xformat!("?{path}<{args}>"),
                    TraitBoundModifier::MaybeConst => xformat!("~const {path}<{args}>"),
                }
            } else {
                match modifier {
                    TraitBoundModifier::None => xformat!("{path}"),
                    TraitBoundModifier::Maybe => xformat!("?{path}"),
                    TraitBoundModifier::MaybeConst => xformat!("~const {path}"),
                }
            }
        }
        GenericBound::Outlives(life) => XString::from(life.as_str()),
    }
}

pub fn generic_arg_name<Kind: FindName>(arg: &GenericArg) -> XString {
    let type_name = Kind::type_name();
    match arg {
        GenericArg::Lifetime(life) => life.as_str().into(),
        GenericArg::Type(ty) => type_name(ty),
        GenericArg::Const(c) => constant::<Kind>(c),
        GenericArg::Infer => INFER,
    }
}

fn generic_arg_name_for_slice<Kind: FindName>(a: &[GenericArg]) -> Option<XString> {
    if a.is_empty() {
        return None;
    }
    Some(XString::from_iter(intersperse(
        a.iter().map(generic_arg_name::<Kind>),
        COMMA,
    )))
}

fn constant<Kind: FindName>(
    Constant {
        type_, expr, value, ..
    }: &Constant,
) -> XString {
    let ty = typename::<Kind>(type_);
    let mut res = xformat!("{ty}: {expr}");
    if let Some(value) = value {
        res.push_str(" = ");
        res.push_str(value);
    }
    res
}

/// Generics definition and where bounds on items.
///
/// This use short names inside.
pub fn generics(
    Generics {
        params,
        where_predicates,
    }: &Generics,
) -> (Option<XString>, Option<XString>) {
    fn where_(w: &WherePredicate) -> XString {
        match w {
            WherePredicate::BoundPredicate {
                type_,
                bounds,
                generic_params,
            } => {
                let ty = short(type_);
                let generic_bound = generic_bound_for_slice::<Short>(bounds);
                let [sep_b, bound] = if let Some(b) = &generic_bound {
                    [COLON, b]
                } else {
                    [""; 2]
                };
                let hrtb = generic_param_def_for_slice::<Short>(generic_params);
                let [sep, hrtb] = if let Some(param) = &hrtb {
                    [" ", param]
                } else {
                    [""; 2]
                };
                xformat!("{hrtb}{sep}{ty}{sep_b}{bound}")
            }
            WherePredicate::RegionPredicate { lifetime, bounds } => {
                let generic_bound = generic_bound_for_slice::<Short>(bounds);
                let [sep, bound] = if let Some(b) = &generic_bound {
                    [COLON, b.as_str()]
                } else {
                    ["", ""]
                };
                xformat!("{lifetime}{sep}{bound}")
            }
            WherePredicate::EqPredicate { lhs, rhs } => {
                let ty = short(lhs);
                xformat!("{ty} = {}", term(rhs))
            }
        }
    }
    fn where_for_slice(w: &[WherePredicate]) -> Option<XString> {
        if w.is_empty() {
            return None;
        }
        Some(XString::from_iter(intersperse(w.iter().map(where_), COMMA)))
    }

    (
        generic_param_def_for_slice::<Short>(params),
        where_for_slice(where_predicates),
    )
}

fn term(rhs: &Term) -> XString {
    match rhs {
        Term::Type(t) => short(t),
        Term::Constant(c) => constant::<Short>(c),
    }
}

pub fn generic_args<Kind: FindName>(g: &GenericArgs) -> Option<XString> {
    match g {
        GenericArgs::AngleBracketed { args, bindings } => {
            dbg!(g);
            let binding = type_binding_for_slice::<Kind>(bindings);
            let arg = generic_arg_name_for_slice::<Kind>(args);
            Some(match (&arg, &binding) {
                (None, None) => return None,
                (Some(arg), None) => xformat!("<{arg}>"),
                (None, Some(binding)) => xformat!("<{binding}>"),
                (Some(arg), Some(binding)) => xformat!("<{arg}: {binding}>"),
            })
        }
        GenericArgs::Parenthesized { inputs, output } => {
            let type_name = Kind::type_name();
            #[allow(clippy::redundant_closure)]
            let args: XString = intersperse(inputs.iter().map(|x| type_name(x)), COMMA).collect();
            let ret = output
                .as_ref()
                .map(|t| xformat!(" -> {}", type_name(t)))
                .unwrap_or_default();
            Some(xformat!("({args}){ret}"))
        }
    }
}

fn type_binding<Kind: FindName>(
    TypeBinding {
        name,
        args,
        binding,
    }: &TypeBinding,
) -> XString {
    let arg = generic_args::<Kind>(args);
    let arg = arg.as_deref().unwrap_or("");
    match binding {
        TypeBindingKind::Equality(t) => xformat!("{name}{arg} = {}", term(t)),
        TypeBindingKind::Constraint(b) => {
            let bound = generic_bound_for_slice::<Kind>(b);
            let bound = bound.as_deref().unwrap_or("");
            xformat!("{name}{arg}: {bound}")
        }
    }
}

fn type_binding_for_slice<Kind: FindName>(b: &[TypeBinding]) -> Option<XString> {
    if b.is_empty() {
        return None;
    }
    Some(XString::from_iter(intersperse(
        b.iter().map(type_binding::<Kind>),
        COMMA,
    )))
}
