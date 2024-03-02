//! A block that shows name, version and features of a selected pkg in toml.
//!
//! Note: if the line is too long, you should move the cursor to see exceeding texts.

use crate::{
    color::{BG_CURSOR, PKG_TOML},
    database::Features,
    ui::{render_line, Surround},
};
use ratatui::{
    prelude::{Alignment, Buffer, Color, Constraint, Layout, Line, Rect},
    widgets::{Block, Borders},
};
use std::fmt::Write;
use unicode_width::UnicodeWidthStr;

#[derive(Default)]
pub struct PkgToml {
    toml: String,
    /// The width on toml string.
    toml_width: u16,
    inner: Rect,
    border: Surround,
}

pub fn surround(area: Rect) -> Surround {
    Surround::new(
        Block::new()
            .title_bottom(Line::from(" Selected Pkg ").alignment(Alignment::Right))
            .borders(Borders::ALL),
        area,
    )
}

/// Returns [Remainings, PkgToml] areas in vertical split.
pub fn split_for_pkg_toml(area: Rect) -> [Rect; 2] {
    Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).areas(area)
}

impl PkgToml {
    pub fn update_toml(&mut self, name: &str, ver: &str, features: &Features) {
        self.toml.clear();
        let buf = &mut self.toml;
        let _ = match features {
            Features::Default => write!(buf, "{name} = {ver:?}"),
            // TODO: currently no way to generate All variant, but if we want to support it,
            // we must check the pkg's all features, because no all-features field here.
            Features::All => write!(buf, "{name} = {{ version = {ver:?} }}"),
            Features::DefaultPlus(feats) => {
                write!(
                    buf,
                    "{name} = {{ version = {ver:?}, features = {feats:?} }}"
                )
            }
            Features::NoDefault => {
                write!(
                    buf,
                    "{name} = {{ version = {ver:?}, default-features = false }}"
                )
            }
            Features::NoDefaultPlus(feats) => {
                write!(buf,"{name} = {{ version = {ver:?}, features = {feats:?}, default-features = false }}")
            }
        };
        self.toml_width = self.toml.width() as u16;
    }

    pub fn set_area(&mut self, border: Surround) {
        self.inner = border.inner();
        self.border = border;
    }

    pub fn update_area(&mut self, area: Rect) {
        if let Some(inner) = self.border.update_area(area) {
            self.inner = inner;
        }
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.border.render(buf);

        let Rect { x, y, width, .. } = self.inner;
        render_line(Some((&*self.toml, PKG_TOML)), buf, x, y, width as usize);

        if self.toml_width > width {
            let cell = buf.get_mut(width.saturating_sub(1) + x, y);
            cell.bg = BG_CURSOR;
            cell.fg = Color::White;
        }
    }
}
