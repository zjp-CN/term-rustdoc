use super::{generic::generic_args, Long, Short};
use crate::util::{xformat, XString};
use rustdoc_types::Path;

/// Show full names in path.
///
/// Not guaranteed to always be an absolute path for any Path.
pub fn long_path(p: &Path) -> XString {
    let name = p.name.as_str();
    p.args
        .as_deref()
        .and_then(generic_args::<Long>)
        .map_or_else(|| name.into(), |arg| xformat!("{name}{arg}"))
}

/// Only show the last name in path.
pub fn short_path(p: &Path) -> XString {
    fn short_name(name: &str) -> &str {
        &name[name.rfind(':').map_or(0, |x| x + 1)..]
    }
    let name = short_name(&p.name);
    p.args
        .as_deref()
        .and_then(generic_args::<Short>)
        .map_or_else(|| name.into(), |arg| xformat!("{name}{arg}"))
}
