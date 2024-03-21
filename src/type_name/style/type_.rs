use super::{generics::hrtb, path::*, utils::*, Punctuation, StyledType, Syntax};
use rustdoc_types::{DynTrait, PolyTrait, Type};

impl Format for Type {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        buf.write_span_type_name(|buf| {
            match self {
                Type::ResolvedPath(p) => p.format::<Kind>(buf),
                Type::DynTrait(t) => {
                    buf.write(Syntax::Dyn);
                    t.format::<Kind>(buf);
                }
                Type::Generic(s) => buf.write(s),
                Type::Primitive(p) => {
                    // TODO: when external crates for std is ready, we should
                    // add an extra tag for Primitive types. But for now, we
                    // use the plain name.
                    buf.write(p);
                }
                Type::FunctionPointer(f) => f.format::<Kind>(buf),
                Type::Tuple(types) => buf.write_in_parentheses(|buf| types.format::<Kind>(buf)),
                Type::Slice(ty) => buf.write_in_squre_bracket(|b| ty.format::<Kind>(b)),
                Type::Array { type_, len } => buf.write_in_squre_bracket(|buf| {
                    type_.format::<Kind>(buf);
                    buf.write(Punctuation::SemiColon);
                    buf.write(Punctuation::WhiteSpace);
                    buf.write(len);
                }),
                Type::ImplTrait(bounds) => {
                    buf.write(Syntax::Impl);
                    bounds.format::<Kind>(buf);
                }
                Type::Infer => buf.write(Syntax::Infer),
                Type::RawPointer { mutable, type_ } => {
                    buf.write(if *mutable {
                        Syntax::RawPointerMut
                    } else {
                        Syntax::RawPointer
                    });
                    type_.format::<Kind>(buf);
                }
                Type::BorrowedRef {
                    lifetime,
                    mutable,
                    type_,
                } => borrow_ref::<Kind>(lifetime.as_deref(), *mutable, type_, buf),
                Type::QualifiedPath {
                    name,
                    args,
                    self_type,
                    trait_,
                } => {
                    // <Type as Trait>::Name-Args
                    // If the trait path is empty (but can still carry an ID), the QualifiedPath
                    // is shown without the trait. If we were to use the name, `<Type as >::Name`
                    // will be shown; if we were to use the ID, &IDMap should be addded in format.
                    if let Some(trait_path) = trait_.as_ref().filter(|s| !s.name.is_empty()) {
                        buf.write_in_angle_bracket(|buf| {
                            self_type.format::<Kind>(buf);
                            buf.write(Syntax::As);
                            trait_path.format::<Kind>(buf);
                        });
                    } else {
                        self_type.format::<Kind>(buf);
                    }
                    buf.write(Syntax::PathSep);
                    buf.write(name);
                    args.format::<Kind>(buf);
                }
            };
        });
    }
}

impl Format for [Type] {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        buf.write_slice(self, Type::format::<Kind>, write_comma);
    }
}

impl Format for DynTrait {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        let DynTrait { traits, lifetime } = self;
        // there is at least one trait
        if let Some(s) = lifetime {
            buf.write(s);
            write_plus(buf);
        }
        traits.format::<Kind>(buf);
    }
}

impl Format for PolyTrait {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        let PolyTrait {
            trait_,
            generic_params, // HRTB
        } = self;
        hrtb::<Kind>(generic_params, buf);
        trait_.format::<Kind>(buf);
    }
}

impl Format for [PolyTrait] {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        buf.write_slice(self, PolyTrait::format::<Kind>, write_plus);
    }
}

fn borrow_ref<Kind: FindName>(
    lifetime: Option<&str>,
    mutable: bool,
    ty: &Type,
    buf: &mut StyledType,
) {
    match (mutable, lifetime) {
        (false, None) => buf.write(Syntax::Reference), // &Type
        (true, None) => buf.write(Syntax::ReferenceMut), // &mut Type
        (false, Some(s)) => {
            // &'lifetime Type
            buf.write(Syntax::Reference);
            buf.write(s);
            buf.write(Punctuation::WhiteSpace);
        }
        (true, Some(s)) => {
            // &'lifetime mut Type
            buf.write(Syntax::Reference);
            buf.write(s);
            buf.write(Punctuation::WhiteSpace);
            buf.write(Syntax::Mut);
        }
    }

    match ty {
        Type::DynTrait(d) if parenthesized_type(d) => buf.write_in_parentheses(|buf| {
            buf.write(Syntax::Dyn);
            d.format::<Kind>(buf);
        }),
        _ => ty.format::<Kind>(buf),
    }
}

/// Ref: <https://doc.rust-lang.org/reference/types.html#parenthesized-types>
///
/// dyn multi-Traits behind a reference or raw pointer type needs `()` disambiguation.
///
/// bool means whether the type should be added `()`.
fn parenthesized_type(d: &DynTrait) -> bool {
    d.traits.len() + d.lifetime.is_some() as usize > 1
}
