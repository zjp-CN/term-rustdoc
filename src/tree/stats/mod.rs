use super::DModule;

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
        len!(self . modules structs unions enums functions traits constants statics type_alias macros_decl macros_func macros_attr macros_derv);
        ItemCount {
            modules, structs, unions, enums, functions, traits, constants, statics,
            type_alias, macros_decl, macros_func, macros_attr, macros_derv,
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

/// Count the impl **blocks**.
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

/* FIXME: to elaborate metrics on Impls

/// Count impl blocks.
#[derive(Clone, Debug)]
pub struct ImplBlockCounts(pub ImplCounts);

/// Count functions (including methods) in impl blocks w.r.t
/// * the amount of items
/// * the amount of arguments
/// * the amount of generics (lifetime/type/constant)
/// * the amount of trait bounds (lifetime/type/constant)
/// * the occurence of identical fn name (using the Vec<Name> instead of digits)
#[derive(Clone, Debug)]
pub struct ImplFunctionCounts {
    pub items: ImplCounts,
}

/// Count methods (the receiver is ) in impl blocks w.r.t
/// * all the metrics on ImplFunctionCounts
/// * and the amount of receiver types separately
///   * Self/&Self/&mut Self/Box<Self>/Rc<Self>/Arc<Self>/Pin<P>
///     see <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
#[derive(Clone, Debug)]
pub struct ImplMethodCounts(pub ImplCounts);
*/
