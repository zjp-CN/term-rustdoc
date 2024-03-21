use super::{path::*, utils::*, Punctuation, StyledType, Syntax, Tag};
use rustdoc_types::{
    Constant, GenericArg, GenericArgs, GenericBound, GenericParamDef, GenericParamDefKind, Term,
    TraitBoundModifier, TypeBinding, TypeBindingKind, WherePredicate,
};

/// Outlives are a vec of String, so to meet the bound on [`write_bounds`] fn,
/// we define a wrapper type here.
struct Outlives<'s>(&'s [String]);
impl<'s> IntoIterator for Outlives<'s> {
    type Item = &'s String;
    type IntoIter = std::slice::Iter<'s, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl From<&String> for Tag {
    fn from(val: &String) -> Self {
        Tag::from(val.as_str())
    }
}

impl Format for GenericParamDef {
    /// One generic parameter definition in `<...>`, but `<>` is excluded,
    /// because `Vec<GenericsDef>` is to form `<..., ..., ...>`.
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        let GenericParamDef { name, kind } = self;
        match kind {
            GenericParamDefKind::Lifetime { outlives } => {
                buf.write(name);
                if !outlives.is_empty() {
                    buf.write_bounds(Outlives(outlives));
                }
            }
            GenericParamDefKind::Type {
                bounds,
                default,
                synthetic,
            } => {
                // Don't write `impl TraitBound` in generic definition,
                // otherwise we'll see something like `pub fn f<impl Trait: Trait>(_: impl Trait)`
                if *synthetic {
                    return;
                }
                buf.write(name);
                if !bounds.is_empty() {
                    write_colon(buf);
                    buf.write_slice(bounds, GenericBound::format::<Kind>, write_plus);
                    if let Some(ty) = default {
                        buf.write(Punctuation::Equal);
                        ty.format::<Kind>(buf);
                    }
                }
            }
            GenericParamDefKind::Const { type_, default } => {
                buf.write(Syntax::Const);
                buf.write(name);
                buf.write(Punctuation::Colon);
                type_.format::<Kind>(buf);
                if let Some(s) = default {
                    buf.write(Punctuation::Equal);
                    buf.write(s);
                }
            }
        }
    }
}

impl Format for [GenericParamDef] {
    /// Full parameter definition contains multiple parameter definitions,
    /// thus we put outer `<>` here, and `, ` separator.
    /// Write nothing if the slice is empty.
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        if !self.is_empty() {
            generic_param_def_inner::<Kind>(self, buf);
        }
    }
}

fn generic_param_def_inner<Kind: FindName>(def: &[GenericParamDef], buf: &mut StyledType) {
    buf.write_in_angle_bracket(|buf| {
        buf.write_slice(def, GenericParamDef::format::<Kind>, write_comma);
    });
}

impl Format for GenericBound {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        match self {
            GenericBound::TraitBound {
                trait_,
                generic_params, // HRTB: for<...>
                modifier,       // none, ?Trait, ~const Trait
            } => {
                hrtb::<Kind>(generic_params, buf);
                modifier.format::<Kind>(buf);
                trait_.format::<Kind>(buf);
            }
            GenericBound::Outlives(s) => buf.write(s),
        };
    }
}

impl Format for [GenericBound] {
    /// Multiple bounds are concatenated with ` + `.
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        buf.write_slice(self, GenericBound::format::<Kind>, write_plus);
    }
}

impl Format for TraitBoundModifier {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        match self {
            TraitBoundModifier::None => (),
            TraitBoundModifier::Maybe => buf.write(Syntax::Maybe),
            TraitBoundModifier::MaybeConst => buf.write(Syntax::MaybeConst),
        }
    }
}

