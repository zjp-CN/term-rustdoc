#![allow(clippy::redundant_static_lifetimes)]
use crate::{
    color::{BG_CURSOR_LINE, FG_CURSOR_LINE, NEW},
    ui::{render_line, LineState, Scroll, Surround},
};
use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{Block, BorderType, Borders},
};
use term_rustdoc::tree::{DataItemKind as Kind, IDMap, ID};

struct NaviOutlineInner {
    /// Selected item that has inner data of a kind like fields/variants/impls.
    selected: Option<Selected>,
    lines: &'static [NaviAction],
}

impl Default for NaviOutlineInner {
    fn default() -> Self {
        NaviOutlineInner {
            selected: None,
            lines: &[NaviAction::Item, NaviAction::BackToHome],
        }
    }
}

impl std::ops::Deref for NaviOutlineInner {
    type Target = [NaviAction];

    fn deref(&self) -> &Self::Target {
        self.lines
    }
}

impl LineState for NaviAction {
    type State = NaviAction;

    fn state(&self) -> Self::State {
        *self
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        *self == *state
    }
}

pub struct NaviOutline {
    display: Scroll<NaviOutlineInner>,
    border: Surround,
}

impl Default for NaviOutline {
    fn default() -> Self {
        Self {
            display: Default::default(),
            border: Surround::new(block(), Default::default()),
        }
    }
}

struct Selected {
    id: ID,
    kind: Kind,
}

fn lines(kind: Kind) -> &'static [NaviAction] {
    match kind {
        Kind::Struct | Kind::Union => STRUCT,
        Kind::Enum => ENUM,
        Kind::Trait => TRAIT,
    }
}

const STRUCT: &'static [NaviAction] = &[
    NaviAction::Item,
    NaviAction::StructInner,
    NaviAction::ITABImpls,
    NaviAction::BackToHome,
];
const ENUM: &'static [NaviAction] = &[
    NaviAction::Item,
    NaviAction::EnumInner,
    NaviAction::ITABImpls,
    NaviAction::BackToHome,
];
const TRAIT: &'static [NaviAction] = &[
    NaviAction::Item,
    NaviAction::TraitAssociated,
    NaviAction::TraitImplementors,
    NaviAction::BackToHome,
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NaviAction {
    StructInner,
    EnumInner,
    TraitAssociated,
    TraitImplementors,
    ITABImpls,
    Item,
    #[default]
    BackToHome,
}

impl NaviAction {
    fn text(self) -> &'static str {
        match self {
            NaviAction::StructInner => "Fields",
            NaviAction::EnumInner => "Varaints",
            NaviAction::TraitAssociated => "Associated",
            NaviAction::TraitImplementors => "Implementors",
            NaviAction::ITABImpls => "Impls",
            NaviAction::Item => "Current Item",
            NaviAction::BackToHome => "Back To Home",
        }
    }

    fn len(self) -> u16 {
        self.text().len() as u16
    }
}

pub fn height() -> u16 {
    [STRUCT.len(), ENUM.len(), TRAIT.len()]
        .into_iter()
        .max()
        .unwrap_or(0) as u16
        + 1u16
}

pub fn width() -> u16 {
    [STRUCT, ENUM, TRAIT]
        .map(|val| val.iter().map(|s| s.len()))
        .into_iter()
        .flatten()
        .max()
        .unwrap_or(0u16)
        + 5u16
}

impl NaviOutline {
    pub fn set_item_inner(&mut self, id: Option<&str>, map: &IDMap) -> Option<ID> {
        let id = id?;
        let selected = Selected {
            kind: Kind::new(id, map)?,
            id: id.into(),
        };

        // self.display.start = 0;
        // self.display.cursor.y = 0;

        let inner = &mut self.display.lines;
        inner.lines = lines(selected.kind);
        let ret = Some(selected.id.clone());
        inner.selected = Some(selected);
        ret
    }

    pub fn reset(&mut self) {
        *self.inner() = Default::default();
        self.set_cursor_back_to_home();
    }

    pub fn set_cursor_back_to_home(&mut self) {
        self.display.move_bottom_cursor();
    }

    pub fn update_area(&mut self, area: Rect) {
        if let Some(inner) = self.border.update_area(area) {
            self.display.area = inner;
        }
    }

    fn inner(&mut self) -> &mut NaviOutlineInner {
        &mut self.display.lines
    }

    fn inner_ref(&self) -> &NaviOutlineInner {
        &self.display.lines
    }

    fn kind(&self) -> Option<Kind> {
        self.inner_ref().selected.as_ref().map(|v| v.kind)
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.border.render(buf);

        if let Some(lines) = self.display.visible_lines() {
            let width = self.display.area.width as usize;
            let Rect { x, mut y, .. } = self.display.area;
            for &line in lines {
                let line = ["👉 ", line.text()].map(|s| (s, NEW));
                render_line(line, buf, x, y, width);
                y += 1;
            }
            self.display.highlight_current_line(buf, |cell| {
                cell.bg = BG_CURSOR_LINE;
                cell.fg = FG_CURSOR_LINE;
            });
        }
    }

    /// Returns true if user clicks on the item to ask for update of outline.
    pub fn update_outline(&mut self, y: u16) -> Option<NaviAction> {
        self.display.force_line_on_screen(y).copied()
    }

    pub fn next_action(&mut self) -> Option<NaviAction> {
        let current = self.display.get_line_of_current_cursor().copied();
        if matches!(current, Some(NaviAction::BackToHome)) {
            self.display.move_top_cursor();
        } else {
            self.display.move_forward_cursor();
        }
        self.display.get_line_of_current_cursor().copied()
    }
}

fn block() -> Block<'static> {
    Block::new()
        .borders(Borders::TOP)
        .border_type(BorderType::Thick)
}
