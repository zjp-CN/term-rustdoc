use super::{Page, Panel, Surround};
use crate::color::SET;
use ratatui::{
    prelude::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders},
};

impl Page {
    fn layout(&self) -> Layout {
        let outline_width = self.outline.display.max_width + 1;
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(outline_width),
                Constraint::Min(0),
                Constraint::Percentage(10),
            ])
    }

    /// This is called in Widget's render method because inner widgets don't implement
    /// Widget, since the areas they draw are updated only from here, not from Widget trait.
    pub(super) fn update_area(&mut self, full: Rect) {
        // skip updating since the size is the same
        if self.area == full {
            return;
        }

        self.update_area_inner(full);
    }

    /// Force update Page inner layout.
    ///
    /// `full` usually should be the full screen area or Page area.
    pub(super) fn update_area_inner(&mut self, full: Rect) {
        // layout
        self.area = full;
        let layout = self.layout().split(full);

        // border
        let outline_border = Block::new()
            .borders(Borders::RIGHT)
            .border_type(BorderType::Thick);
        self.outline.border = Surround::new(
            if matches!(self.current, None | Some(Panel::Outline)) {
                outline_border.style(SET)
            } else {
                outline_border
            },
            layout[0],
        );
        let outline_area = self.outline.border.inner();
        self.content.border = Surround::new(Block::new(), layout[1]);
        let content_area = self.content.border.inner();

        // display.area
        self.outline.display.area = outline_area;
        // self.outline.display.cursor = outline_area.y;
        let outline_max_width = self.outline.display.max_width;
        if outline_area.width < outline_max_width {
            warn!(
                outline_area.width,
                outline_max_width,
                "Outline width exceeds the area width, so lines may be truncated."
            );
        }

        self.content.display.area = content_area;
        self.content.display.max_width = content_area.width;

        if let Some(&navi_outer_area) = layout.get(2) {
            *self.navi.border() = Surround::new(
                Block::new()
                    .borders(Borders::LEFT)
                    .border_type(BorderType::Thick),
                navi_outer_area,
            );
            self.navi.heading().area = self.navi.border().inner();
        }

        // auto update content when screen size changes
        self.update_content();
    }
}
