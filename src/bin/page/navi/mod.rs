use crate::ui::{scrollable::ScrollHeading, Surround};

#[derive(Default, Debug)]
pub struct Navigation {
    pub display: ScrollHeading,
    pub border: Surround,
}
