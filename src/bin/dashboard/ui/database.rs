mod cache;

use self::cache::{Cache, CacheID, Count, SortKind};
use crate::{
    database::{CachedDocInfo, DataBase},
    event::Sender,
    local_registry::PkgInfo,
    ui::{render_line, Scrollable, Surround},
};
use ratatui::prelude::{Buffer, Color, Rect};
use std::path::PathBuf;
use term_rustdoc::util::xformat;

#[derive(Default)]
pub struct PkgDocs {
    db: DataBase,
    caches: Vec<Cache>,
    caches_sort: SortKind,
    /// NOTE: the indices only change when the length of caches changes,
    /// because we need to sort caches for displaying, thus both lengths should equal.
    indices: Vec<CacheID>,
}

impl std::ops::Deref for PkgDocs {
    type Target = [CacheID];

    fn deref(&self) -> &Self::Target {
        &self.indices
    }
}

#[derive(Default)]
pub struct DataBaseUI {
    inner: Scrollable<PkgDocs>,
    border: Surround,
}

impl DataBaseUI {
    pub fn init(sender: Sender) -> Self {
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
        }
        ui
    }

    fn pkg_docs(&mut self) -> &mut PkgDocs {
        &mut self.inner.lines
    }

    /// Usually this appends the in-progress doc to the caches vec.
    ///
    /// But when the pkg is cached before, there will be a duplicate.
    /// In this case, this method will change its status.
    pub fn compile_doc(&mut self, pkg_dir: PathBuf, pkg_info: PkgInfo) {
        if let Some(pkg_key) = self.pkg_docs().db.compile_doc(pkg_dir, pkg_info) {
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
                buf.get_mut(col, row).set_bg(Color::from_u32(0x005DA063)); // #5DA063
            }
        }

        let pkgs = &text.lines.caches;
        for id in ids {
            let num = xformat!("{start:02}. ");
            let [(kind, style_kind), (name, style_name), (ver, style_ver)] = pkgs[id.0].line();
            let line = [
                (kind, style_kind),
                (" ", style_kind),
                (&*num, style_name),
                (name, style_name),
                (" v", style_ver),
                (ver, style_ver),
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

    pub fn load_doc(&mut self) {
        if let Some(id) = self.inner.get_line_of_current_cursor().map(|id| id.0) {
            if self.inner.lines.caches[id].loadable() {
                let unloaded = self.inner.lines.caches.remove(id);
                let loaded = unloaded.load_doc();
                self.inner.lines.caches.push(loaded);
                self.sort_caches();
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

    pub fn scroll_text(&mut self) -> &mut Scrollable<PkgDocs> {
        &mut self.inner
    }
}
