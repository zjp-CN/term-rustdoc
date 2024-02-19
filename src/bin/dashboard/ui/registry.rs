use crate::{
    dashboard::local_registry::LocalRegistry,
    fuzzy::Fuzzy,
    ui::{render_line, LineState, Scrollable, Surround},
};
use ratatui::prelude::{Buffer, Color, Rect};
use term_rustdoc::util::xformat;

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
            // render current selected pkg
            if text.get_line_of_current_cursor().is_some() {
                let row = text.area.y + text.cursor.y;
                for col in x..text.area.width + x {
                    buf.get_mut(col, row).set_bg(Color::from_u32(0x00548B54)); // #548B54
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
        }

        // write the match result to the border bottom line
        let text = xformat!(
            " got {} / total {} ",
            self.inner.total_len(),
            self.inner.lines.local.len()
        );
        self.border.render_with_bottom_right_text(buf, &text);
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
}
