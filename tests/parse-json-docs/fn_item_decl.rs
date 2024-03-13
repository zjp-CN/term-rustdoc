use crate::{doc, snap};
use term_rustdoc::decl::fn_item;

#[test]
fn fn_items() {
    let map = &doc();
    let dmod = map.dmodule();
    let mut fns_str = Vec::new();
    for fn_ in &dmod.functions {
        fns_str.push(fn_item(&fn_.id, map));
    }
    snap!(fns_str, @r###"
    [
        "pub fn func_dyn_trait(d: &(dyn ATrait + Send + Sync)) -> &dyn ATrait",
        "pub fn func_dyn_trait2(_: Box<dyn ATrait + Send + Sync>)",
        "pub fn func_fn_pointer_impl_trait(f: fn(_: *mut u8) -> *const u8) -> impl Copy + Fn(*mut u8) -> *const u8",
        "pub fn func_lifetime_bounds<'a, 'b: 'a>() where 'a: 'b",
        "pub fn func_primitive(s: &str) -> usize",
        "pub fn func_trait_bounds<T>() where T: Clone + Copy",
        "pub fn func_tuple_array_slice<'a, 'b>(a: &'a u8, b: &'b mut [u8; 8], _: &'b mut (dyn 'a + ATrait)) -> (&'a u8, &'b mut [u8; 8])",
        "pub fn func_with_1arg(_: FieldsNamedStruct)",
        "pub fn func_with_1arg_and_ret(f: FieldsNamedStruct) -> AUnitEnum",
        "pub fn func_with_const<T: Copy, const N: usize>(t: T) -> [T; N]",
        "pub fn func_with_no_args()",
    ]
    "###);
}

#[test]
fn methods() {
    let map = &doc();
    let dmod = map.dmodule();
    let mut fns_str = Vec::new();
    for struct_ in &dmod.structs {
        for inh in &*struct_.impls.merged_inherent.functions {
            fns_str.push(fn_item(&inh.id, map));
        }
    }
    for enum_ in &dmod.enums {
        for inh in &*enum_.impls.merged_inherent.functions {
            fns_str.push(fn_item(&inh.id, map));
        }
    }
    for union_ in &dmod.unions {
        for inh in &*union_.impls.merged_inherent.functions {
            fns_str.push(fn_item(&inh.id, map));
        }
    }
    for trait_ in &dmod.traits {
        for fn_ in &*trait_.functions {
            fns_str.push(fn_item(&fn_.id, map));
        }
    }
    snap!(fns_str, @r###"
    [
        "pub fn by_rc(self: Rc<Self>)",
        "pub fn by_ref(&self)",
        "pub fn by_ref_mut(&mut self)",
        "pub fn consume(self)",
        "pub fn new() -> Self",
    ]
    "###);
}
