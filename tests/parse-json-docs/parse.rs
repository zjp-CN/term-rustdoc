use crate::{shot, snap, INTEGRATION};
use similar_asserts::assert_eq;
use term_rustdoc::tree::{DModule, IDMap, Show, TreeLines};

#[test]
fn parse_module() {
    let doc = &INTEGRATION.doc;
    let parsed = DModule::new(doc);
    snap!("DModule", parsed);
    shot!("show-id", parsed.show());

    let tree = parsed.show_prettier(&IDMap::from_crate(doc));
    let display = tree.to_string();
    shot!("show-prettier", display);

    let (flatten_tree, empty) = TreeLines::new(tree);
    let display_new = flatten_tree.display_as_plain_text();
    assert_eq!(expected: display, actual: display_new);
    snap!("flatten-tree", flatten_tree.lines());
    shot!("empty-tree-with-same-depth", empty);

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
