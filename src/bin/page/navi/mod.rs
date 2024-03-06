#![allow(dead_code)]

mod outline;

use self::outline::NaviOutline;
use crate::ui::{
    scrollable::{ScrollHeading, ScrollText},
    Surround,
};
use ratatui::{
    layout::Position,
    prelude::{Buffer, Constraint, Layout, Rect},
};
use term_rustdoc::tree::{CrateDoc, ID};

#[derive(Default, Debug)]
pub struct Navigation {
    display: Navi,
    border: Surround,
}

impl Navigation {
    pub fn heading(&mut self) -> &mut ScrollHeading {
        &mut self.display.heading
    }

    // position in (x, y)
    pub fn contains(&self, position: Position) -> bool {
        self.border.area().contains(position)
    }

    pub fn border(&mut self) -> &mut Surround {
        &mut self.border
    }

    pub fn set_item_inner(&mut self, id: Option<&str>, doc: &CrateDoc) -> Option<ID> {
        self.display.outline.set_item_inner(id, doc)
    }

    pub fn update_area(&mut self, border: Surround) {
        let inner = border.inner();
        let [heading, outline] = split(inner);
        self.display.heading.area = heading;
        self.border = border;
        self.display.outline.update_area(outline);
    }

    pub fn render(&self, buf: &mut Buffer, content: &ScrollText) {
        self.border.render(buf);

        let content_start = content.start;
        let content_end = content.area.height as usize + content_start;
        self.display.heading.render(buf, content_start, content_end);

        self.display.outline.render(buf);
    }
}

#[derive(Default)]
struct Navi {
    heading: ScrollHeading,
    outline: NaviOutline,
}

impl std::fmt::Debug for Navi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Navi {{ ... }}")
    }
}

fn split(area: Rect) -> [Rect; 2] {
    Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)]).areas(area)
}
