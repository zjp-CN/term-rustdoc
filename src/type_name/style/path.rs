use super::StyledType;
use rustdoc_types::{Path, Type};

// pub trait TypeName: Copy + FnOnce(&Type, &mut StyledType) {}
// impl<F> TypeName for F where F: Copy + FnOnce(&Type, &mut StyledType) {}
pub trait ResolvePath: Copy + FnOnce(&Path, &mut StyledType) {}
impl<F> ResolvePath for F where F: Copy + FnOnce(&Path, &mut StyledType) {}

pub trait FindName {
    // fn type_name() -> impl TypeName;
    fn resolve_path() -> impl ResolvePath;
    // fn type_and_path() -> (impl TypeName, impl ResolvePath) {
    //     (Self::type_name(), Self::resolve_path())
    // }
}

pub struct Short;

impl FindName for Short {
    // fn type_name() -> impl TypeName {
    //     short
    // }
    fn resolve_path() -> impl ResolvePath {
        __short_path__
    }
}

pub struct Long;

impl FindName for Long {
    // fn type_name() -> impl TypeName {
    //     long
    // }
    fn resolve_path() -> impl ResolvePath {
        __long_path__
    }
}

// pub fn short(ty: &Type, buf: &mut StyledType) {
//     <Type as Format>::format::<Short>(ty, buf);
// }

pub fn long(ty: &Type) -> String {
    let mut buf = StyledType::with_capacity(16);
    <Type as Format>::format::<Long>(ty, &mut buf);
    buf.to_non_wrapped_string()
}

pub fn long_path(p: &Path) -> String {
    let mut buf = StyledType::with_capacity(16);
    let Path { name, id, args } = p;
    buf.write_span_path_name(|s| s.write_id_name(id, name));
    if let Some(generic_args) = args.as_deref() {
        generic_args.format::<Long>(&mut buf);
    }
    buf.to_non_wrapped_string()
}

/// Show full names in path.
///
/// Not guaranteed to always be an absolute path for any Path.
pub fn __long_path__(p: &Path, buf: &mut StyledType) {
    let Path { name, id, args } = p;
    buf.write_span_path_name(|s| s.write_id_name(id, name));
    if let Some(generic_args) = args.as_deref() {
        generic_args.format::<Long>(buf);
    }
}

/// Only show the last name in path.
pub fn __short_path__(p: &Path, buf: &mut StyledType) {
    fn short_name(name: &str) -> &str {
        &name[name.rfind(':').map_or(0, |x| x + 1)..]
    }
    let Path { name, id, args } = p;
    let name = short_name(name);
    buf.write_span_path_name(|s| s.write_id_name(id, name));
    if let Some(generic_args) = args.as_deref() {
        generic_args.format::<Short>(buf);
    }
}

pub trait Format {
    fn format<Kind: FindName>(&self, buf: &mut StyledType);
    // fn format_as_short(&self, buf: &mut StyledType) {
    //     self.format::<Short>(buf);
    // }
}

impl Format for Path {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        (Kind::resolve_path())(self, buf);
    }
}

// Turn Format into trait object.
// trait FormatObj<K: FindName> {
//     fn format_styled(&self, buf: &mut StyledType);
// }
// impl<T: Format, K: FindName> FormatObj<K> for T {
//     fn format_styled(&self, buf: &mut StyledType) {
//         self.format::<K>(buf);
//     }
// }
// fn check<K: FindName>(_: &dyn FormatObj<K>) {}
