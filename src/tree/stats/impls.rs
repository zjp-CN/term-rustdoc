use super::{ImplCount, ImplCounts, ImplKind, ItemCount};
use std::ops::{Add, AddAssign};

pub fn acc_sum<T: Add + AddAssign>(mut acc: T, new: T) -> T {
    acc += new;
    acc
}

impl Add for ItemCount {
    type Output = ItemCount;

    fn add(self, rhs: Self) -> Self::Output {
        ItemCount {
            modules: self.modules + rhs.modules,
            structs: self.structs + rhs.structs,
            unions: self.unions + rhs.unions,
            enums: self.enums + rhs.enums,
            functions: self.functions + rhs.functions,
            traits: self.traits + rhs.traits,
            constants: self.constants + rhs.constants,
            statics: self.statics + rhs.statics,
            type_alias: self.type_alias + rhs.type_alias,
            imports: self.imports + rhs.imports,
            macros_decl: self.macros_decl + rhs.macros_decl,
            macros_func: self.macros_func + rhs.macros_func,
            macros_attr: self.macros_attr + rhs.macros_attr,
            macros_derv: self.macros_derv + rhs.macros_derv,
        }
    }
}

impl Add for ImplKind {
    type Output = ImplKind;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ImplKind::Inherent, ImplKind::Inherent) => ImplKind::Inherent,
            (ImplKind::Trait, ImplKind::Trait) => ImplKind::Trait,
            _ => ImplKind::Both,
        }
    }
}

impl AddAssign for ImplKind {
    fn add_assign(&mut self, rhs: Self) {
        *self = match (&self, rhs) {
            (ImplKind::Inherent, ImplKind::Inherent) => ImplKind::Inherent,
            (ImplKind::Trait, ImplKind::Trait) => ImplKind::Trait,
            _ => ImplKind::Both,
        };
    }
}

impl AddAssign for ImplCount {
    fn add_assign(&mut self, rhs: Self) {
        self.structs += rhs.structs;
        self.enums += rhs.enums;
        self.unions += rhs.unions;
        self.kind += rhs.kind;
        self.total += rhs.total;
    }
}

impl Add for ImplCount {
    type Output = ImplCount;

    fn add(self, rhs: Self) -> Self::Output {
        ImplCount {
            structs: self.structs + rhs.structs,
            enums: self.enums + rhs.enums,
            unions: self.unions + rhs.unions,
            kind: self.kind + rhs.kind,
            total: self.total + rhs.total,
        }
    }
}

impl AddAssign for ItemCount {
    fn add_assign(&mut self, rhs: Self) {
        self.modules += rhs.modules;
        self.structs += rhs.structs;
        self.unions += rhs.unions;
        self.enums += rhs.enums;
        self.functions += rhs.functions;
        self.traits += rhs.traits;
        self.constants += rhs.constants;
        self.statics += rhs.statics;
        self.type_alias += rhs.type_alias;
        self.imports += rhs.imports;
        self.macros_decl += rhs.macros_decl;
        self.macros_func += rhs.macros_func;
        self.macros_attr += rhs.macros_attr;
        self.macros_derv += rhs.macros_derv;
    }
}

impl Add for ImplCounts {
    type Output = ImplCounts;

    fn add(self, rhs: Self) -> Self::Output {
        ImplCounts {
            inherent: self.inherent + rhs.trait_,
            trait_: self.trait_ + rhs.trait_,
            total: self.total + rhs.total,
        }
    }
}

impl AddAssign for ImplCounts {
    fn add_assign(&mut self, rhs: Self) {
        self.inherent += rhs.trait_;
        self.trait_ += rhs.trait_;
        self.total += rhs.total;
    }
}
