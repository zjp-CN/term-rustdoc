use super::{
    path::{FindName, Format, Short},
    Punctuation, StyledType, Vis,
};
use crate::tree::IDMap;
use rustdoc_types::{Function, Generics, ItemEnum, Visibility};

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
            // ItemEnum::Struct(s) => s.parse(vis, fname, map),
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

impl Declaration for Function {
    fn format<K: FindName>(&self, map: VisNameMap, buf: &mut StyledType) {
        map.vis.format::<K>(buf);
        let Function {
            decl,
            generics,
            header,
            has_body,
        } = self;
        header.format::<K>(buf);
        buf.write(Punctuation::WhiteSpace);
        buf.write(map.name);
        let Generics {
            params,
            where_predicates,
        } = generics;
        if !params.is_empty() {
            params.format::<K>(buf);
        }
        decl.format::<K>(buf);
        where_predicates.format::<K>(buf);
        if !*has_body {
            // if a function has no body, it's likely an associated function in trait definition
            buf.write(Punctuation::SemiColon);
        }
    }
}
