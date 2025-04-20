use super::*;
use crate::tree::ID;
use rustdoc_types::{ItemEnum, ItemKind, Use};

/// Add the item of `pub use source {as name}` to DModule.
///
/// ## Note
///
/// pub-use item shouldn't be a real tree node because the source
/// can be any other item which should be merged into one of DModule's fields.
pub(super) fn parse_import(
    id: ID,
    import: &Use,
    map: &IDMap,
    dmod: &mut DModule,
    kin: &mut Vec<ID>,
) {
    let Some(import_id) = &import.id else { return };
    // Import's id can be empty when the source is Primitive.
    if let Some(source) = map.indexmap().get(import_id) {
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
            ItemEnum::Constant { .. } => dmod.constants.push(DConstant::new(id)),
            ItemEnum::Static(_) => dmod.statics.push(DStatic::new(id)),
            ItemEnum::Macro(_) => dmod.macros_decl.push(DMacroDecl::new(id)),
            ItemEnum::ProcMacro(proc) => match proc.kind {
                MacroKind::Bang => dmod.macros_func.push(DMacroFunc::new(id)),
                MacroKind::Attr => dmod.macros_attr.push(DMacroAttr::new(id)),
                MacroKind::Derive => dmod.macros_derv.push(DMacroDerv::new(id)),
            },
            _ => (),
        }
    } else if let Some(extern_item) = map.pathmap().get(import_id) {
        let id = import_id.to_ID();
        // TODO: external items are in path map, which means no further information
        // except full path and item kind will be known.
        // To get details of an external item, we need to compile the external crate,
        // and search it with the full path and kind.
        // A simple example of this is `nucleo` crate.
        match extern_item.kind {
            ItemKind::Module if !kin.contains(&id) => {
                // We don't know items inside external modules.
                debug!(
                    "Push down the reexported external `{}` without inner items.",
                    extern_item.path.join("::")
                );
                dmod.modules.push(DModule::new_external(id))
            }
            ItemKind::Struct => dmod.structs.push(DStruct::new_external(id)),
            ItemKind::Union => dmod.unions.push(DUnion::new_external(id)),
            ItemKind::Enum => dmod.enums.push(DEnum::new_external(id)),
            ItemKind::Function => dmod.functions.push(DFunction::new(id)),
            ItemKind::TypeAlias => dmod.type_alias.push(DTypeAlias::new(id)),
            ItemKind::Constant => dmod.constants.push(DConstant::new(id)),
            ItemKind::Trait => dmod.traits.push(DTrait::new_external(id)),
            ItemKind::Static => dmod.statics.push(DStatic::new(id)),
            ItemKind::Macro => dmod.macros_decl.push(DMacroDecl::new(id)),
            ItemKind::ProcAttribute => dmod.macros_attr.push(DMacroAttr::new(id)),
            ItemKind::ProcDerive => dmod.macros_derv.push(DMacroDerv::new(id)),
            ItemKind::Primitive => (),
            _ => (),
        }
    }
}
