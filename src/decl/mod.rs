use rustdoc_types::Visibility;
use std::fmt::Write;

mod fn_;
pub use fn_::fn_item;

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
}
