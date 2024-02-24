// Example: for `pub use crate::AUnitStruct`
//
// Item {
//   id: Id("0:12-0:3:1774"), crate_id: 0, name: None,
//   span: Some(Span {..}),
//   visibility: Public, docs: None, links: {}, attrs: [], deprecation: None,
//   inner: Import(Import {
//      source: "crate::AUnitStruct",
//      name: "AUnitStruct",
//      id: Some(Id("0:3:1774")),
//      glob: false
//   })
// }

use super::*;
use crate::tree::ID;
use rustdoc_types::{Import, ItemEnum};

/// Add the item of `pub use source {as name}` to DModule.
///
/// ## Note
///
/// pub-use item shouldn't be a real tree node because the source
/// can be any other item which should be merged into one of DModule's fields.
pub(super) fn parse_import(
    id: ID,
    import: &Import,
    map: &IDMap,
    dmod: &mut DModule,
    kin: &mut Vec<ID>,
) {
    // Import's id can be empty when the source is Primitive.
    if let Some(source) = import.id.as_ref().and_then(|id| map.indexmap().get(id)) {
        match &source.inner {
            ItemEnum::Module(item) => {
                // check id for ItemSummary/path existence:
                // reexported modules are not like other normal items,
                // they can recursively points to each other, causing stack overflows.
                // RUST_LOG=term_rustdoc::tree::nodes=debug can be used to quickly check the logs.
                match map.path_or_name(&id) {
                    Ok(path) => {
                        // usual reexported module
                        debug!("Push down the reexported `{path}` module.");
                        dmod.modules
                            .push(DModule::new_inner(id, &item.items, map, kin))
                    }
                    Err(name) if !kin.contains(&id) => {
                        // Unusual reexported module: copy the items inside until a duplicate
                        // of ancestor module (if any).
                        debug!("Push down the reexported `{name}` module that is not found in PathMap.");
                        dmod.modules
                            .push(DModule::new_inner(id, &item.items, map, &mut kin.clone()))
                    }
                    Err(name) => warn!(
                        "Stop at the reexported `{name}` module that duplicates as an ancestor module.\n\
                         Ancestors before this module is {:?}",
                        kin.iter().map(|id| map.path(id)).collect::<Vec<_>>()
                    ),
                }
            }
            ItemEnum::Union(item) => dmod.unions.push(DUnion::new(id, item, map)),
            ItemEnum::Struct(item) => dmod.structs.push(DStruct::new(id, item, map)),
            ItemEnum::Enum(item) => dmod.enums.push(DEnum::new(id, item, map)),
            ItemEnum::Trait(item) => dmod.traits.push(DTrait::new(id, item, map)),
            ItemEnum::Function(_) => dmod.functions.push(DFunction::new(id)),
            ItemEnum::TypeAlias(_) => dmod.type_alias.push(DTypeAlias::new(id)),
            ItemEnum::Constant(_) => dmod.constants.push(DConstant::new(id)),
            ItemEnum::Static(_) => dmod.statics.push(DStatic::new(id)),
            ItemEnum::Macro(_) => dmod.macros_decl.push(DMacroDecl::new(id)),
            ItemEnum::ProcMacro(proc) => match proc.kind {
                MacroKind::Bang => dmod.macros_func.push(DMacroFunc::new(id)),
                MacroKind::Attr => dmod.macros_attr.push(DMacroAttr::new(id)),
                MacroKind::Derive => dmod.macros_derv.push(DMacroDerv::new(id)),
            },
            _ => (),
        }
    }
}
