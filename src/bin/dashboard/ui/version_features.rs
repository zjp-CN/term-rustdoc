use crate::{
    color::BG_CURSOR_LINE,
    database::FeaturesUI,
    local_registry::{PkgInfo, PkgNameVersion},
    ui::{render_line, LineState, Scrollable, Surround},
};
use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect},
    widgets::{Block, Borders},
};

#[derive(Default)]
pub struct VersionFeatures {
    features: FeaturesUI,
    versions: Versions,
    area: Rect,
}

fn split_ver_feat(outer: Rect) -> [Rect; 2] {
    Layout::vertical([Constraint::Percentage(75), Constraint::Percentage(25)]).areas(outer)
}

impl VersionFeatures {
    pub fn new(pkg_info: PkgInfo, all_verions: Vec<PkgInfo>, outer: Rect) -> Self {
        let [feat, ver] = split_ver_feat(outer);
        VersionFeatures {
            features: FeaturesUI::new(pkg_info, feat),
            versions: Versions::new(all_verions, ver),
            area: outer,
        }
    }

    pub fn features(&mut self) -> &mut FeaturesUI {
        &mut self.features
    }

    pub fn update_area(&mut self, outer: Rect) {
        if self.area == outer {
            return;
        }
        let [feat, ver] = split_ver_feat(outer);
        self.features.update_area(feat);
        self.versions.update_area(ver);
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.features.render(buf);
        self.versions.render(buf);
    }
}

#[derive(Default)]
struct Versions {
    inner: Scrollable<VersionsInner>,
    border: Surround,
}

impl Versions {
    fn new(all_verions: Vec<PkgInfo>, area: Rect) -> Self {
        let border = Surround::new(
            Block::new()
                .title(" Version Selection ")
                .borders(Borders::ALL),
            area,
        );
        Self {
            inner: Scrollable {
                lines: VersionsInner { all: all_verions },
                area: border.inner(),
                ..Default::default()
            },
            border,
        }
    }

    fn update_area(&mut self, area: Rect) {
        if let Some(inner) = self.border.update_area(area) {
            self.inner.area = inner;
        }
    }

    fn render(&self, buf: &mut Buffer) {
        self.border.render(buf);

        let width = self.inner.area.width as usize;
        let Rect { x, mut y, .. } = self.inner.area;
        if self.inner.get_line_of_current_cursor().is_some() {
            let current = y + self.inner.cursor.y;
            for w in 0..self.inner.area.width {
                buf.get_mut(x + w, current).bg = BG_CURSOR_LINE;
            }
        }
        if let Some(lines) = self.inner.visible_lines() {
            for info in lines {
                let line = info.styled_name_ver();
                render_line(line, buf, x, y, width);
                y += 1;
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
