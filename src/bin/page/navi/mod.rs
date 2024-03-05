use crate::ui::{scrollable::ScrollHeading, Surround};
use ratatui::layout::Position;

#[derive(Default, Debug)]
pub struct Navigation {
    display: ScrollHeading,
    border: Surround,
}

impl Navigation {
    pub fn heading(&mut self) -> &mut ScrollHeading {
        &mut self.display
    }

    // position in (x, y)
    pub fn contains(&self, position: Position) -> bool {
        self.border.area().contains(position)
    }

    pub fn border(&mut self) -> &mut Surround {
        &mut self.border
    }
}
