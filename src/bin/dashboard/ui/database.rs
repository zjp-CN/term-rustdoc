mod cache;

use self::cache::{Cache, CacheID, Count, SortKind};
use crate::{
    color::BG_CURSOR_LINE,
    database::{CachedDocInfo, DataBase, Features, PkgKey, PkgWithFeatures},
    event::Sender,
    fuzzy::Fuzzy,
    ui::{render_line, Scroll, Surround},
};
use ratatui::prelude::{Buffer, Rect};
use term_rustdoc::{tree::CrateDoc, util::xformat};

#[derive(Default)]
pub struct PkgDocs {
    db: DataBase,
    caches: Vec<Cache>,
    caches_sort: SortKind,
    /// NOTE: the indices only change when the length of caches changes,
    /// because we need to sort caches for displaying, thus both lengths should equal.
    indices: Vec<CacheID>,
    fuzzy: Option<Fuzzy>,
}

impl std::ops::Deref for PkgDocs {
    type Target = [CacheID];

    fn deref(&self) -> &Self::Target {
        &self.indices
    }
}

#[derive(Default)]
pub struct DataBaseUI {
    inner: Scroll<PkgDocs>,
    border: Surround,
}

impl DataBaseUI {
    pub fn init(sender: Sender, fuzzy: Fuzzy) -> Self {
        let mut ui = DataBaseUI::default();
        if let Ok(db) = DataBase::init(sender) {
            let caches: Vec<_> = db
                .all_caches()
                .map_err(|err| error!("Failed to read CachedDocInfo:\\n{err}"))
                .map(|v| v.into_iter().map(Cache::new_unloaded).collect())
                .unwrap_or_default();
            ui.pkg_docs().indices = (0..caches.len()).map(CacheID).collect();
            ui.pkg_docs().caches = caches;
            ui.pkg_docs().db = db;
            // NOTE: by default we choose the sort for all by *started* time,
            // it's the time to start compiling instead of finishing.
            // But db stores docs sequentially after generation, thus the sort is needed.
            // We could use finish time, but I don't know which is better.
            ui.sort_caches();
        }
        ui.pkg_docs().fuzzy = Some(fuzzy);
        ui
    }

    fn pkg_docs(&mut self) -> &mut PkgDocs {
        &mut self.inner.lines
    }

    pub fn update_search(&mut self, pattern: &str) {
        struct Ele<'s>(&'s str, CacheID);
        impl AsRef<str> for Ele<'_> {
            fn as_ref(&self) -> &str {
                self.0
            }
        }
        impl From<Ele<'_>> for CacheID {
            fn from(value: Ele<'_>) -> Self {
                value.1
            }
        }

        let pkg_docs = self.pkg_docs();
        if let Some(fuzzy) = &mut pkg_docs.fuzzy {
            fuzzy.parse(pattern);
            let iter = pkg_docs.caches.iter().enumerate();
            let iter = iter.map(|(idx, cache)| Ele(cache.name(), CacheID(idx)));
            fuzzy.match_list(iter, &mut pkg_docs.indices);

            // fill all if the result is empty
            if pkg_docs.indices.is_empty() {
                let indices = (0..pkg_docs.caches.len()).map(CacheID);
                pkg_docs.indices.extend(indices);
            }

            self.set_cursor();
        }
    }

    /// Reset to all pkgs.
    pub fn clear_and_reset(&mut self) {
        self.pkg_docs().indices.clear();
        let indices = (0..self.pkg_docs().caches.len()).map(CacheID);
        self.pkg_docs().indices.extend(indices);
        self.inner.start = 0;
        self.set_cursor();
    }

    /// Also see `Registry::set_cursor`.
    fn set_cursor(&mut self) {
        self.inner.start = 0;
        if !self.inner.check_if_can_return_to_previous_cursor() {
            // NOTE: we reset the cursor to first line on purporse here
            self.inner.cursor.y = 0;
        }
    }

    /// Usually this appends the in-progress doc to the caches vec.
    ///
    /// But when the pkg is cached before, there will be a duplicate.
    /// In this case, this method will change its status.
    pub fn compile_doc(&mut self, pkg: PkgWithFeatures) {
        if let Some(pkg_key) = self.pkg_docs().db.compile_doc(pkg) {
            let caches = &mut self.pkg_docs().caches;
            if let Some(old) = caches.iter_mut().find(|cache| **cache == pkg_key) {
                *old = Cache::new_being_cached(pkg_key);
            } else {
                let id = CacheID(caches.len());
                caches.push(Cache::new_being_cached(pkg_key));
                self.pkg_docs().indices.push(id);
            }
            self.sort_caches();
        }
    }

    /// Sort the Cache vec because the inner states have changed.
    fn sort_caches(&mut self) {
        let kind = self.pkg_docs().caches_sort;
        self.pkg_docs().caches.sort_unstable_by(kind.cmp_fn());
    }

    pub fn switch_sort(&mut self) {
        let kind = self.pkg_docs().caches_sort.next();
        self.pkg_docs().caches_sort = kind;
        self.sort_caches();
    }

    /// Downgrade a loaded doc to cached doc.
    /// This will free the memory of the loaded doc.
    ///
    /// y is Some for a mouse click, and None for a key press.
    ///
    /// This method doesn't mean deleting the db file, so it won't
    /// apply for Cached kind.
    /// It doesn't means removing the being-cached kind either, because
    /// for now there is no way to cancel a compilation task.
    pub fn downgrade(&mut self, y: Option<u16>) {
        let line = y.map_or_else(
            || self.inner.get_line_of_current_cursor(),
            |y| self.inner.get_line_on_screen(y),
        );
        if let Some(id) = line.map(|id| id.0) {
            if let Some(loaded) = self.inner.lines.caches.get_mut(id) {
                if let Some(key) = loaded.downgrade() {
                    self.inner.lines.db.send_downgraded_doc(key);
                }
                // sort because of sort kind
                self.sort_caches();
            }
        }
    }
}

