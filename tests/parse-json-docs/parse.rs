use crate::{snap, INTEGRATION};
use term_rustdoc::tree::DModule;

#[test]
fn parse_module() {
    let parsed = DModule::new(&INTEGRATION.doc);
    snap!("DModule", parsed);

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
