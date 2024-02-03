use super::TreeLines;
use crate::tree::{DModule, IDMap, ID};

/// how to fold the text tree
#[derive(Default)]
enum Kind {
    /// Expand all public items in all modules.
    #[default]
    ExpandAll,
    /// Always focus on current module with other modules including parents folded.
    CurrentModule,
    /// Always focus on current module with parents also expanded.
    RememberParentModules,
    /// Firstly, fold all modules. Once a module is opened, remember it till it's closed.
    RememberExpandedModules,
}

/// Fold based on module tree.
#[derive(Default)]
pub struct Fold {
    kind: Kind,
    /// module IDs that should be expanded
    mods: Vec<ID>,
}

// ─➤  ─⮞ ─▶ ▶

/// Fold a tree.
impl TreeLines {
    /// Expand all module items including data structure's impl and trait's implementors.
    pub fn expand_all_including_impls(&mut self) {
        *self = Self::new_with(self.doc(), |doc| doc.dmodule_show_prettier()).0;
    }

    pub fn dmodule(&self) -> &DModule {
        self.doc.dmodule()
    }

    pub fn idmap(&self) -> &IDMap {
        &self.doc
    }

    pub fn expand_all(&mut self) {
        self.fold.kind = Kind::ExpandAll;
        let dmod = &self.dmodule().modules;
        self.fold.mods = dmod.iter().map(|m| m.id.clone()).collect();
        self.update_cached_lines();
    }

    pub fn expand_current_module_only(&mut self, id: ID) {
        self.fold.kind = Kind::CurrentModule;
        if !self.dmodule().modules.iter().any(|m| m.id == id) {
            error!(
                "ID({id}) is not a non-module item `{}`",
                self.idmap().name(&id)
            );
            return;
        }
        self.fold.mods.clear();
        self.fold.mods.push(id);
        self.update_cached_lines();
    }

    fn update_cached_lines(&mut self) {
        let map = self.idmap();
        let dmod = &self.dmodule();
        let mods = &self.fold.mods;
        if mods.is_empty() {
            // if no mods are sepecified, default to expand all
            self.lines = self.dmodule().item_tree(map).cache_lines().0;
            return;
        }
        let mut root = node!(Module: map, &dmod.id);
        for m in dmod.modules.iter() {
            let tree = if mods.contains(&m.id) {
                m.item_tree_only_in_one_specified_mod(map)
            } else {
                node!(ModuleFolded: map, Module, &m.id)
            };
            root.tree.push(tree.tree);
        }
        self.lines = root.cache_lines().0;
    }
}
