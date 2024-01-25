use crate::{shot, snap, INTEGRATION};
use term_rustdoc::tree::{DModule, IDMap, Show};

#[test]
fn parse_module() {
    let doc = &INTEGRATION.doc;
    let parsed = DModule::new(doc);
    snap!("DModule", parsed);
    shot!(parsed.show(), @r###"
    [mod] 0:0:1778
    ├── [mod] 0:10:1776
    │   ├── [mod] 0:11:415
    │   │   ├── Structs
    │   │   ├── Unions
    │   │   ├── Enums
    │   │   ├── Traits
    │   │   ├── Functions
    │   │   ├── Constants
    │   │   ├── Statics
    │   │   ├── Macros - Declarative
    │   │   ├── Macro - Function
    │   │   ├── Macro - Attribute
    │   │   └── Macro - Derive
    │   ├── Structs
    │   ├── Unions
    │   ├── Enums
    │   ├── Traits
    │   ├── Functions
    │   ├── Constants
    │   ├── Statics
    │   ├── Macros - Declarative
    │   ├── Macro - Function
    │   ├── Macro - Attribute
    │   └── Macro - Derive
    ├── Structs
    │   └── [struct] 0:3:1774
    │       ├── Fields
    │       └── Implementations
    │           ├── Inherent Impls
    │           ├── Trait Impls
    │           │   └── 0:6
    │           ├── Auto Impls
    │           │   ├── a:2:2957:254-0:3:1774
    │           │   ├── a:2:42204:2420-0:3:1774
    │           │   ├── a:2:32574:2039-0:3:1774
    │           │   ├── a:2:32492:244-0:3:1774
    │           │   └── a:2:42205:2749-0:3:1774
    │           └── Blanket Impls
    │               ├── b:2:2745-0:3:1774
    │               ├── b:2:3504-0:3:1774
    │               ├── b:2:2435-0:3:1774
    │               ├── b:2:2750-0:3:1774
    │               ├── b:2:2739-0:3:1774
    │               ├── b:2:2735-0:3:1774
    │               └── b:2:2432-0:3:1774
    ├── Unions
    ├── Enums
    ├── Traits
    │   └── [trait] 0:5:260
    │       ├── Associated Types
    │       ├── Associated Constants
    │       ├── Associated Functions
    │       └── Implementors
    │           └── 0:6
    ├── Functions
    ├── Constants
    ├── Statics
    ├── Macros - Declarative
    ├── Macro - Function
    ├── Macro - Attribute
    └── Macro - Derive
    "###);
    shot!(parsed.show_prettier(&IDMap::from_crate(doc)), @r###"
    [mod] integration
    ├── [mod] integration::a
    │   └── [mod] integration::a::c
    ├── Structs
    │   └── [struct] integration::S
    │       ├── No field
    │       └── Implementations
    │           ├── Inherent Impls
    │           ├── Trait Impls
    │           │   └── [trait] 0:6
    │           ├── Auto Impls
    │           │   ├── [auto] a:2:2957:254-0:3:1774
    │           │   ├── [auto] a:2:42204:2420-0:3:1774
    │           │   ├── [auto] a:2:32574:2039-0:3:1774
    │           │   ├── [auto] a:2:32492:244-0:3:1774
    │           │   └── [auto] a:2:42205:2749-0:3:1774
    │           └── Blanket Impls
    │               ├── [blkt] b:2:2745-0:3:1774
    │               ├── [blkt] b:2:3504-0:3:1774
    │               ├── [blkt] b:2:2435-0:3:1774
    │               ├── [blkt] b:2:2750-0:3:1774
    │               ├── [blkt] b:2:2739-0:3:1774
    │               ├── [blkt] b:2:2735-0:3:1774
    │               └── [blkt] b:2:2432-0:3:1774
    └── Traits
        └── [trait] integration::Trait
            ├── Associated Types
            ├── Associated Constants
            ├── Associated Functions
            └── Implementors
                └── 0:6
    "###);

    snap!(parsed.current_items_counts(), @r###"
    ItemCount {
        modules: 1,
        structs: 1,
        traits: 1,
    }
    "###);
    snap!(parsed.recursive_items_counts(), @r###"
    ItemCount {
        modules: 2,
        structs: 1,
        traits: 1,
    }
    "###);

    snap!(parsed.current_impls_counts(), @r###"
    ImplCounts {
        total: ImplCount {
            kind: Both,
            total: 1,
            structs: 1,
        },
        trait: ImplCount {
            kind: Trait,
            total: 1,
            structs: 1,
        },
    }
    "###);
    snap!(parsed.recursive_impls_counts(), @r###"
    ImplCounts {
        total: ImplCount {
            kind: Both,
            total: 1,
            structs: 1,
        },
        trait: ImplCount {
            kind: Trait,
            total: 1,
            structs: 1,
        },
    }
    "###);
}
