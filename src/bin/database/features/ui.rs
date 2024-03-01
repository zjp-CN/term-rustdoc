use super::{
    parse_cargo_toml::{FeatureControlledByUsers, FeaturesControlledByUsers},
    Features,
};
use crate::{
    color::{BG_CURSOR_LINE, FG_FEATURES},
    database::util::PkgWithFeatures,
    local_registry::PkgInfo,
    ui::{render_line, LineState, Scrollable, Surround},
};
use ratatui::{
    prelude::{Buffer, Color, Modifier, Rect, Style},
    widgets::{Block, Borders},
};
use smallvec::{smallvec, SmallVec};
use std::path::PathBuf;
use term_rustdoc::{
    tree::Text,
    util::{xformat, XString},
};

type LineTexts = SmallVec<[Text; 5]>;

#[derive(Clone, Default, Debug)]
pub struct Line {
    selected: Selected,
    feature: XString,
    render: LineTexts,
}

impl Line {
    fn new(feat: &str, control: &FeatureControlledByUsers) -> Line {
        let selected = Selected::new(control);
        let feature = feat.into();
        let render = selected.render_line(&feature);
        Line {
            selected,
            feature,
            render,
        }
    }

    fn line(&self) -> impl Iterator<Item = (&str, Style)> {
        self.render.iter().map(|w| (&*w.text, w.style))
    }
}

impl LineState for Line {
    type State = (Selected, XString);

    fn state(&self) -> Self::State {
        (self.selected.clone(), self.feature.clone())
    }

    fn is_identical(&self, (selected, feat): &Self::State) -> bool {
        self.selected == *selected && self.feature == feat
    }
}

/// A state for a feature.
#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub enum Selected {
    /// User selected this feature.
    Yes,
    /// User doesn't select this feature.
    #[default]
    No,
    /// Some feature(s) selected enable(s) this feature, so user can't disable it.
    /// But the feature can still be enabled, so when it happens, the state will become
    /// NeedlesslyEnabledBy.
    LockedBy(XString),
    /// The feature is selected by user, but will emit a warning to have enabled it.
    NeedlesslyEnabledBy(XString),
}

impl Selected {
    fn new(control: &FeatureControlledByUsers) -> Self {
        let enabled_by = &control.enabled_by;
        match (control.specify_enabled, enabled_by.is_empty()) {
            (true, true) => Selected::Yes,
            (false, true) => Selected::No,
            (false, false) => Selected::LockedBy(xformat!("{enabled_by:?}")),
            (true, false) => Selected::NeedlesslyEnabledBy(xformat!("{enabled_by:?}")),
        }
    }

    fn render_line(&self, feat: &XString) -> LineTexts {
        const E: Style = Style::new();
        const Y: Style = Style {
            fg: Some(FG_FEATURES),
            add_modifier: Modifier::BOLD,
            ..Style::new()
        };
        const H: Style = Style {
            fg: Some(FG_FEATURES),
            add_modifier: Modifier::ITALIC,
            ..Style::new()
        };
        const L: Style = Style {
            fg: Some(Color::LightGreen),
            add_modifier: Modifier::BOLD,
            ..Style::new()
        };
        const R: Style = Style {
            fg: Some(Color::Red),
            add_modifier: Modifier::BOLD,
            ..Style::new()
        };
        match self {
            Selected::Yes => {
                smallvec![
                    Text::new(" ".into(), Y),
                    Text::new(" ".into(), E),
                    Text::new(feat.clone(), Y),
                ]
            }
            Selected::No => smallvec![Text::new("   ".into(), E), Text::new(feat.clone(), E)],
            Selected::LockedBy(s) => {
                smallvec![
                    Text::new("🔒".into(), L),
                    Text::new(" ".into(), E),
                    Text::new(feat.clone(), L),
                    Text::new(" Locked by these features: ".into(), H),
                    Text::new(s.clone(), H),
                ]
            }
            Selected::NeedlesslyEnabledBy(s) => {
                smallvec![
                    Text::new(" ".into(), R),
                    Text::new(" ".into(), E),
                    Text::new(feat.clone(), R),
                    Text::new(" Already enabled by these features: ".into(), H),
                    Text::new(s.clone(), H),
                ]
            }
        }
    }
}

#[derive(Default)]
pub struct Select {
    select: Option<FeaturesControlledByUsers>,
    pkg: Option<PkgWithFeatures>,
    list: Vec<Line>,
}

