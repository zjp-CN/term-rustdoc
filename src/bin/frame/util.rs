use ratatui::{
    layout::Flex,
    prelude::{Constraint, Layout, Rect},
};

pub fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let horizontal = Layout::horizontal([Constraint::Percentage(width)]).flex(Flex::Center);
    let vertical = Layout::vertical([Constraint::Percentage(height)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
