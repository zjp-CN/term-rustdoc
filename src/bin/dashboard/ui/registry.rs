use crate::{
    dashboard::local_registry::LocalRegistry,
    ui::{render_line, LineState, Scrollable, Surround},
};
use nucleo::{
    pattern::{CaseMatching, Normalization},
    Injector, Nucleo,
};
use ratatui::prelude::*;
use std::{cell::RefCell, sync::Arc};
use term_rustdoc::util::xformat;

#[derive(Default)]
pub(super) struct PkgLists {
    local: LocalRegistry,
    filter: Vec<LocalPkgsIndex>,
}

impl PkgLists {
    fn new_local() -> Self {
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
        let list = PkgLists {
            filter: (0..registry.len()).map(LocalPkgsIndex).collect(),
            local: registry,
        };
        list.set_fuzzy_matcher();
        list
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

    /// Pass the pkg names to the fuzzy matcher.
    /// This should only called when full list is searched in.
    fn set_fuzzy_matcher(&self) {
        let inject = injector();
        for (idx, pkg) in self.local.iter().enumerate() {
            inject.push(LocalPkgsIndex(idx), |buf| {
                if let Some(buf) = buf.first_mut() {
                    *buf = pkg.name().into();
                }
            });
        }
    }

    fn update_search(&mut self, search_text: &str) {
        parse_pattern(search_text);
        get_fuzzy_result(&mut self.filter);
        self.fill_filter();
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
    pub fn new_local() -> Self {
        Registry {
            inner: Scrollable {
                lines: PkgLists::new_local(),
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
            let mut start = text.start;
            let style = Style::new();
            for line in lines {
                let pkg = xformat!("{start}. {}", pkgs[line.0].name());
                render_line(Some((&*pkg, style)), buf, x, y, width);
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

    pub fn update_search(&mut self, search_text: &str) {
        self.inner.lines.update_search(search_text);
    }

    /// Reset to all pkgs.
    pub fn clear_and_reset(&mut self) {
        self.inner.lines.force_all();
    }
}

type Matcher = Nucleo<LocalPkgsIndex>;

/// only one colo
fn init_fuzzy_matcher() -> Matcher {
    Nucleo::new(nucleo::Config::DEFAULT, Arc::new(|| {}), None, 1)
}

thread_local! {
    static MATCHER: RefCell< Matcher> = RefCell::new(init_fuzzy_matcher());
}

fn injector() -> Injector<LocalPkgsIndex> {
    MATCHER.with(|m| m.borrow().injector())
}

fn parse_pattern(search_text: &str) {
    MATCHER.with(|m| {
        m.borrow_mut().pattern.reparse(
            0,
            search_text,
            CaseMatching::Ignore,
            Normalization::Smart,
            false,
        );
    });
}

fn get_fuzzy_result(filter: &mut Vec<LocalPkgsIndex>) {
    MATCHER.with(move |m| {
        let matcher = &mut m.borrow_mut();
        let status = matcher.tick(100);
        if status.running {
            info!("Fuzzy Matcher is still running");
        }
        if status.changed {
            let snapshot = matcher.snapshot();
            let total = snapshot.item_count();
            let got = snapshot.matched_item_count();
            info!(total, got, "Snapshot yields matched items");
            if got == 0 {
                info!("no search result");
                return;
            }
            filter.clear();
            filter.extend(snapshot.matched_items(..).map(|item| item.data));
        }
    })
}
