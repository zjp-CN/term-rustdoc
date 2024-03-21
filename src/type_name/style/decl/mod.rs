mod function;
mod struct_;

use super::{
    path::{FindName, Format, Short},
    StyledType, Vis,
};
use crate::tree::IDMap;
use rustdoc_types::{ItemEnum, Visibility};

pub fn item_styled(id: &str, map: &IDMap) -> StyledType {
    if let Some(item) = map.get_item(id) {
        let vis_name_map = VisNameMap {
            name: item.name.as_deref().unwrap_or(""),
            vis: &item.visibility,
            map,
        };
        let mut buf = StyledType::with_capacity(48);
        match &item.inner {
            ItemEnum::Import(reexport) => {
                let id = reexport.id.as_ref();
                return id.map(|id| item_styled(&id.0, map)).unwrap_or_default();
            }
            ItemEnum::Function(f) => f.format_as_short(vis_name_map, &mut buf),
            ItemEnum::Struct(s) => s.format_as_short(vis_name_map, &mut buf),
            _ => return StyledType::default(),
        };
        return buf;
    }
    StyledType::default()
}

impl Format for Visibility {
    fn format<Kind: FindName>(&self, buf: &mut StyledType) {
        buf.write(match self {
            Visibility::Public => Vis::Pub,
            Visibility::Default => Vis::Default,
            Visibility::Crate => Vis::PubCrate,
            Visibility::Restricted { parent, path } => {
                buf.write_vis_scope(parent, path);
                return;
            }
        });
    }
}

struct VisNameMap<'a> {
    vis: &'a Visibility,
    name: &'a str,
    map: &'a IDMap,
}

trait Declaration {
    fn format<K: FindName>(&self, map: VisNameMap, buf: &mut StyledType);
    fn format_as_short(&self, map: VisNameMap, buf: &mut StyledType) {
        <Self as Declaration>::format::<Short>(self, map, buf);
    }
}
