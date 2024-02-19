use crate::{
    dashboard::local_registry::LocalRegistry,
    fuzzy::Fuzzy,
    ui::{render_line, LineState, Scrollable, Surround},
};
use ratatui::prelude::{Buffer, Color, Rect, Style};
use term_rustdoc::util::xformat;
use unicode_width::UnicodeWidthStr;

#[derive(Default)]
pub(super) struct PkgLists {
    local: LocalRegistry,
    filter: Vec<LocalPkgsIndex>,
    fuzzy: Option<Fuzzy>,
}

impl PkgLists {
    fn new_local(fuzzy: Fuzzy) -> Self {
        let registry = match LocalRegistry::lastest_pkgs_in_latest_registry() {
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
            fuzzy: Some(fuzzy),
        }
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

    pub fn render(&self, buf: &mut Buffer) {
        // render border
        self.border.render(buf);

        let text = &self.inner;
        let Rect {
            x, mut y, width, ..
        } = text.area;
        let width = width as usize;
        let pkgs = &text.lines.local;
        if let Some(lines) = text.visible_lines() {
            let mut start = text.start + 1;
            let style = Style::new();
            let ver_style = Style {
                fg: Some(Color::DarkGray),
                ..Style::new()
            };
            for line in lines {
                let pkg = &pkgs[line.0];
                let mut text = xformat!("{start:02}. {}", pkg.name());
                let used_width = render_line(Some((&*text, style)), buf, x, y, width);
                let need = pkg.ver().width() + 2;
                // display version if possible
                if width.checked_sub(used_width + need).is_some() {
                    text.clear();
                    text.push(' ');
                    text.push('v');
                    text.push_str(pkg.ver());
                    let version = Some((&*text, ver_style));
                    render_line(version, buf, x + used_width as u16, y, need);
                }
                y += 1;
                start += 1;
            }
        }

        // write the match result to the border bottom line
        let text = xformat!(
            " got {} / total {} ",
            self.inner.total_len(),
            self.inner.lines.local.len()
        );
        self.border.render_with_bottom_right_text(buf, &text);
    }

    pub fn update_search(&mut self, pattern: &str) {
        self.inner.lines.update_search(pattern);
    }

    /// Reset to all pkgs.
    pub fn clear_and_reset(&mut self) {
        self.inner.lines.force_all();
    }
}
