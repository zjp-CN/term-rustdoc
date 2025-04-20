use super::{navi::navi_outline_width, Page, Panel, Surround};
use crate::color::SET;
use ratatui::{
    prelude::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders},
};

impl Page {
    fn layout(&self) -> Layout {
        let outline_width = self.outline.max_width() + 1;
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                // The width is dynamic, always fits the max_width.
                // User will see variable widths of outline and content.
                Constraint::Length(outline_width),
                Constraint::Min(20),
                // Leave the exact space for NaviOutline.
                // navi_outline_width is not dynamic.
                // If the constraint is flexible by Min(width),
                // we'll see variable widths on both side, which is not good UX.
                Constraint::Length(navi_outline_width()),
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
        let [a_outline, a_content, a_navi] = self.layout().areas(full);

        // outline
        let outline_border = Block::new()
            .borders(Borders::RIGHT)
            .border_type(BorderType::Thick);
        let outline_border = Surround::new(
            if matches!(self.current, None | Some(Panel::Outline)) {
                outline_border.style(SET)
            } else {
                outline_border
            },
            a_outline,
        );
        self.outline.update_area(outline_border);

        // content
        let border = Surround::new(Block::new(), a_content);
        let id = self
            .outline
            .inner
            .display()
            .get_line_of_current_cursor()
            .and_then(|t| t.id);
        self.content.update_area(border, id);

        // navi
        self.navi.update_area(Surround::new(
            Block::new()
                .borders(Borders::LEFT)
                .border_type(BorderType::Thick),
            a_navi,
        ));

        // auto update content when screen size changes
        self.update_content();
    }
}
