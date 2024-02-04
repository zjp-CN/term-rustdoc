use rustdoc_types::ItemEnum;

use super::TreeLines;
use crate::tree::{DModule, DocTree, IDMap, ID};

/// how to fold the text tree
#[derive(Default)]
enum Kind {
    /// Expand all public items in all modules.
    #[default]
    ExpandAll,
    /// Expand level zero and one items.
    ///
    /// Level zero refers to items directly under root module.
    ///
    /// Level one refers to items from level-zero modules.
    ///
    /// Items from these two outermost levels may be the most important APIs.
    ExpandToFirstLevelModules,
    /// Always focus on current module.
    ///
    /// NOTE: this allows all (sub)modules to expand (but with non-module items hidden),
    /// because it's helpful for users to not only know the current one, but also quickly
    /// jump into any other one.
    CurrentModule,
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
        self.fold.mods.clear();
        self.lines = self.dmodule().item_tree(self.idmap()).cache_lines().0;
    }

    pub fn expand_first_level_modules_only(&mut self) {
        self.fold.kind = Kind::ExpandToFirstLevelModules;
        let dmod = &self.dmodule().modules;
        self.fold.mods = dmod.iter().map(|m| m.id.clone()).collect();
        self.update_cached_lines(|dmod, map, mods| {
            let mut root = dmod.item_tree_only_in_one_specified_mod(map);
            for m in &dmod.modules {
                let tree = if mods.contains(&m.id) {
                    let mut tree = m.item_tree_only_in_one_specified_mod(map);
                    for submod in &m.modules {
                        let leaf = node!(ModuleFolded: map, Module, &submod.id);
                        tree.push(leaf);
                    }
                    tree
                } else {
                    node!(ModuleFolded: map, Module, &m.id)
                };
                root.push(tree);
            }
            root
        });
    }

    pub fn expand_current_module_only(&mut self, id: ID) {
        self.fold.kind = Kind::CurrentModule;
        if !self.dmodule().modules.iter().any(|m| {
            // only Module or reexported item as Module can be in list
            self.idmap()
                .get_item(&id)
                .map(|item| match &item.inner {
                    ItemEnum::Module(_) => true,
                    ItemEnum::Import(reepxort) => {
                        if let Some(id) = &reepxort.id {
                            if let Some(item) = self.idmap().get_item(&id.0) {
                                return matches!(item.inner, ItemEnum::Module(_));
                            }
                        }
                        false
                    }
                    _ => false,
                })
                .unwrap_or(false)
        }) {
            error!(
                "ID({id}) is not a non-module item `{}` {:?}",
                self.idmap().name(&id),
                self.idmap().get_item(&id)
            );
            return;
        }
        self.fold.mods.clear();
        self.fold.mods.push(id);
        self.update_cached_lines(|dmod, map, mods| {
            let mut root = node!(Module: map, &dmod.id);
            modules_traversal(dmod, map, &mut root, &mut |m| mods.contains(&m.id));
            root
        });
    }

    fn update_cached_lines(&mut self, f: impl FnOnce(&DModule, &IDMap, &[ID]) -> DocTree) {
        let map = self.idmap();
        let dmod = &self.dmodule();
        let mods = &self.fold.mods;
        if mods.is_empty() {
            // if no mods are sepecified, default to expand all
            self.lines = self.dmodule().item_tree(map).cache_lines().0;
            return;
        }
        let root = f(dmod, map, mods);
        self.lines = root.cache_lines().0;
    }
}

fn modules_traversal(
    dmod: &DModule,
    map: &IDMap,
    parent: &mut DocTree,
    should_stop: &mut impl FnMut(&DModule) -> bool,
) {
    for m in &dmod.modules {
        if should_stop(m) {
            // use long path because it's helpful to instantly know where it is
            let mut node = m.item_tree_only_in_one_specified_mod(map);
            parent.push(node);
            // NOTE: Stop traverlling down inside but still travel in other modules.
            // This is because it's not helpful to only show/know target modules.
        } else {
            // use short name for non-target modules
            let mut node = node!(@name ModuleFolded: map, &m.id);
            modules_traversal(m, map, &mut node, should_stop);
            parent.push(node);
        };
    }
}
