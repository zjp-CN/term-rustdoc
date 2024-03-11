use crate::{doc, shot, snap};
use similar_asserts::assert_eq;
use term_rustdoc::tree::{Show, TreeLines};

#[test]
fn parse_module() {
    let (treelines, empty) = TreeLines::new_with(doc(), |doc| doc.dmodule_show_prettier());
    let doc = treelines.doc();
    let dmod = doc.dmodule();
    snap!("DModule", dmod);
    shot!("show-id", dmod.show());

    let tree = doc.dmodule_show_prettier();
    let display = tree.to_string();
    shot!("show-prettier", display);

    let display_new = treelines.display_as_plain_text();
    assert_eq!(expected: display, actual: display_new);
    snap!("flatten-tree", treelines.all_lines());
    shot!("empty-tree-with-same-depth", empty);

    // item tree
    shot!("item-tree", dmod.item_tree(&doc));

    snap!(dmod.current_items_counts(), @r###"
    ItemCount {
        modules: 1,
        structs: 2,
        functions: 3,
        traits: 1,
        constants: 2,
        macros_decl: 1,
    }
    "###);
    snap!(dmod.recursive_items_counts(), @r###"
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

    snap!(dmod.current_impls_counts(), @r###"
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
    snap!(dmod.recursive_impls_counts(), @r###"
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

    // struct inner
    let (struct_, _) = TreeLines::new_with(treelines.doc(), |doc| {
        doc.dmodule().structs[0].show_prettier(doc)
    });
    shot!(struct_.display_as_plain_text(), @r###"
    integration::AUnitStruct
    ├── No Fields!
    └── Implementations
        ├── Trait Impls
        │   └── AUnitStruct: ATrait
        ├── Auto Impls
        │   ├── AUnitStruct: RefUnwindSafe
        │   ├── AUnitStruct: Send
        │   ├── AUnitStruct: Sync
        │   ├── AUnitStruct: Unpin
        │   └── AUnitStruct: UnwindSafe
        └── Blanket Impls
            ├── T: Any
            ├── T: Borrow<T>
            ├── T: BorrowMut<T>
            ├── T: From<T>
            ├── T: Into<U>
            ├── T: TryFrom<U>
            └── T: TryInto<U>
    "###);
}
