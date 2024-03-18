use super::{path::*, Punctuation, StyledType, Syntax};
use rustdoc_types::Type;

impl Format for Type {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        buf.write_span_type_name(|s| {
            match self {
                Type::ResolvedPath(p) => p.format::<Kind>(s),
                Type::DynTrait(_) => todo!(),
                Type::Generic(_) => todo!(),
                Type::Primitive(p) => {
                    // TODO: when external crates for std is ready, we should
                    // add an extra tag for Primitive types. But for now, we
                    // use the plain name.
                    s.write_name(p);
                }
                Type::FunctionPointer(_) => todo!(),
                Type::Tuple(_) => todo!(),
                Type::Slice(ty) => s.write_in_squre_bracket(|b| ty.format::<Kind>(b)),
                Type::Array { type_, len } => s.write_in_squre_bracket(|b| {
                    type_.format::<Kind>(b);
                    b.write_punctuation(Punctuation::SimiColon);
                    b.write_name(len);
                }),
                Type::ImplTrait(_) => todo!(),
                Type::Infer => s.write_syntax(Syntax::Infer),
                Type::RawPointer { mutable, type_ } => todo!(),
                Type::BorrowedRef {
                    lifetime,
                    mutable,
                    type_,
                } => todo!(),
                Type::QualifiedPath {
                    name,
                    args,
                    self_type,
                    trait_,
                } => todo!(),
            };
        });
    }
}
