use crate::{doc, snap};
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
    snap!(fns_str, @r###"
    [
        "pub fn func_dyn_trait(d: &(dyn ATrait + Send + Sync)) -> &dyn ATrait",
        "pub fn func_dyn_trait2(_: Box<dyn ATrait + Send + Sync>)",
        "pub fn func_fn_pointer_impl_trait(f: fn(*mut u8) -> *const u8) -> impl Copy + Fn(*mut u8) -> *const u8",
        "pub fn func_hrtb<T: ATraitWithGAT>() where for<'a> <T as ATraitWithGAT>::Assoc<'a>: Copy",
        "pub fn func_lifetime_bounds<'a, 'b: 'a>() where 'a: 'b",
        "pub fn func_primitive(s: &str) -> usize",
        "pub fn func_qualified_path<'a, I: Iterator>(iter: I) -> Option<I::Item> where I::Item: 'a + Debug + Iterator<Item = ()> + ATraitWithGAT<Assoc<'a> = ()>",
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
    snap!(fns_str, @r###"
    [
        "pub fn by_rc(self: Rc<Self>)",
        "pub fn by_ref(&self)",
        "pub fn by_ref_mut(&mut self)",
        "pub fn consume(self)",
        "pub fn new() -> Self",
        "fn return_assoc(&self) -> Self::Assoc<'_>",
    ]
    "###);
}

#[test]
fn structs() {
    let map = &doc();
    let dmod = map.dmodule();
    let mut structs_str = Vec::new();
    recursive_struct_str(dmod, &mut structs_str, map);
    snap!(structs_str, @r###"
    [
        "pub struct AUnitStruct;",
        "pub struct FieldsNamedStruct {[Id(\"0:18:1800\"), Id(\"0:19:1801\"), Id(\"0:20:1802\")]/* private fields */}",
        "pub struct AUnitStruct;",
        "pub struct AUnitStruct;",
    ]
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
