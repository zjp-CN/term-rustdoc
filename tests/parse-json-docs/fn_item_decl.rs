use crate::{doc, shot, snap};
use term_rustdoc::{
    tree::{DModule, IDMap},
    type_name::{DeclarationLines, StyledType},
};

#[test]
fn fn_items() {
    let map = &doc();
    let dmod = map.dmodule();
    let mut styled_fn = Vec::with_capacity(24);
    for fn_ in &dmod.functions {
        styled_fn.push(StyledType::new(&fn_.id, map));
    }
    shot!(DisplaySlice(&styled_fn), @r###"
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
        a: &'a [u8], 
        b: &'b mut [u8; 8], 
        _: &'b mut (dyn 'a + ATrait)
    ) -> (&'a [u8], &'b mut [u8; 8])
    pub fn func_with_1arg(_: FieldsNamedStruct)
    pub fn func_with_1arg_and_ret(f: FieldsNamedStruct) -> AUnitEnum
    pub fn func_with_const<T: Copy, const N: usize>(t: T) -> [T; N]
    pub fn func_with_no_args()
    pub fn no_synthetic(_: impl Sized)
    pub unsafe extern "C" fn variadic(_: *const (), ...)
    pub unsafe extern "C" fn variadic_multiline(
        _: *const (), 
        _: *mut (), 
        ...
    )
    "###);

    let lines = Vec::from_iter(styled_fn.iter().map(DeclarationLines::from));
    snap!("DeclarationLines-fn-items", DisplaySlice(&lines));
}

#[test]
fn methods() {
    let map = &doc();
    let dmod = map.dmodule();
    // let mut styled_fn = Vec::with_capacity(16);
    let mut fns_str = Vec::with_capacity(16);
    for struct_ in &dmod.structs {
        for inh in &*struct_.impls.merged_inherent.functions {
            fns_str.push(StyledType::new(&inh.id, map));
        }
    }
    for enum_ in &dmod.enums {
        for inh in &*enum_.impls.merged_inherent.functions {
            fns_str.push(StyledType::new(&inh.id, map));
        }
    }
    for union_ in &dmod.unions {
        for inh in &*union_.impls.merged_inherent.functions {
            fns_str.push(StyledType::new(&inh.id, map));
        }
    }
    for trait_ in &dmod.traits {
        for fn_ in &*trait_.functions {
            fns_str.push(StyledType::new(&fn_.id, map));
        }
    }
    shot!(DisplaySlice(&fns_str), @r###"
    pub fn by_rc(self: Rc<Self>)
    pub fn by_ref(&self)
    pub fn by_ref_mut(&mut self)
    pub fn consume(self)
    pub fn new() -> Self
    fn return_assoc(&self) -> Self::Assoc<'_>; 
    "###);

    let lines = Vec::from_iter(fns_str.iter().map(DeclarationLines::from));
    snap!("DeclarationLines-methods", DisplaySlice(&lines));
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
    pub struct Named {
        fut: Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>>>>,
        /* private fields */
    }
    pub struct NamedAllPrivateFields { /* private fields */ }
    pub struct NamedAllPublicFields {
        fut: Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>>>>
    }
    pub struct NamedGeneric<'a, T, const N: usize> {
        f1: &'a T,
        f2: [T; N]
    }
    pub struct NamedGenericAllPrivate<'a, T, const N: usize> { /* private fields */ }
    pub struct NamedGenericWithBound<'a, T = (), const N: usize = 1>
    where
        T: Copy
    {
        f1: &'a [T],
        f2: [T; N]
    }
    pub struct NamedGenericWithBoundAllPrivate<'a, T, const N: usize>
    where
        T: Copy
    { /* private fields */ }
    pub struct Tuple(
        _,
        _,
        FieldsNamedStruct
    );
    pub struct TupleAllPrivate(_, _, _);
    pub struct TupleGeneric<'a, T: 'a, const N: usize>(
        &'a T,
        [T; N]
    );
    pub struct TupleGenericWithBound<'a, T, const N: usize>(
        &'a T,
        _
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

    let lines = Vec::from_iter(structs_str.iter().map(DeclarationLines::from));
    snap!("DeclarationLines-structs", DisplaySlice(&lines));
}

fn recursive_struct_str(dmod: &DModule, structs_str: &mut Vec<StyledType>, map: &IDMap) {
    for struct_ in &dmod.structs {
        structs_str.push(StyledType::new(&struct_.id, map));
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

impl<'s, T: std::fmt::Debug> std::fmt::Debug for DisplaySlice<'s, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ele in self.0 {
            _ = writeln!(f, "{ele:?}");
        }
        Ok(())
    }
}
