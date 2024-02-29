mod parse_cargo_toml;
mod ui;

pub use self::ui::FeaturesUI;

use serde::{Deserialize, Serialize};
use term_rustdoc::util::XString;

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[allow(dead_code)]
pub enum Features {
    #[default]
    Default,
    All,
    DefaultPlus(Box<[XString]>),
    NoDefault,
    NoDefaultPlus(Box<[XString]>),
}
