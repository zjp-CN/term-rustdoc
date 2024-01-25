use crate::{shot, snap, INTEGRATION};
use term_rustdoc::tree::{DModule, IDMap, Show};

#[test]
fn parse_module() {
    let doc = &INTEGRATION.doc;
    let parsed = DModule::new(doc);
    snap!("DModule", parsed);
    shot!(parsed.show(), @r###"
    [mod] 0:0:1779
    ├── [mod] 0:10:1776
    │   └── [mod] 0:11:415
    │       └── Traits
    │           └── [trait] 0:14:1778
    │               ├── Associated Types
    │               ├── Associated Constants
    │               ├── Associated Functions
    │               └── Implementors
    ├── Structs
    │   └── [struct] 0:3:1774
    │       ├── Fields
    │       └── Implementations
    │           ├── Inherent Impls
    │           ├── Trait Impls
    │           │   └── 0:6
    │           ├── Auto Impls
    │           │   ├── a:2:2957:254-0:3:1774
    │           │   ├── a:2:42204:2421-0:3:1774
    │           │   ├── a:2:32574:2040-0:3:1774
    │           │   ├── a:2:32492:244-0:3:1774
    │           │   └── a:2:42205:2750-0:3:1774
    │           └── Blanket Impls
    │               ├── b:2:2745-0:3:1774
    │               ├── b:2:3504-0:3:1774
    │               ├── b:2:2435-0:3:1774
    │               ├── b:2:2750-0:3:1774
    │               ├── b:2:2739-0:3:1774
    │               ├── b:2:2735-0:3:1774
    │               └── b:2:2432-0:3:1774
    └── Traits
        └── [trait] 0:5:260
            ├── Associated Types
            ├── Associated Constants
            ├── Associated Functions
            └── Implementors
                └── 0:6
    "###);
    shot!(parsed.show_prettier(&IDMap::from_crate(doc)), @r###"
    [mod] integration
    ├── [mod] integration::a
    │   └── [mod] integration::a::c
    │       └── Traits
    │           └── [trait] integration::a::c::ATraitNeverImplementedForTypes
    │               └── No Associated Items Or Implementors!
    ├── Structs
    │   └── [struct] integration::S
    │       ├── No field
    │       └── Implementations
    │           ├── Trait Impls
    │           │   └── [trait] S: Trait
    │           ├── Auto Impls
    │           │   ├── [auto] S: Sync
    │           │   ├── [auto] S: UnwindSafe
    │           │   ├── [auto] S: Unpin
    │           │   ├── [auto] S: Send
    │           │   └── [auto] S: RefUnwindSafe
    │           └── Blanket Impls
    │               ├── [blkt] T: TryInto<U>
    │               ├── [blkt] T: Any
    │               ├── [blkt] T: BorrowMut<T>
    │               ├── [blkt] T: TryFrom<U>
    │               ├── [blkt] T: From<T>
    │               ├── [blkt] T: Into<U>
    │               └── [blkt] T: Borrow<T>
    └── Traits
        └── [trait] integration::Trait
            └── Implementors
                └──  S: Trait
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
