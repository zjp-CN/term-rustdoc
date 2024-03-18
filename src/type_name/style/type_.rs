use super::{path::*, StyledType};
use rustdoc_types::Type;

impl Format for Type {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        match self {
            Type::ResolvedPath(p) => p.format::<Kind>(buf),
            Type::DynTrait(_) => todo!(),
            Type::Generic(_) => todo!(),
            Type::Primitive(_) => todo!(),
            Type::FunctionPointer(_) => todo!(),
            Type::Tuple(_) => todo!(),
            Type::Slice(_) => todo!(),
            Type::Array { type_, len } => todo!(),
            Type::ImplTrait(_) => todo!(),
            Type::Infer => todo!(),
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
    }
}
