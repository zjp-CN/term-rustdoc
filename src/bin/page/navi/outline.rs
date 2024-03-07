#![allow(clippy::redundant_static_lifetimes)]
use crate::{
    color::{BG_CURSOR_LINE, FG_CURSOR_LINE, NEW},
    ui::{render_line, LineState, Scroll, Surround},
};
use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{Block, BorderType, Borders},
};
use rustdoc_types::ItemEnum;
use term_rustdoc::tree::{CrateDoc, ID};

#[derive(Default)]
struct NaviOutlineInner {
    /// Selected item that has inner data of a kind like fields/variants/impls.
    selected: Option<Selected>,
    lines: &'static [NaviAction],
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

#[derive(Default)]
pub struct NaviOutline {
    display: Scroll<NaviOutlineInner>,
    border: Surround,
}

struct Selected {
    id: ID,
    kind: Kind,
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Struct,
    Enum,
    Trait,
    Union,
}

impl Kind {
    fn new(id: &str, doc: &CrateDoc) -> Option<Kind> {
        doc.get_item(id).and_then(|item| {
            Some(match &item.inner {
                ItemEnum::Struct(_) => Kind::Struct,
                ItemEnum::Enum(_) => Kind::Enum,
                ItemEnum::Trait(_) => Kind::Trait,
                ItemEnum::Union(_) => Kind::Union,
                ItemEnum::Import(reexport) => {
                    let id = reexport.id.as_ref()?;
                    match &doc.get_item(&id.0)?.inner {
                        ItemEnum::Struct(_) => Kind::Struct,
                        ItemEnum::Enum(_) => Kind::Enum,
                        ItemEnum::Trait(_) => Kind::Trait,
                        ItemEnum::Union(_) => Kind::Union,
                        _ => return None,
                    }
                }
                _ => return None,
            })
        })
    }

    fn lines(self) -> &'static [NaviAction] {
        match self {
            Kind::Struct | Kind::Union => STRUCT,
            Kind::Enum => ENUM,
            Kind::Trait => TRAIT,
        }
    }
}

const STRUCT: &'static [NaviAction] = &[
    NaviAction::StructInner,
    NaviAction::ITABImpls,
    NaviAction::BackToHome,
];
const ENUM: &'static [NaviAction] = &[
    NaviAction::EnumInner,
    NaviAction::ITABImpls,
    NaviAction::BackToHome,
];
const TRAIT: &'static [NaviAction] = &[NaviAction::TraitInner, NaviAction::BackToHome];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NaviAction {
    StructInner,
    EnumInner,
    TraitInner,
    ITABImpls,
    #[default]
    BackToHome,
}

impl NaviAction {
    fn text(self) -> &'static str {
        match self {
            NaviAction::StructInner => "Fields",
            NaviAction::EnumInner => "Varaints",
            NaviAction::TraitInner => "Implementors",
            NaviAction::ITABImpls => "Impls",
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
    pub fn set_item_inner(&mut self, id: Option<&str>, doc: &CrateDoc) -> Option<ID> {
        let inner = &mut self.display.lines;
        inner.selected = id.and_then(|id| {
            Kind::new(id, doc).map(|kind| Selected {
                id: id.into(),
                kind,
            })
        });

        let ret = if let Some(selected) = &inner.selected {
            *self.border.block_mut() = block();
            inner.lines = selected.kind.lines();
            Some(selected.id.clone())
        } else {
            *self.border.block_mut() = Default::default();
            None
        };
        // border changes, thus inner area should change
        self.display.area = self.border.inner();

        ret
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
}

fn block() -> Block<'static> {
    Block::new()
        .borders(Borders::TOP)
        .border_type(BorderType::Thick)
}
