/// Documentation for struct AStruct
pub struct AUnitStruct;

pub trait Trait {}
impl Trait for AUnitStruct {}

struct PrivateUnitStruct;
impl Trait for PrivateUnitStruct {}

pub mod submod1 {
    pub mod submod2 {
        pub use crate::AUnitStruct;
        pub use crate::AUnitStruct as AStructAlias;

        pub trait ATraitNeverImplementedForTypes {}
    }
}
