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
use crate::tree::{IndexMap, ID};
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
    index: &IDMap,
    dmod: &mut DModule,
    level: &mut u16,
) {
    let name = index.name(&id);
    // Import's id can be empty when the source is Primitive.
    if let Some(source) = import.id.as_ref().and_then(|id| index.indexmap().get(id)) {
        match &source.inner {
            ItemEnum::Module(item) => {
                info!("Reexport Module => {name}, level = {level}");
                let Some(name) = index.get_path(&id).map(|item| item.path.join("::")) else {
                    error!("Found an unusual item. Check out the JSON doc with the ID {id}");
                    return;
                };
                dmod.modules
                    .push(DModule::new_inner(id, &item.items, index, level))
            }
            ItemEnum::Union(item) => {
                info!("Reexport Union => {name}");
                dmod.unions.push(DUnion::new(id, item, index))
            }
            ItemEnum::Struct(item) => {
                info!("Reexport Struct => {name}");
                dmod.structs.push(DStruct::new(id, item, index))
            }
            ItemEnum::Enum(item) => {
                info!("Reexport Enum => {name}");
                dmod.enums.push(DEnum::new(id, item, index))
            }
            ItemEnum::Trait(item) => {
                info!("Reexport Trait => {name}");
                dmod.traits.push(DTrait::new(id, item, index))
            }
            ItemEnum::Function(_) => {
                info!("Reexport Function => {name}");
                dmod.functions.push(DFunction::new(id))
            }
            ItemEnum::TypeAlias(_) => {
                info!("Reexport TypeAlias => {name}");
                dmod.type_alias.push(DTypeAlias::new(id))
            }
            ItemEnum::Constant(_) => {
                info!("Reexport Constant => {name}");
                dmod.constants.push(DConstant::new(id))
            }
            ItemEnum::Static(_) => {
                info!("Reexport Static => {name}");
                dmod.statics.push(DStatic::new(id))
            }
            ItemEnum::Macro(_) => {
                info!("Reexport Macro => {name}");
                dmod.macros_decl.push(DMacroDecl::new(id))
            }
            ItemEnum::ProcMacro(proc) => {
                info!("Reexport ProcMacro => {name}");
                match proc.kind {
                    MacroKind::Bang => dmod.macros_func.push(DMacroFunc::new(id)),
                    MacroKind::Attr => dmod.macros_attr.push(DMacroAttr::new(id)),
                    MacroKind::Derive => dmod.macros_derv.push(DMacroDerv::new(id)),
                }
            }
            x => info!("Reexport res => {x:?}"),
        }
    }
}
