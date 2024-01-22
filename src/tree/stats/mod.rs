use super::{DMacroKind, DModule};

mod impls;
use impls::acc_sum;

#[derive(Default, Clone)]
pub struct ItemCount {
    pub modules: u32,
    pub structs: u32,
    pub unions: u32,
    pub enums: u32,
    pub functions: u32,
    pub traits: u32,
    pub constants: u32,
    pub statics: u32,
    pub type_alias: u32,
    pub imports: u32,
    pub macros_decl: u32,
    pub macros_func: u32,
    pub macros_attr: u32,
    pub macros_derv: u32,
}

impl DModule {
    /// Count the items under current module excluding the current module itself.
    #[rustfmt::skip]
    pub fn current_items_counts(&self) -> ItemCount {
        macro_rules! len {
            ($self:ident . $( $field:ident )+ ) => { $(
                let $field = $self.$field.len().try_into()
                    .expect("the count exceeds the maximum of u32");
            )+ };
        }
        len!(self . modules structs unions enums functions traits constants statics type_alias imports);
        let [mut decl, mut func, mut attr, mut derv]: [u32; 4] = Default::default();
        for m in &self.macros {
            match m.kind {
                DMacroKind::Declarative => decl += 1,
                DMacroKind::ProcFunction => func += 1,
                DMacroKind::ProcAttribute => attr += 1,
                DMacroKind::ProcDerive => derv += 1,
            }
        }
        ItemCount {
            modules, structs, unions, enums, functions, traits, constants, statics, type_alias, imports,
            macros_decl: decl,
            macros_func: func,
            macros_attr: attr,
            macros_derv: derv,
        }
    }

    /// Count all the items excluding the root itself.
    pub fn recursive_items_counts(&self) -> ItemCount {
        self.modules
            .iter()
            .map(Self::current_items_counts)
            .fold(self.current_items_counts(), acc_sum)
    }
}

/// This type implements `Add` and `AddAssign`, and it means
/// when both operands are the same, the output is the same,
/// if not, the output is `ImplKind::Both`.
///
/// This leads to before you add [`ImplCount`]s, if you care about
/// the same impl kind, you should check it by yourself.
#[derive(Clone, Copy, Debug)]
pub enum ImplKind {
    Inherent,
    Trait,
    Both,
}

#[derive(Clone, Copy)]
pub struct ImplCount {
    pub structs: u32,
    pub enums: u32,
    pub unions: u32,
    pub kind: ImplKind,
    pub total: u32,
}

#[derive(Clone)]
pub struct ImplCounts {
    pub inherent: ImplCount,
    pub trait_: ImplCount,
    pub total: ImplCount,
}

impl ImplCounts {
    pub const EMPTY: Self = ImplCounts {
        inherent: ImplCount {
            structs: 0,
            enums: 0,
            unions: 0,
            kind: ImplKind::Inherent,
            total: 0,
        },
        trait_: ImplCount {
            structs: 0,
            enums: 0,
            unions: 0,
            kind: ImplKind::Trait,
            total: 0,
        },
        total: ImplCount {
            structs: 0,
            enums: 0,
            unions: 0,
            kind: ImplKind::Both,
            total: 0,
        },
    };
}

impl DModule {
    pub fn current_impls_counts(&self) -> ImplCounts {
        let (s_in, s_tr) = {
            let iter = self.structs.iter();
            (
                iter.clone().map(|s| s.impls.inherent.len()).sum::<usize>(),
                iter.map(|s| s.impls.trait_.len()).sum::<usize>(),
            )
        };
        let (e_in, e_tr) = {
            let iter = self.enums.iter();
            (
                iter.clone().map(|s| s.impls.inherent.len()).sum::<usize>(),
                iter.map(|s| s.impls.trait_.len()).sum::<usize>(),
            )
        };
        let (u_in, u_tr) = {
            let iter = self.unions.iter();
            (
                iter.clone().map(|s| s.impls.inherent.len()).sum::<usize>(),
                iter.map(|s| s.impls.trait_.len()).sum::<usize>(),
            )
        };
        let inherent = ImplCount {
            structs: s_in as _,
            enums: e_in as _,
            unions: u_in as _,
            kind: ImplKind::Inherent,
            total: (s_in + e_in + u_in) as _,
        };
        let trait_ = ImplCount {
            structs: s_tr as _,
            enums: e_tr as _,
            unions: u_tr as _,
            kind: ImplKind::Trait,
            total: (s_tr + e_tr + u_tr) as _,
        };
        ImplCounts {
            total: inherent + trait_,
            inherent,
            trait_,
        }
    }

    pub fn recursive_impls_counts(&self) -> ImplCounts {
        self.modules
            .iter()
            .map(Self::current_impls_counts)
            .fold(self.current_impls_counts(), acc_sum)
    }
}
