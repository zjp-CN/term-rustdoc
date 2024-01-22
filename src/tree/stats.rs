use super::{DMacroKind, DModule};
use std::ops::{Add, AddAssign};

#[derive(Default)]
pub struct TotolCount {
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

impl Add for TotolCount {
    type Output = TotolCount;

    fn add(self, rhs: Self) -> Self::Output {
        TotolCount {
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

impl AddAssign for TotolCount {
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

impl DModule {
    /// Count the items under current module excluding the current module itself.
    #[rustfmt::skip]
    pub fn current_items_counts(&self) -> TotolCount {
        macro_rules! len {
            ($self:ident . $( $field:ident )+ ) => { $(
                let $field = $self.$field.len().try_into()
                    .expect("the count exceeds the maximum of u32");
            )+ };
        }
        len!(self . modules structs unions enums functions 
             traits constants statics type_alias imports);
        let [mut decl, mut func, mut attr, mut derv]: [u32; 4] = Default::default();
        for m in &self.macros {
            match m.kind {
                DMacroKind::Declarative => decl += 1,
                DMacroKind::ProcFunction => func += 1,
                DMacroKind::ProcAttribute => attr += 1,
                DMacroKind::ProcDerive => derv += 1,
            }
        }
        TotolCount {
            modules, structs, unions, enums, functions, 
            traits, constants, statics, type_alias, imports,
            macros_decl: decl,
            macros_func: func,
            macros_attr: attr,
            macros_derv: derv,
        }
    }

    /// Count all the items excluding the root itself.
    pub fn recursive_items_counts(&self) -> TotolCount {
        self.modules.iter().map(Self::current_items_counts).fold(
            self.current_items_counts(),
            |mut acc, tc| {
                acc += tc;
                acc
            },
        )   
    }
}