impl Format for GenericArgs {
    /// `<...>` or `(...) -> ...`
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        match self {
            GenericArgs::AngleBracketed { args, bindings } => {
                // <'a, 32, B: Copy, C = u32>
                match (args.is_empty(), bindings.is_empty()) {
                    (true, true) => (),
                    (false, true) => buf.write_in_angle_bracket(|buf| args.format::<Kind>(buf)),
                    (true, false) => buf.write_in_angle_bracket(|buf| bindings.format::<Kind>(buf)),
                    (false, false) => buf.write_in_angle_bracket(|buf| {
                        args.format::<Kind>(buf);
                        write_comma(buf);
                        bindings.format::<Kind>(buf);
                    }),
                }
            }
            // Fn(A, B) -> C
            GenericArgs::Parenthesized { inputs, output } => {
                buf.write_in_parentheses(|buf| inputs.format::<Kind>(buf));
                buf.write(Syntax::ReturnArrow);
                if let Some(ty) = output {
                    ty.format::<Kind>(buf);
                }
            }
        }
    }
}

impl Format for GenericArg {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        match self {
            GenericArg::Lifetime(s) => buf.write(s),
            GenericArg::Type(ty) => ty.format::<Kind>(buf),
            GenericArg::Const(c) => c.format::<Kind>(buf),
            GenericArg::Infer => buf.write(Syntax::Infer),
        }
    }
}

impl Format for [GenericArg] {
    /// NOTE: no AngleBracketes surrounded
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        buf.write_slice(self, GenericArg::format::<Kind>, write_comma);
    }
}

impl Format for Constant {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        // TODO: need examples
        let Constant {
            type_, expr, value, ..
        } = self;
        buf.write(expr);
        buf.write(Punctuation::Colon);
        type_.format::<Kind>(buf);
        if let Some(s) = value {
            buf.write(s);
        }
    }
}

impl Format for TypeBinding {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        // e.g. C = i32
        let TypeBinding {
            name,
            args,
            binding,
        } = self;
        buf.write(name);
        args.format::<Kind>(buf);
        binding.format::<Kind>(buf);
    }
}

impl Format for [TypeBinding] {
    /// NOTE: no AngleBracketes surrounded
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        buf.write_slice(self, TypeBinding::format::<Kind>, write_comma);
    }
}

impl Format for TypeBindingKind {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        match self {
            TypeBindingKind::Equality(term) => {
                buf.write(Punctuation::Equal);
                term.format::<Kind>(buf);
            }
            TypeBindingKind::Constraint(bounds) => {
                buf.write(Punctuation::Colon);
                bounds.format::<Kind>(buf);
            }
        }
    }
}

impl Format for Term {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        match self {
            Term::Type(ty) => ty.format::<Kind>(buf),
            Term::Constant(c) => c.format::<Kind>(buf),
        }
    }
}

pub fn hrtb<Kind: FindName>(def: &[GenericParamDef], buf: &mut StyledType) {
    if !def.is_empty() {
        buf.write(Syntax::For);
        generic_param_def_inner::<Kind>(def, buf);
        buf.write(Punctuation::WhiteSpace);
    }
}

impl Format for WherePredicate {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        match self {
            WherePredicate::BoundPredicate {
                type_,
                bounds,
                generic_params, // HRTB
            } => {
                hrtb::<Kind>(generic_params, buf);
                type_.format::<Kind>(buf);
                // colon is necessary here because empty bound like `[T; N]:` is allowed.
                buf.write(Punctuation::Colon);
                bounds.format::<Kind>(buf);
            }
            WherePredicate::RegionPredicate { lifetime, bounds } => {
                buf.write(lifetime);
                buf.write(Punctuation::Colon);
                bounds.format::<Kind>(buf);
            }
            WherePredicate::EqPredicate { lhs, rhs } => {
                lhs.format::<Kind>(buf);
                buf.write(Punctuation::Equal);
                rhs.format::<Kind>(buf);
            }
        }
    }
}

impl Format for [WherePredicate] {
    /// Emit `\nwhere\n...` if not empty; do nothing if empty.
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        if self.is_empty() {
            return;
        }
        buf.write(Punctuation::NewLine);
        buf.write(Syntax::Where);
        buf.write_slice(
            self,
            |t, buf| {
                buf.write(Punctuation::NewLine);
                buf.write(Punctuation::Indent);
                t.format::<Kind>(buf);
            },
            write_comma,
        );
    }
}
