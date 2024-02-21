# Roadmap

- [x] item outline
  - [x] expand / fold (based on module tree)
    - [x] expand zero level items (i.e. items in root module with sub modules folded)
    - [x] expand to first level items
    - [x] focus on the latest module only (but with all other level modules folded)
    - [x] expand all public items
  - [ ] features related
- [x] doc content
  - [x] text wrapping
  - [x] syntax highlighting in codeblocks
  - [x] recognize rustdoc syntax attributes on codeblocks
    - [x] in links
    - [x] in codeblock (default to rust, hide lines, etc)
- [ ] navigation
  - [x] markdown outline
  - [ ] item's associated items/fields outline
- [ ] package source / DashBoard Popup
  - [x] local
    - [x] local registry src dirs
      - [x] fuzzing search
      - [x] select pkgs to compile docs and cache the artifacts in local db files
    - [x] caches in database (json docs that have been generated will be cached in local db)
      - [x] cache raw JSON output and compress it via xz
      - [x] cache parsed output for faster loading and compress it via xz
      - [x] Sorting the cache list for all items or in groups
    - [ ] local paths to Cargo.toml: low priority
  - [ ] non-local (i.e. download pkgs from the web): low priority
- [ ] configuration
  - [ ] theme: low priority
  - [ ] keybind: low priority
- [ ] fuzzing search
  - [ ] by item name
  - [ ] by all documentation contents
  - [ ] by function/method signature
    - [ ] on concrete types
    - [ ] on generic types
    - [ ] on trait bounds
  - [ ] by crate features
- [ ] generic types enhancement
  - [ ] generic type parameters
    - [ ] list concrete candidate types that meet the trait bounds
      - from within the current pkg
      - from within the caches in database
    - [ ] list the functions/methods that
      - [ ] return generic types that hold the same trait bounds
      - [ ] return concrete candidate types
    - [ ] list the function/methods that
      - [ ] accept generic types that hold the same trait bounds
      - [ ] accept concrete candidate types
  - [ ] lifetime parameters
    - [ ] variance (lack of this info in json docs, but maybe not hard to have it)
- [ ] concrete types
  - [ ] list methods in which the concrete `Type` and its ownership variants `&Type` / `&mut Type` is 
    - [ ] receiver type
    - [ ] argument type
    - [ ] return type
- [ ] traits
  - [ ] classify trait implementors
    - [ ] by ownership (`impl Trait` for `Type` vs `&mut Type` vs `&Type` vs `Box<Type>`)
    - [ ] by concrete vs generic 

# Misc/Basics

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

