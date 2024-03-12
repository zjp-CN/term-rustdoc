/// Documentation for struct AUnitStruct.
pub struct AUnitStruct;

pub trait ATrait {}
impl ATrait for AUnitStruct {}

struct PrivateUnitStruct;
impl ATrait for PrivateUnitStruct {}

pub mod submod1 {
    pub mod submod2 {
        pub use crate::AUnitStruct;
        pub use crate::AUnitStruct as AStructAlias;

        pub trait ATraitNeverImplementedForTypes {}
    }

    #[derive(Debug)]
    pub enum AUnitEnum {
        A,
        B,
        C,
    }

    impl AUnitEnum {
        pub fn print(&self) {
            println!("{self:?}");
        }
    }
}

pub struct FieldsNamedStruct {
    pub field1: AUnitStruct,
    pub field2: submod1::submod2::AStructAlias,
    pub field3: Vec<FieldsNamedStruct>,
    private: submod1::AUnitEnum,
}

impl FieldsNamedStruct {
    pub fn new() -> Self {
        FieldsNamedStruct {
            field1: AUnitStruct,
            field2: AUnitStruct,
            field3: Vec::new(),
            private: submod1::AUnitEnum::A,
        }
    }

    pub fn consume(self) {}
    pub fn by_ref(&self) {}
    pub fn by_ref_mut(&mut self) {}
    pub fn by_rc(self: std::rc::Rc<Self>) {}
}

impl Default for FieldsNamedStruct {
    fn default() -> Self {
        Self::new()
    }
}

pub fn func_with_no_args() {}
pub fn func_with_1arg(_: FieldsNamedStruct) {}
pub fn func_with_1arg_and_ret(f: FieldsNamedStruct) -> submod1::AUnitEnum {
    f.private
}
pub fn func_dyn_trait(d: &(dyn ATrait + Send + Sync)) -> &dyn ATrait {
    d
}

pub const ACONSTANT: u8 = 123;
pub const ASTATIC: u8 = 123;

#[macro_export]
macro_rules! a_decl_macro {
    () => {};
}
