use crate::tree::IDMap;
use rustdoc_types::{ItemEnum, Visibility};
use std::fmt::Write;

mod fn_;
pub use fn_::fn_item;

mod struct_;

fn vis(v: &Visibility, buf: &mut String) {
    match v {
        Visibility::Public => buf.push_str("pub "),
        Visibility::Default => (),
        Visibility::Crate => buf.push_str("pub(crate) "),
        Visibility::Restricted { path, .. } => _ = write!(buf, "pub({path}) "),
    };
}

/// Parse Item as String.
trait Parse {
    fn parse(&self, v: &Visibility, fname: &str) -> String;
    fn buf(v: &Visibility) -> String {
        let mut buf = String::with_capacity(128);
        vis(v, &mut buf);
        buf
    }
    fn item(item: &ItemEnum) -> Option<&Self>;
    fn item_str(id: &str, map: &IDMap) -> String {
        if let Some(item) = map.get_item(id) {
            if let Some(inner) = Self::item(&item.inner) {
                let fname = item.name.as_deref().unwrap_or("");
                return inner.parse(&item.visibility, fname);
            }
        }
        String::new()
    }
}

pub fn item_str(id: &str, map: &IDMap) -> String {
    if let Some(item) = map.get_item(id) {
        let fname = item.name.as_deref().unwrap_or("");
        let vis = &item.visibility;
        return match &item.inner {
            ItemEnum::Import(reexport) => reexport
                .id
                .as_ref()
                .map(|id| item_str(&id.0, map))
                .unwrap_or_default(),
            ItemEnum::Function(f) => f.parse(vis, fname),
            ItemEnum::Struct(s) => s.parse(vis, fname),
            _ => String::new(),
        };
    }
    String::new()
}
