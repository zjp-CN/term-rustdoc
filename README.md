
* [data access policy on crates.io ](https://crates.io/data-access)
  * <index.crates.io> can be accessed without rate limits to query crates' history versions, features and dependencies
* local registry cache:
  * `~/.cargo/registry/src/` contains the official crates.io registry and other mirror/custom registries
  * `~/.cargo/registry/index/` contains the API URLs to query or download crates from these registries

## id rules

Mainly steal from [`id_from_item_inner`](https://doc.rust-lang.org/nightly/nightly-rustc/rustdoc/json/conversions/fn.id_from_item_inner.html)

`[IMPL:]CRATE_ID:ITEM_ID[:NAME_ID][-EXTRA]`:
* `impl`
  * `a:` for auto impls
  * `b:` for blanket impls
  * empty if others, like non-impls, inherent impls, normal trait impls
* `name` is the item's name if available (it's not for impl blocks for example).
* `extra` is used for reexports: it contains the ID of the reexported item. It is used to allow
  to have items with the same name but different types to both appear in the generated JSON.