impl Select {
    pub fn from_registry(mut pkg_dir: PathBuf, pkg_info: PkgInfo) -> Select {
        pkg_dir.push("Cargo.toml");
        let path = &pkg_dir;
        let select = FeaturesControlledByUsers::new(path)
            .map_err(|err| {
                error!("Features not parsed from {}:\n{err}", path.display());
            })
            .ok()
            // Don't display selection popup if no feature
            .filter(|f| !f.features.is_empty() || f.features.keys().eq(Some(&"default")));
        pkg_dir.pop();
        let mut select = Select {
            select,
            pkg: Some(PkgWithFeatures {
                features: Features::Default,
                dir: pkg_dir,
                info: pkg_info,
            }),
            list: Vec::new(),
        };
        select.update_lines();
        select
    }

    fn update_lines(&mut self) {
        if let Some(select) = &self.select {
            self.list = select
                .features
                .iter()
                .map(|(f, control)| Line::new(f, control))
                .collect();
        }
    }

    pub fn update_features(&mut self) {
        if let Some(pkg) = &mut self.pkg {
            let features_controlled_by_users = self.select.as_ref();
            pkg.features = features_controlled_by_users
                .map(|select| {
                    let selected = self
                        .list
                        .iter()
                        .filter_map(|l| {
                            if matches!(
                                l.selected,
                                Selected::Yes | Selected::NeedlesslyEnabledBy(_)
                            ) && l.feature != "default"
                            {
                                // Skip default here because this is checked below,
                                // otherwise, we'll see `DefaultPlus(["default", ...])`.
                                return Some(l.feature.clone());
                            }
                            None
                        })
                        .collect::<Box<[_]>>();
                    let default = select
                        .features
                        .get("default")
                        .map(|d| d.is_enabled())
                        .unwrap_or(false);
                    match (default, selected.is_empty()) {
                        (true, true) => Features::Default,
                        (true, false) => Features::DefaultPlus(selected),
                        (false, true) => Features::NoDefault,
                        (false, false) => Features::NoDefaultPlus(selected),
                    }
                })
                .unwrap_or_default();
        }
    }

    pub fn pkg_with_features(&mut self) -> Option<PkgWithFeatures> {
        self.update_features();
        self.pkg.clone()
    }

    // pub fn take_pkg_with_features(&mut self) -> Option<PkgWithFeatures> {
    //     self.pkg.take()
    // }
}

impl std::ops::Deref for Select {
    type Target = [Line];

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

#[derive(Default)]
pub struct FeaturesUI {
    inner: Scrollable<Select>,
    border: Surround,
}

impl FeaturesUI {
    pub fn new(pkg_dir: PathBuf, pkg_info: PkgInfo, outer: Rect) -> FeaturesUI {
        let border = Surround::new(
            Block::new()
                .borders(Borders::ALL)
                .title(" Features Selection "),
            outer,
        );
        let inner = Scrollable::<Select> {
            lines: Select::from_registry(pkg_dir, pkg_info),
            area: border.inner(),
            ..Default::default()
        };
        FeaturesUI { inner, border }
    }

    pub fn scroll_text(&mut self) -> &mut Scrollable<Select> {
        &mut self.inner
    }

    /// If there is no feature available to select, returns true.
    pub fn skip_selection(&self) -> bool {
        let select = self.inner.lines.select.as_ref();
        select
            .map(|s| {
                s.features.is_empty() || {
                    s.features.len() == 1 && s.manifest.default_for_nothing()
                }
            })
            .unwrap_or(true)
    }

    /// If this returns true, it means we don't need to generate a new instance
    /// and reuse the FeaturesUI based on selected features.
    pub fn is_same_pkg(&self, info: &PkgInfo) -> bool {
        let pkg = self.inner.lines.pkg.as_ref();
        pkg.map(|pkg| pkg.info.is_same_pkg(info)).unwrap_or(false)
    }

    pub fn pkg_with_features(&mut self) -> Option<PkgWithFeatures> {
        self.inner.lines.pkg_with_features()
    }

    pub fn toggle(&mut self) {
        if let Some(feat) = self
            .inner
            .get_line_of_current_cursor()
            .map(|f| f.feature.clone())
        {
            if let Some(select) = &mut self.inner.lines.select {
                select.toggle(&feat);
            }
            self.inner.lines.update_lines();
        }
    }

    pub fn update_area(&mut self, outer: Rect) {
        if let Some(inner) = self.border.update_area(outer) {
            self.inner.area = inner;
        }
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.border.render(buf);

        // render feature list
        let Some(lines) = self.inner.visible_lines() else {
            return;
        };
        let width = self.inner.area.width as usize;
        let area @ Rect { x, mut y, .. } = self.inner.area;
        // hightlight current line
        let cursor = self.inner.cursor.y;
        if lines.get(cursor as usize).is_some() {
            for offset in 0..area.width {
                buf.get_mut(x + offset, y + cursor).bg = BG_CURSOR_LINE;
            }
        }
        for feat in lines {
            render_line(feat.line(), buf, x, y, width);
            y += 1;
        }
    }
}