/// Rendering
impl DataBaseUI {
    pub fn set_area(&mut self, surround: Surround) {
        self.inner.area = surround.inner();
        self.border = surround;
    }

    pub fn render(&self, buf: &mut Buffer, current: bool) {
        self.border.render(buf);

        let Some(ids) = self.inner.visible_lines() else {
            return;
        };
        let mut start = self.inner.start + 1;
        let Rect { x, mut y, .. } = self.inner.area;
        let width = self.inner.area.width as usize;

        // render current selected pkg
        let text = &self.inner;
        if current && text.get_line_of_current_cursor().is_some() {
            let row = text.area.y + text.cursor.y;
            for col in x..text.area.width + x {
                buf.get_mut(col, row).set_bg(BG_CURSOR_LINE);
            }
        }

        let pkgs = &text.lines.caches;
        for id in ids {
            let num = xformat!("{start:02}. ");
            let [(kind, style_kind), (name, style_name), (ver, style_ver), (feat, style_feat)] =
                pkgs[id.0].line();
            let line = [
                (kind, style_kind),
                (" ", style_kind),
                (&*num, style_name),
                (name, style_name),
                (" v", style_ver),
                (ver, style_ver),
                (" ", style_feat),
                (feat, style_feat),
            ];
            render_line(line, buf, x, y, width);
            start += 1;
            y += 1;
        }

        // write the sor description and counts to the border bottom line
        let mut count = Count::default();
        let iter = self.inner.lines.caches.iter();
        iter.for_each(|cache| cache.add(&mut count));
        let text = count.describe();
        let used = self.border.render_only_bottom_right_text(buf, &text);
        let desc = self.inner.lines.caches_sort.describe();
        self.border.render_only_bottom_left_text(buf, desc, used);
    }

    /// When y is Some, it comes from a mouse click posotion.
    /// When y is None, it comes from a key press.
    pub fn load_doc(&mut self, y: Option<u16>) {
        let line = y.map_or_else(
            || self.inner.get_line_of_current_cursor(),
            |y| self.inner.get_line_on_screen(y),
        );
        if let Some(id) = line.map(|id| id.0) {
            if self.inner.lines.caches[id].loadable() {
                if let Some(cache) = self.inner.lines.caches.get_mut(id) {
                    cache.load_doc(&self.inner.lines.db);
                    // sort because of sort kind
                    self.sort_caches();
                }
            }
        }
    }

    pub fn receive_compiled_doc(&mut self, info: CachedDocInfo) {
        let key = &info.pkg;
        let caches = &mut self.pkg_docs().caches;
        if let Some(cache) = caches.iter_mut().find(|cache| cache.is_in_progress(key)) {
            *cache = Cache::new_unloaded(info);
        } else {
            error!("{key:?} is not found in the caches vec, but it should.");
            let id = CacheID(caches.len());
            caches.push(Cache::new_unloaded(info));
            self.pkg_docs().indices.push(id);
        }
        self.sort_caches();
    }

    pub fn is_empty(&self) -> bool {
        self.inner.all_lines().is_empty()
    }

    pub fn scroll_text(&mut self) -> &mut Scroll<PkgDocs> {
        &mut self.inner
    }

    pub fn get_loaded_doc(&self, key: &PkgKey) -> Option<CrateDoc> {
        let iter = &mut self.inner.lines.caches.iter();
        iter.find_map(|cache| cache.get_loaded_doc(key))
    }

    pub fn get_current_pkg(&self) -> Option<(&str, &str, &Features)> {
        if let Some(idx) = self.inner.get_line_of_current_cursor().map(|id| id.0) {
            if let Some(cache) = self.inner.lines.caches.get(idx) {
                return Some(cache.pkg_feat());
            }
        }
        None
    }
}
