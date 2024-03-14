use crate::{doc, shot};
use term_rustdoc::{
    decl::item_str,
    tree::{DModule, IDMap},
};

#[test]
fn fn_items() {
    let map = &doc();
    let dmod = map.dmodule();
    let mut fns_str = Vec::new();
    for fn_ in &dmod.functions {
        fns_str.push(item_str(&fn_.id, map));
    }
    shot!(DisplaySlice(&fns_str), @r###"
    pub fn func_dyn_trait(d: &(dyn ATrait + Send + Sync)) -> &dyn ATrait
    pub fn func_dyn_trait2(_: Box<dyn ATrait + Send + Sync>)
    pub fn func_fn_pointer_impl_trait(f: fn(*mut u8) -> *const u8) -> impl Copy + Fn(*mut u8) -> *const u8
    pub fn func_hrtb<T: ATraitWithGAT>()
    where
        for<'a> <T as ATraitWithGAT>::Assoc<'a>: Copy
    pub fn func_lifetime_bounds<'a, 'b: 'a>()
    where
        'a: 'b
    pub fn func_primitive(s: &str) -> usize
    pub fn func_qualified_path<'a, I: Iterator>(iter: I) -> Option<I::Item>
    where
        I::Item: 'a + Debug + Iterator<Item = ()> + ATraitWithGAT<Assoc<'a> = ()>
    pub fn func_trait_bounds<T>()
    where
        T: Clone + Copy
    pub fn func_tuple_array_slice<'a, 'b>(
        a: &'a u8,
        b: &'b mut [u8; 8],
        _: &'b mut (dyn 'a + ATrait)
    ) -> (&'a u8, &'b mut [u8; 8])
    pub fn func_with_1arg(_: FieldsNamedStruct)
    pub fn func_with_1arg_and_ret(f: FieldsNamedStruct) -> AUnitEnum
    pub fn func_with_const<T: Copy, const N: usize>(t: T) -> [T; N]
    pub fn func_with_no_args()
    pub unsafe extern "C" fn variadic(_: *const (), _: ...)
    pub unsafe extern "C" fn variadic_multiline(
        _: *const (),
        _: *mut (),
        _: ...
    )
    "###);
}

#[test]
fn methods() {
    let map = &doc();
    let dmod = map.dmodule();
    let mut fns_str = Vec::new();
    for struct_ in &dmod.structs {
        for inh in &*struct_.impls.merged_inherent.functions {
            fns_str.push(item_str(&inh.id, map));
        }
    }
    for enum_ in &dmod.enums {
        for inh in &*enum_.impls.merged_inherent.functions {
            fns_str.push(item_str(&inh.id, map));
        }
    }
    for union_ in &dmod.unions {
        for inh in &*union_.impls.merged_inherent.functions {
            fns_str.push(item_str(&inh.id, map));
        }
    }
    for trait_ in &dmod.traits {
        for fn_ in &*trait_.functions {
            fns_str.push(item_str(&fn_.id, map));
        }
    }
    shot!(DisplaySlice(&fns_str), @r###"
    pub fn by_rc(self: Rc<Self>)
    pub fn by_ref(&self)
    pub fn by_ref_mut(&mut self)
    pub fn consume(self)
    pub fn new() -> Self
    fn return_assoc(&self) -> Self::Assoc<'_>
    "###);
}

#[test]
fn structs() {
    let map = &doc();
    let dmod = map.dmodule();
    let mut structs_str = Vec::new();
    recursive_struct_str(dmod, &mut structs_str, map);
    shot!(DisplaySlice(&structs_str), @r###"
    pub struct AUnitStruct;
    pub struct FieldsNamedStruct {
        field1: AUnitStruct,
        field2: AStructAlias,
        field3: Vec<FieldsNamedStruct>,
        /* private fields */
    }
    pub struct Tuple(
        FieldsNamedStruct,
        /* private fields */
    )
    pub struct TupleGeneric<'a, T: 'a, const N: usize>(
        &'a T,
        [T; N],
    );
    pub struct TupleGenericWithBound<'a, T, const N: usize>(
        &'a T,
        /* private fields */
    )
    where
        [T; N]: ,
        T: Copy + 'a;
    pub struct TupleWithBound()
    where
        u8: Copy;
    pub struct Unit;
    pub struct UnitGeneric<const N: bool>;
    pub struct UnitGenericWithBound<const N: usize>
    where
        [(); N]: ;
    pub struct UnitWithBound
    where
        u8: Copy;
    pub struct AUnitStruct;
    pub struct AUnitStruct;
    "###);
}

fn recursive_struct_str(dmod: &DModule, structs_str: &mut Vec<String>, map: &IDMap) {
    for struct_ in &dmod.structs {
        structs_str.push(item_str(&struct_.id, map));
    }
    for m in &dmod.modules {
        recursive_struct_str(m, structs_str, map);
    }
}

struct DisplaySlice<'s, T>(&'s [T]);

impl<'s, T: std::fmt::Display> std::fmt::Display for DisplaySlice<'s, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ele in self.0 {
            _ = writeln!(f, "{ele}");
        }
        Ok(())
    }
}
