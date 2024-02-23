use crate::ui::{MarkdownAndHeading, ScrollMarkdown, Surround};
use ratatui::{
    layout::Alignment,
    prelude::{Buffer, Rect},
    text::Line,
    widgets::{Block, Borders},
};

use super::centered_rect;

pub struct Help {
    md: HelpMarkdown,
    /// full screen area
    full: Rect,
}

impl Help {
    pub fn new(full: Rect) -> Help {
        let outer = split_surround(full);
        Help {
            md: HelpMarkdown::new(outer),
            full,
        }
    }

    pub fn update_area(&mut self, full: Rect) {
        if self.full == full {
            return;
        }
        self.full = full;
        let outer = split_surround(full);
        self.md = HelpMarkdown::new(outer);
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.md.border.render(buf);
        self.md.inner.render(buf);
    }

    pub fn scroll_text(&mut self) -> &mut ScrollMarkdown {
        self.md.inner.scroll_text()
    }

    pub fn heading_jump(&mut self, position: (u16, u16)) -> bool {
        // position: (x, y)
        if self.md.inner.heading().area.contains(position.into()) {
            return self.md.inner.heading_jump(position.1);
        }
        false
    }

    pub fn contains(&self, position: (u16, u16)) -> bool {
        self.md.border.area().contains(position.into())
    }
}

fn split_surround(full: Rect) -> Surround {
    let outer = centered_rect(full, 80, 80);
    let title = Line::from(" Press F1 to toggle this Help ").alignment(Alignment::Right);
    Surround::new(
        Block::new()
            .title(" Help ")
            .title_bottom(title)
            .borders(Borders::ALL),
        outer,
    )
}

struct HelpMarkdown {
    inner: MarkdownAndHeading,
    border: Surround,
}

impl HelpMarkdown {
    fn new(border: Surround) -> Self {
        let inner = MarkdownAndHeading::new(Self::HELP, border.inner());
        HelpMarkdown { inner, border }
    }

    const HELP: &'static str = include_str!("help.md");
}
