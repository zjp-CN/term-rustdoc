use super::TreeLines;
use crate::tree::{DModule, DocTree, IDMap};
use rustc_hash::FxHashSet as HashSet;
use rustdoc_types::{Id, ItemEnum};

/// how to fold the text tree
#[derive(Default, PartialEq, Eq)]
enum Kind {
    /// Expand all public items in all modules.
    #[default]
    ExpandAll,
    /// Only expand items under root module with sub modules folded.
    ExpandZero,
    /// Expand level zero and one items.
    ///
    /// Level zero refers to items directly under root module.
    ///
    /// Level one refers to items from level-zero modules.
    ///
    /// Items from these two outermost levels may be the most important APIs.
    ExpandToFirstLevelModules,
    /// Always expand and focus on current module.
    ///
    /// NOTE: this allows all (sub)modules to expand (but with non-module items hidden),
    /// because it's helpful for users to not only know the current one, but also quickly
    /// jump into any other one.
    CurrentModule,
}

/// Fold based on module tree.
#[derive(Default)]
pub struct Fold {
    kind: Kind,
    /// module IDs that should be expanded
    expand: HashSet<Id>,
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
        self._expand_all();
        self.lines = self.dmodule().item_tree(self.idmap()).cache_lines().0;
    }

    pub(super) fn _expand_all(&mut self) {
        fn traversal_id(m: &DModule, mods: &mut HashSet<Id>) {
            mods.insert(m.id.clone());
            for submod in &m.modules {
                traversal_id(submod, mods);
            }
        }
        self.fold.kind = Kind::ExpandAll;
        self.fold.expand.clear();
        traversal_id(self.doc().dmodule(), &mut self.fold.expand);
    }

    pub fn expand_zero_level(&mut self) {
        self.fold.kind = Kind::ExpandZero;
        self.fold.expand.clear();
        self.fold.expand.insert(self.dmodule().id.clone());
        self.update_cached_lines(|dmod, map, _| {
            let mut root = dmod.item_tree_only_in_one_specified_mod(map);
            root.extend(
                dmod.modules
                    .iter()
                    .map(|m| node!(ModuleFolded: map, Module, m.id)),
            );
            root
        });
    }

    pub fn expand_to_first_level_modules(&mut self) {
        self.fold.kind = Kind::ExpandToFirstLevelModules;
        let dmod = &self.dmodule().modules;
        self.fold.expand = dmod.iter().map(|m| m.id.clone()).collect();
        self.update_cached_lines(|dmod, map, mods| {
            let mut root = dmod.item_tree_only_in_one_specified_mod(map);
            for m in &dmod.modules {
                let tree = if mods.contains(&m.id) {
                    let mut tree = m.item_tree_only_in_one_specified_mod(map);
                    for submod in &m.modules {
                        let leaf = node!(ModuleFolded: map, Module, submod.id);
                        tree.push(leaf);
                    }
                    tree
                } else {
                    node!(ModuleFolded: map, Module, m.id)
                };
                root.push(tree);
            }
            root
        });
    }
}

impl TreeLines {
    /// Expand a folded module or fold an expanded one.
    ///
    /// This pushs a module ID to a without setting any fold kind.
    pub fn expand_toggle(&mut self, id: Id) {
        fn modules_traversal(
            dmod: &DModule,
            map: &IDMap,
            parent: &mut DocTree,
            should_stop: &mut impl FnMut(&DModule) -> bool,
        ) {
            for m in &dmod.modules {
                if should_stop(m) {
                    let node = node!(ModuleFolded: map, Module, m.id);
                    parent.push(node);
                } else {
                    let mut node = m.item_tree_only_in_one_specified_mod(map);
                    modules_traversal(m, map, &mut node, should_stop);
                    parent.push(node);
                };
            }
        }

        if self.fold.kind == Kind::CurrentModule {
            // FIXME: poor interaction with CurrentModule bahavior
            //
            // To fix this, we have to remember all the modules' id and an extra
            // expand-vs-fold state.
            //
            // This is a smallUX improvement, but for now, just forbid toggling
            // when expanding CurrentModule.
            return;
        }

        if !self.check_id(&id) {
            return;
        }
        let mods = &mut self.fold.expand;
        if mods.contains(&id) {
            mods.remove(&id);
        } else {
            mods.insert(id);
        }
        self.update_cached_lines(|dmod, map, mods| {
            let mut root = dmod.item_tree_only_in_one_specified_mod(map);
            modules_traversal(dmod, map, &mut root, &mut |m| !mods.contains(&m.id));
            root
        });
    }
}

impl TreeLines {
    pub fn expand_current_module_only(&mut self, id: Id) {
        self.fold.kind = Kind::CurrentModule;
        if !self.check_id(&id) {
            return;
        }
        self.fold.expand.clear();
        self.fold.expand.insert(id);
        self._expand_current_module_only();
    }

    fn _expand_current_module_only(&mut self) {
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
                    node.extend(
                        m.modules
                            .iter()
                            .map(|m| node!(@name ModuleFolded: map, m.id)),
                    );
                    parent.push(node);
                    // NOTE: Stop traverlling down inside but still travel in other modules.
                    // This is because it's not helpful to only show/know target modules.
                } else {
                    // use short name for non-target modules
                    let mut node = node!(@name ModuleFolded: map, m.id);
                    modules_traversal(m, map, &mut node, should_stop);
                    parent.push(node);
                };
            }
        }
        self.update_cached_lines(|dmod, map, mods| {
            let mut root = node!(Module: map, dmod.id);
            if mods.contains(&dmod.id) {
                // item tree already contains the root module, thus only mv the leaves
                let iter = dmod.item_tree_only_in_one_specified_mod(map).tree.leaves;
                root.tree.extend(iter);
            }
            modules_traversal(dmod, map, &mut root, &mut |m| mods.contains(&m.id));
            root
        });
    }
}

impl TreeLines {
    /// check if the id is a module (or reexported as module)
    fn check_id(&self, id: &Id) -> bool {
        if !self.dmodule().modules.iter().any(|_m| {
            // only Module or reexported item as Module can be in list
            self.idmap()
                .get_item(id)
                .map(|item| match &item.inner {
                    ItemEnum::Module(_) => true,
                    ItemEnum::Use(reepxort) => {
                        if let Some(id) = &reepxort.id {
                            if let Some(item) = self.idmap().get_item(id) {
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
                "{id:?} is not a non-module item `{}` {:?}",
                self.idmap().name(&id),
                self.idmap().get_item(id)
            );
            return false;
        }
        true
    }

    fn update_cached_lines(&mut self, f: impl FnOnce(&DModule, &IDMap, &HashSet<Id>) -> DocTree) {
        let map = self.idmap();
        let dmod = &self.dmodule();
        let mods = &self.fold.expand;
        if mods.is_empty() {
            // if no mods are sepecified, default to expand all
            self.lines = self.dmodule().item_tree(map).cache_lines().0;
            return;
        }
        let root = f(dmod, map, mods);
        self.lines = root.cache_lines().0;
    }
}
