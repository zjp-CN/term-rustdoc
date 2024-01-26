use crate::{shot, snap, INTEGRATION};
use term_rustdoc::tree::{DModule, IDMap, Show};

#[test]
fn parse_module() {
    let doc = &INTEGRATION.doc;
    let parsed = DModule::new(doc);
    snap!("DModule", parsed);
    shot!(parsed.show(), @r###"
    [mod] 0:0:1800
    ├── [mod] 0:10:1777
    │   ├── [mod] 0:11:1778
    │   │   └── Traits
    │   │       └── [trait] 0:14:1780
    │   │           ├── Associated Types
    │   │           ├── Associated Constants
    │   │           ├── Associated Functions
    │   │           └── Implementors
    │   └── Enums
    │       └── [enum]
    │           ├── Variants
    │           │   ├── 0:34:1782
    │           │   ├── 0:36:1783
    │           │   └── 0:38:106
    │           └── Implementations
    │               ├── Inherent Impls
    │               │   └── 0:15
    │               ├── Trait Impls
    │               │   └── 0:40
    │               ├── Auto Impls
    │               │   ├── a:2:2957:254-0:33:1781
    │               │   ├── a:2:42204:5740-0:33:1781
    │               │   ├── a:2:32574:2059-0:33:1781
    │               │   ├── a:2:32492:244-0:33:1781
    │               │   └── a:2:42205:5922-0:33:1781
    │               └── Blanket Impls
    │                   ├── b:2:2745-0:33:1781
    │                   ├── b:2:3504-0:33:1781
    │                   ├── b:2:2435-0:33:1781
    │                   ├── b:2:2750-0:33:1781
    │                   ├── b:2:2739-0:33:1781
    │                   ├── b:2:2735-0:33:1781
    │                   └── b:2:2432-0:33:1781
    ├── Structs
    │   ├── [struct] 0:3:1774
    │   │   ├── Fields
    │   │   └── Implementations
    │   │       ├── Inherent Impls
    │   │       ├── Trait Impls
    │   │       │   └── 0:6
    │   │       ├── Auto Impls
    │   │       │   ├── a:2:2957:254-0:3:1774
    │   │       │   ├── a:2:42204:5740-0:3:1774
    │   │       │   ├── a:2:32574:2059-0:3:1774
    │   │       │   ├── a:2:32492:244-0:3:1774
    │   │       │   └── a:2:42205:5922-0:3:1774
    │   │       └── Blanket Impls
    │   │           ├── b:2:2745-0:3:1774
    │   │           ├── b:2:3504-0:3:1774
    │   │           ├── b:2:2435-0:3:1774
    │   │           ├── b:2:2750-0:3:1774
    │   │           ├── b:2:2739-0:3:1774
    │   │           ├── b:2:2735-0:3:1774
    │   │           └── b:2:2432-0:3:1774
    │   └── [struct] 0:17:1787
    │       ├── Fields
    │       │   ├── 0:18:1788
    │       │   ├── 0:19:1789
    │       │   ├── 0:20:1790
    │       │   └── /* private fields */
    │       └── Implementations
    │           ├── Inherent Impls
    │           │   └── 0:22
    │           ├── Trait Impls
    │           │   └── 0:25
    │           ├── Auto Impls
    │           │   ├── a:2:2957:254-0:17:1787
    │           │   ├── a:2:42204:5740-0:17:1787
    │           │   ├── a:2:32574:2059-0:17:1787
    │           │   ├── a:2:32492:244-0:17:1787
    │           │   └── a:2:42205:5922-0:17:1787
    │           └── Blanket Impls
    │               ├── b:2:2745-0:17:1787
    │               ├── b:2:3504-0:17:1787
    │               ├── b:2:2435-0:17:1787
    │               ├── b:2:2750-0:17:1787
    │               ├── b:2:2739-0:17:1787
    │               ├── b:2:2735-0:17:1787
    │               └── b:2:2432-0:17:1787
    ├── Traits
    │   └── [trait] 0:5:1775
    │       ├── Associated Types
    │       ├── Associated Constants
    │       ├── Associated Functions
    │       └── Implementors
    │           └── 0:6
    ├── Functions
    │   ├── 0:27:1793
    │   ├── 0:28:1794
    │   └── 0:29:1795
    ├── Constants
    │   ├── 0:30:1796
    │   └── 0:31:1798
    └── Macros - Declarative
        └── 0:32:1799
    "###);
    shot!(parsed.show_prettier(&IDMap::from_crate(doc)), @r###"
    [mod] integration
    ├── [mod] integration::submod1
    │   ├── [mod] integration::submod1::submod2
    │   │   └── Traits
    │   │       └── [trait] integration::submod1::submod2::ATraitNeverImplementedForTypes
    │   │           └── No Associated Items Or Implementors!
    │   └── Enums
    │       └── [enum] integration::submod1::AUnitEnum
    │           ├── Variants
    │           │   ├── [variant] A
    │           │   ├── [variant] B
    │           │   └── [variant] C
    │           └── Implementations
    │               ├── Inherent Impls
    │               │   └── [inhrt] 0:15
    │               ├── Trait Impls
    │               │   └── [trait] AUnitEnum: Debug
    │               ├── Auto Impls
    │               │   ├── [auto] AUnitEnum: Sync
    │               │   ├── [auto] AUnitEnum: UnwindSafe
    │               │   ├── [auto] AUnitEnum: Unpin
    │               │   ├── [auto] AUnitEnum: Send
    │               │   └── [auto] AUnitEnum: RefUnwindSafe
    │               └── Blanket Impls
    │                   ├── [blkt] T: TryInto<U>
    │                   ├── [blkt] T: Any
    │                   ├── [blkt] T: BorrowMut<T>
    │                   ├── [blkt] T: TryFrom<U>
    │                   ├── [blkt] T: From<T>
    │                   ├── [blkt] T: Into<U>
    │                   └── [blkt] T: Borrow<T>
    ├── Structs
    │   ├── [struct] integration::AUnitStruct
    │   │   ├── No fields!
    │   │   └── Implementations
    │   │       ├── Trait Impls
    │   │       │   └── [trait] AUnitStruct: ATrait
    │   │       ├── Auto Impls
    │   │       │   ├── [auto] AUnitStruct: Sync
    │   │       │   ├── [auto] AUnitStruct: UnwindSafe
    │   │       │   ├── [auto] AUnitStruct: Unpin
    │   │       │   ├── [auto] AUnitStruct: Send
    │   │       │   └── [auto] AUnitStruct: RefUnwindSafe
    │   │       └── Blanket Impls
    │   │           ├── [blkt] T: TryInto<U>
    │   │           ├── [blkt] T: Any
    │   │           ├── [blkt] T: BorrowMut<T>
    │   │           ├── [blkt] T: TryFrom<U>
    │   │           ├── [blkt] T: From<T>
    │   │           ├── [blkt] T: Into<U>
    │   │           └── [blkt] T: Borrow<T>
    │   └── [struct] integration::FieldsNamedStruct
    │       ├── 3 fields
    │       │   ├── [field] field1
    │       │   ├── [field] field2
    │       │   └── [field] field3
    │       ├── /* private fields */
    │       └── Implementations
    │           ├── Inherent Impls
    │           │   └── [inhrt] 0:22
    │           ├── Trait Impls
    │           │   └── [trait] FieldsNamedStruct: Default
    │           ├── Auto Impls
    │           │   ├── [auto] FieldsNamedStruct: Sync
    │           │   ├── [auto] FieldsNamedStruct: UnwindSafe
    │           │   ├── [auto] FieldsNamedStruct: Unpin
    │           │   ├── [auto] FieldsNamedStruct: Send
    │           │   └── [auto] FieldsNamedStruct: RefUnwindSafe
    │           └── Blanket Impls
    │               ├── [blkt] T: TryInto<U>
    │               ├── [blkt] T: Any
    │               ├── [blkt] T: BorrowMut<T>
    │               ├── [blkt] T: TryFrom<U>
    │               ├── [blkt] T: From<T>
    │               ├── [blkt] T: Into<U>
    │               └── [blkt] T: Borrow<T>
    ├── Traits
    │   └── [trait] integration::ATrait
    │       └── Implementors
    │           └──  AUnitStruct: ATrait
    ├── Functions
    │   ├── [fn] func_with_no_args
    │   ├── [fn] func_with_1arg
    │   └── [fn] func_with_1arg_and_ret
    ├── Constants
    │   ├── [constant] ACONSTANT
    │   └── [constant] ASTATIC
    └── Macros - Declarative
        └── [macro decl] a_decl_macro
    "###);

    snap!(parsed.current_items_counts(), @r###"
    ItemCount {
        modules: 1,
        structs: 2,
        functions: 3,
        traits: 1,
        constants: 2,
        macros_decl: 1,
    }
    "###);
    snap!(parsed.recursive_items_counts(), @r###"
    ItemCount {
        modules: 2,
        structs: 2,
        enums: 1,
        functions: 3,
        traits: 1,
        constants: 2,
        macros_decl: 1,
    }
    "###);

    snap!(parsed.current_impls_counts(), @r###"
    ImplCounts {
        total: ImplCount {
            kind: Both,
            total: 3,
            structs: 3,
        },
        inherent: ImplCount {
            kind: Inherent,
            total: 1,
            structs: 1,
        },
        trait: ImplCount {
            kind: Trait,
            total: 2,
            structs: 2,
        },
    }
    "###);
    snap!(parsed.recursive_impls_counts(), @r###"
    ImplCounts {
        total: ImplCount {
            kind: Both,
            total: 5,
            structs: 3,
            enums: 2,
        },
        inherent: ImplCount {
            kind: Both,
            total: 2,
            structs: 1,
            enums: 1,
        },
        trait: ImplCount {
            kind: Trait,
            total: 3,
            structs: 2,
            enums: 1,
        },
    }
    "###);
}
