use super::LoadedDoc;
use crate::database::{CachedDocInfo, PkgKey};
use ratatui::prelude::{Color, Style};
use std::time::SystemTime;

pub enum CacheInner {
    /// cached & loaded pkg docs
    Loaded(LoadedDoc),
    /// cached but not loaded docs
    Unloaded(CachedDocInfo),
    /// pkgs which is being sent to compile doc
    BeingCached(PkgKey, SystemTime),
}

impl CacheInner {
    pub fn pkg_key(&self) -> &PkgKey {
        match self {
            CacheInner::Loaded(load) => &load.info.pkg,
            CacheInner::Unloaded(unload) => &unload.pkg,
            CacheInner::BeingCached(pk, _) => pk,
        }
    }

    pub fn kind(&self) -> (&'static str, Style) {
        match self {
            CacheInner::Loaded(_) => ("[Loaded]", Style::new()),
            CacheInner::Unloaded(_) => (
                "[Cached]",
                Style {
                    fg: Some(Color::from_u32(0x00A39900)), // #A39900
                    ..Style::new()
                },
            ),
            CacheInner::BeingCached(_, _) => (
                "[HoldOn]",
                Style {
                    fg: Some(Color::LightMagenta),
                    ..Style::new()
                },
            ),
        }
    }
}

impl Eq for CacheInner {}
impl PartialEq for CacheInner {
    /// Only use PkgKey to compare if both are equal.
    fn eq(&self, other: &Self) -> bool {
        self.pkg_key() == other.pkg_key()
    }
}
