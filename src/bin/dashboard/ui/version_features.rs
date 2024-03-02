use crate::{
    color::{BG_CURSOR_LINE, FG_CURSOR_LINE},
    database::FeaturesUI,
    local_registry::{PkgInfo, PkgNameVersion},
    ui::{render_line, LineState, Scroll, Scrollable, Surround},
};
use ratatui::{
    prelude::{Buffer, Constraint, Layout, Modifier, Rect, Style},
    widgets::{Block, Borders},
};

#[derive(Default)]
pub struct VersionFeatures {
    features: FeaturesUI,
    versions: Versions,
    current: Panel,
    area: Rect,
}

#[derive(Default, Clone, Copy)]
enum Panel {
    #[default]
    Features,
    Versions,
}

fn split_ver_feat(outer: Rect, ver_width: u16) -> [Rect; 2] {
    if ver_width == 0 {
        // don't show versions if zero width
        [Rect::default(), outer]
    } else {
        Layout::horizontal([Constraint::Length(ver_width), Constraint::Min(0)]).areas(outer)
    }
}

impl VersionFeatures {
    pub fn new(pkg_info: PkgInfo, all_verions: Vec<PkgInfo>, outer: Rect) -> Self {
        // used this fixed width to show versions on the left
        let ver_width = all_verions
            .iter()
            // should be .width() here, but assume a version consists of ascii chars,
            // which is always upheld for pkgs from crate.io
            .map(|v| 4 + v.ver().len() as u16)
            .max()
            .unwrap_or(0);
        let [ver, feat] = split_ver_feat(outer, ver_width);
        VersionFeatures {
            features: FeaturesUI::new(pkg_info, feat),
            versions: Versions::new(all_verions, ver_width, ver),
            current: Panel::Features,
            area: outer,
        }
    }

    pub fn switch_panel(&mut self) {
        self.current = match self.current {
            Panel::Features => Panel::Versions,
            Panel::Versions => Panel::Features,
        };
    }

    pub fn scroll_text(&mut self) -> &mut dyn Scrollable {
        match self.current {
            Panel::Features => self.features.scroll_text(),
            Panel::Versions => &mut self.versions.inner,
        }
    }

    pub fn contains(&self, position: (u16, u16)) -> bool {
        self.area.contains(position.into())
    }

    pub fn respond_to_left_click(&mut self, position @ (_, y): (u16, u16)) {
        let features = self.features().scroll_text();
        if features.area.contains(position.into()) {
            features.set_cursor(y.saturating_sub(features.area.y));
            self.current = Panel::Features;
            return;
        }

        let versions = self.versions.scroll_text();
        if versions.area.contains(position.into()) {
            versions.set_cursor(y.saturating_sub(versions.area.y));
            if let Some(info) = versions.get_line_of_current_cursor() {
                self.features.update_pkg(info.clone());
            }
            self.current = Panel::Versions;
        }
    }

    pub fn features(&mut self) -> &mut FeaturesUI {
        &mut self.features
    }

    pub fn toggle_features(&mut self) {
        if matches!(self.current, Panel::Features) {
            self.features.toggle();
        }
    }

    /// Skip selection popup when features requirements are met and single version.
    pub fn skip_selection(&self) -> bool {
        self.versions.inner.total_len() == 1 && self.features.skip_selection()
    }

    pub fn update_area(&mut self, outer: Rect) {
        if self.area == outer {
            return;
        }
        let [ver, feat] = split_ver_feat(outer, self.versions.inner.max_width);
        self.features.update_area(feat);
        self.versions.update_area(ver);
    }

    pub fn render(&self, buf: &mut Buffer) {
        let (feat, ver) = match self.current {
            Panel::Features => (true, false),
            Panel::Versions => (false, true),
        };
        self.features.render(buf, feat);
        self.versions.render(buf, ver);
    }
}

#[derive(Default)]
struct Versions {
    inner: Scroll<VersionsInner>,
    border: Surround,
}

impl Versions {
    fn new(all_verions: Vec<PkgInfo>, max_width: u16, area: Rect) -> Self {
        let border = Surround::new(Block::new().title("Version").borders(Borders::ALL), area);
        Self {
            inner: Scroll {
                lines: VersionsInner { all: all_verions },
                area: border.inner(),
                max_width,
                ..Default::default()
            },
            border,
        }
    }

    fn scroll_text(&mut self) -> &mut Scroll<VersionsInner> {
        &mut self.inner
    }

    fn update_area(&mut self, area: Rect) {
        if let Some(inner) = self.border.update_area(area) {
            self.inner.area = inner;
        }
    }

    fn render(&self, buf: &mut Buffer, current_line: bool) {
        self.border.render(buf);

        let width = self.inner.area.width as usize;
        let Rect { x, mut y, .. } = self.inner.area;
        if let Some(lines) = self.inner.visible_lines() {
            for info in lines {
                let line = [(info.ver(), Style::new())];
                render_line(line, buf, x, y, width);
                y += 1;
            }
        }
        if self.inner.get_line_of_current_cursor().is_some() {
            let current = self.inner.area.y + self.inner.cursor.y;
            for w in 0..self.inner.area.width {
                let cell = buf.get_mut(x + w, current);
                if current_line {
                    cell.bg = BG_CURSOR_LINE;
                }
                cell.fg = FG_CURSOR_LINE;
                cell.modifier = Modifier::BOLD;
            }
        }
    }
}

#[derive(Default)]
struct VersionsInner {
    /// TODO: add cached status
    all: Vec<PkgInfo>,
}

impl std::ops::Deref for VersionsInner {
    type Target = [PkgInfo];

    fn deref(&self) -> &Self::Target {
        &self.all
    }
}

impl LineState for PkgInfo {
    type State = PkgNameVersion;

    fn state(&self) -> Self::State {
        self.to_name_ver()
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        state.name_ver() == [self.name(), self.ver()]
    }
}
