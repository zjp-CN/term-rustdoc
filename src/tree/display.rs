use super::*;
use std::fmt::{self, Debug};

/// skip formatting the field when the value is empty or false
macro_rules! skip_fmt {
    ($base:ident, $self:ident . $($field:ident)+ ) => {$(
        if !$self.$field.is_empty() {
            $base.field(::std::stringify!($field), &$self.$field);
        }
    )+};
    (bool: $base:ident, $self:ident . $($field:ident)+ ) => {$(
        if $self.$field {
            $base.field(::std::stringify!($field), &$self.$field);
        }
    )+};
    (option: $base:ident, $self:ident . $($field:ident)+ ) => {$(
        if $self.$field.is_some() {
            $base.field(::std::stringify!($field), &$self.$field);
        }
    )+};
    (0: $base:ident, $self:ident . $($field:ident)+ ) => {$(
        if $self.$field != 0 {
            $base.field(::std::stringify!($field), &$self.$field);
        }
    )+};
}

impl Debug for DModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut base = f.debug_struct("DModule");
        base.field("id", &self.id);
        skip_fmt!(
            base, self . modules structs unions enums
            functions traits constants statics type_alias imports macros
        );
        base.finish()
    }
}

impl Debug for DImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut base = f.debug_struct("DImpl");
        skip_fmt!(base, self . inherent trait_ auto blanket);
        base.finish()
    }
}

impl Debug for DStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut base = f.debug_struct("DStruct");
        base.field("id", &self.id);
        skip_fmt!(bool: base, self.contain_private_fields);
        skip_fmt!(base, self . fields impls);
        base.finish()
    }
}

impl Debug for DUnion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut base = f.debug_struct("DUnion");
        base.field("id", &self.id);
        skip_fmt!(
            base, self . fields impls
        );
        base.finish()
    }
}

impl Debug for DEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut base = f.debug_struct("DEnum");
        base.field("id", &self.id);
        skip_fmt!(base, self . variants impls);
        base.finish()
    }
}

impl Debug for DTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut base = f.debug_struct("DTrait");
        base.field("id", &self.id);
        skip_fmt!(base, self . types constants functions implementations);
        base.finish()
    }
}

impl Debug for DTypeAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut base = f.debug_struct("DTypeAlias");
        base.field("id", &self.id);
        skip_fmt!(option: base, self.source_path);
        base.finish()
    }
}

impl Debug for TotolCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut base = f.debug_struct("TotolCount");
        skip_fmt!(
            0: base, self . modules structs unions enums functions
            traits constants statics type_alias imports
            macros_decl macros_func macros_attr macros_derv
        );
        base.finish()
    }
}
