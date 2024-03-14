#![feature(c_variadic)]
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
pub fn func_dyn_trait2(_: Box<dyn ATrait + Send + Sync>) {}
pub fn func_primitive(s: &str) -> usize {
    s.len()
}
pub fn func_tuple_array_slice<'a, 'b>(
    a: &'a [u8],
    b: &'b mut [u8; 8],
    _: &'b mut (dyn 'a + ATrait),
) -> (&'a [u8], &'b mut [u8; 8]) {
    (a, b)
}
pub fn func_with_const<T: Copy, const N: usize>(t: T) -> [T; N] {
    [t; N]
}
pub fn func_lifetime_bounds<'a, 'b: 'a>()
where
    'a: 'b,
{
}
pub fn func_trait_bounds<T: Copy>()
where
    T: Clone,
{
}
pub fn func_fn_pointer_impl_trait(
    f: fn(*mut u8) -> *const u8,
) -> impl Copy + Fn(*mut u8) -> *const u8 {
    f
}
pub fn func_qualified_path<'a, I: Iterator>(mut iter: I) -> Option<I::Item>
where
    I::Item: 'a + std::fmt::Debug + Iterator<Item = ()> + ATraitWithGAT<Assoc<'a> = ()>,
{
    iter.next()
}
pub fn func_hrtb<T: ATraitWithGAT>()
where
    for<'a> <T as ATraitWithGAT>::Assoc<'a>: Copy,
{
}
/// # Safety
pub unsafe extern "C" fn variadic(_: *const (), _name: ...) {}
/// # Safety
pub unsafe extern "C" fn variadic_multiline(_: *const (), _: *mut (), _name: ...) {}

pub trait ATraitWithGAT {
    type Assoc<'a>
    where
        Self: 'a;
    fn return_assoc(&self) -> Self::Assoc<'_>;
}

pub const ACONSTANT: u8 = 123;
pub const ASTATIC: u8 = 123;

#[macro_export]
macro_rules! a_decl_macro {
    () => {};
}

pub mod structs {
    pub struct Unit;
    pub struct UnitWithBound
    where
        u8: Copy;
    pub struct UnitGeneric<const N: bool>;
    pub struct UnitGenericWithBound<const N: usize>
    where
        [(); N]:;

    pub struct Tuple((), crate::PrivateUnitStruct, pub crate::FieldsNamedStruct);
    pub struct TupleWithBound()
    where
        u8: Copy;
    pub struct TupleGeneric<'a, T: 'a, const N: usize>(pub &'a T, pub [T; N]);
    pub struct TupleGenericWithBound<'a, T: 'a, const N: usize>(pub &'a T, [T; N])
    where
        [T; N]:,
        T: Copy;
}
