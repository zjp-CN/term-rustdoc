use crate::{
    color::BG_CURSOR_LINE,
    fuzzy::Fuzzy,
    local_registry::{LocalRegistry, PkgInfo},
    ui::{render_line, LineState, Scrollable, Surround},
};
use ratatui::prelude::{Buffer, Rect};
use term_rustdoc::util::xformat;

#[derive(Default)]
pub(super) struct PkgLists {
    /// Local pkgs shown with latest version.
    local: LocalRegistry,
    /// Index of latest local pkgs.
    filter: Vec<LocalPkgsIndex>,
    /// Local pkgs with all versions, which are used in FeaturesUI to select
    /// a version and corresponding features.
    local_all_versions: LocalRegistry,
    fuzzy: Option<Fuzzy>,
}

impl PkgLists {
    fn new_local(fuzzy: Fuzzy) -> Self {
        let [registry, all] = match LocalRegistry::all_pkgs_with_latest_and_all_versions() {
            Ok(registry) => registry,
            Err(err) => {
                error!("{err}");
                return PkgLists::default();
            }
        };
        info!(
            "Found {} latest pkgs under {}",
            registry.len(),
            registry.registry_src_path().display()
        );
        PkgLists {
            filter: (0..registry.len()).map(LocalPkgsIndex).collect(),
            local: registry,
            local_all_versions: all,
            fuzzy: Some(fuzzy),
        }
    }

    /// Get all versions for a pkg, but in reverse order. (Latest is first)
    pub fn get_all_version(&self, name: &str) -> Vec<PkgInfo> {
        let all = &self.local_all_versions;
        let Ok(found) = all.binary_search_by(|info| info.name().cmp(name)) else {
            return Vec::new();
        };
        let before = all[..found]
            .iter()
            .rev()
            .take_while(|info| info.name() == name)
            .count();
        let after = all[found..]
            .iter()
            .take_while(|info| info.name() == name)
            .count();
        let mut all = all[found.saturating_sub(before)..found.saturating_add(after)].to_owned();
        all.sort_unstable_by(|a, b| b.ver().cmp(a.ver()));
        all
    }

    fn fill_filter(&mut self) {
        let filtered = &mut self.filter;
        if filtered.is_empty() {
            filtered.extend((0..self.local.len()).map(LocalPkgsIndex));
        }
    }

    /// clear the filter result and fill with all pkgs back
    fn force_all(&mut self) {
        self.filter.clear();
        self.filter
            .extend((0..self.local.len()).map(LocalPkgsIndex));
    }

    fn update_search(&mut self, pattern: &str) {
        struct Ele<'s>(&'s str, LocalPkgsIndex);
        impl AsRef<str> for Ele<'_> {
            fn as_ref(&self) -> &str {
                self.0
            }
        }
        impl From<Ele<'_>> for LocalPkgsIndex {
            fn from(value: Ele<'_>) -> Self {
                value.1
            }
        }

        if let Some(fuzzy) = &mut self.fuzzy {
            fuzzy.parse(pattern);
            let iter = self.local.iter().enumerate();
            let iter = iter.map(|(idx, pkg)| Ele(pkg.name(), LocalPkgsIndex(idx)));
            fuzzy.match_list(iter, &mut self.filter);
            self.fill_filter();
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub(super) struct LocalPkgsIndex(usize);

impl std::ops::Deref for PkgLists {
    type Target = [LocalPkgsIndex];

    fn deref(&self) -> &Self::Target {
        &self.filter
    }
}

impl LineState for LocalPkgsIndex {
    type State = usize;

    fn state(&self) -> Self::State {
        self.0
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        self.0 == *state
    }
}

#[derive(Default)]
pub struct Registry {
    pub inner: Scrollable<PkgLists>,
    border: Surround,
}

impl Registry {
    pub fn new_local(fuzzy: Fuzzy) -> Self {
        Registry {
            inner: Scrollable {
                lines: PkgLists::new_local(fuzzy),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn set_area(&mut self, border: Surround) {
        self.inner.area = border.inner();
        self.border = border;
    }

    pub fn scroll_text(&mut self) -> &mut Scrollable<PkgLists> {
        &mut self.inner
    }

    pub fn render(&self, buf: &mut Buffer, current: bool) {
        // render border
        self.border.render(buf);

        let text = &self.inner;
        let Some(lines) = text.visible_lines() else {
            return;
        };
        let Rect { x, mut y, .. } = text.area;
        let width = text.area.width as usize;
        let pkgs = &text.lines.local;
        // render current selected pkg
        if current && text.get_line_of_current_cursor().is_some() {
            let row = text.area.y + text.cursor.y;
            for col in x..text.area.width + x {
                buf.get_mut(col, row).set_bg(BG_CURSOR_LINE);
            }
        }

        let mut start = text.start + 1;
        for line in lines {
            let pkg = &pkgs[line.0];
            let [(name, style_name), (ver, style_ver)] = pkg.styled_name_ver();
            let num = xformat!("{start:02}. ");
            // render name and version, but with extra info and styles
            let line = [
                (&*num, style_name),
                (name, style_name),
                (" v", style_ver),
                (ver, style_ver),
            ];
            render_line(line, buf, x, y, width);
            y += 1;
            start += 1;
        }

        // write the match result to the border bottom line
        let text = xformat!(
            " Got {} / Total {} ",
            self.inner.total_len(),
            self.inner.lines.local.len()
        );
        self.border.render_only_bottom_right_text(buf, &text);
    }

    /// Update the fuzzy result every time the input pattern changes.
    pub fn update_search(&mut self, pattern: &str) {
        self.inner.lines.update_search(pattern);
        self.inner.start = 0;
        self.set_cursor();
    }

    /// Reset to all pkgs.
    pub fn clear_and_reset(&mut self) {
        self.inner.lines.force_all();
        self.inner.start = 0;
        self.set_cursor();
    }

    /// Set the cursor to previously selected pkg position if possible.
    /// If not possible, i.e. the previous selected result is not in visual,
    /// this resets the cursor to the first item.
    fn set_cursor(&mut self) {
        if !self.inner.check_if_can_return_to_previous_cursor() {
            // NOTE: we reset the cursor to first line on purporse here
            self.inner.cursor.y = 0;
        }
    }

    pub fn get_pkg(&self, y: Option<u16>) -> Option<PkgInfo> {
        let pkgs = &self.inner.lines.local;
        y.map_or_else(
            || self.inner.get_line_of_current_cursor(),
            |y| self.inner.get_line_on_screen(y),
        )
        .map(|idx| pkgs[idx.0].clone())
    }
}
