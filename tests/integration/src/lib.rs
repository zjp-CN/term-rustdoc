/// struct S
pub struct S;

pub trait Trait {}
impl Trait for S {}

struct S2;
impl Trait for S2 {}

pub mod a {
    pub mod c {
        pub use crate::S;
        pub use crate::S as S1;

        pub trait ATraitNeverImplementedForTypes {}
    }
}
