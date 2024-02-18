use crate::{
    dashboard::local_registry::LocalRegistry,
    ui::{render_line, LineState, Scrollable},
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
    matcher: Option<Injector<LocalPkgsIndex>>,
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
        let mut list = PkgLists {
            filter: (0..registry.len()).map(LocalPkgsIndex).collect(),
            local: registry,
            matcher: None,
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

    /// Pass the pkg names to the fuzzy matcher.
    /// This should only called when full list is searched in.
    fn set_fuzzy_matcher(&mut self) {
        let inject = self.matcher.get_or_insert_with(injector);
        for (idx, pkg) in self.local.iter().enumerate() {
            inject.push(LocalPkgsIndex(idx), |buf| {
                if let Some(buf) = buf.first_mut() {
                    *buf = pkg.name().into();
                }
            });
        }
    }

    fn update_search(&mut self, search_text: &str) -> Option<FuzzyOutput> {
        parse_pattern(search_text);
        let res = get_fuzzy_result(&mut self.filter);
        self.fill_filter();
        res
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
    pub text: Scrollable<PkgLists>,
    matched: FuzzyOutput,
}

impl Registry {
    pub fn new_local() -> Self {
        Registry {
            text: Scrollable {
                lines: PkgLists::new_local(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn set_area(&mut self, area: Rect) {
        self.text.area = area;
    }

    pub fn scroll_text(&mut self) -> &mut Scrollable<PkgLists> {
        &mut self.text
    }

    pub fn render(&self, buf: &mut Buffer) {
        let text = &self.text;
        let Rect {
            x,
            mut y,
            width,
            height,
        } = text.area;
        let width = width as usize;
        let bottom = y + height;
        let pkgs = &text.lines.local;
        let style = Style::new();
        if let Some(lines) = text.visible_lines() {
            for line in lines {
                render_line(Some((pkgs[line.0].name(), style)), buf, x, y, width);
                y += 1;
            }
        }
        // write the match result to the border bottom line
        render_line(
            Some((
                &*xformat!("got {} / total {}", self.matched.got, self.matched.total),
                style,
            )),
            buf,
            x,
            bottom,
            width,
        );
    }

    pub fn update_search(&mut self, search_text: &str) {
        if let Some(matched) = self.text.lines.update_search(search_text) {
            self.matched = matched;
        }
    }
}

type Matcher = Nucleo<LocalPkgsIndex>;

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
        )
    });
}

#[derive(Default)]
struct FuzzyOutput {
    total: u32,
    got: u32,
}

fn get_fuzzy_result(filter: &mut Vec<LocalPkgsIndex>) -> Option<FuzzyOutput> {
    MATCHER.with(move |m| {
        let mut res = None;
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
            res = Some(FuzzyOutput { total, got });
            if got == 0 {
                return res;
            }
            filter.clear();
            filter.extend(snapshot.matched_items(..).map(|item| item.data));
        }
        res
    })
}
