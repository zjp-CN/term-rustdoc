use super::TreeLines;
use crate::tree::ID;

/// how to fold the text tree
enum Kind {
    ExpandAll,
    /// Always focus on current module with other modules including parents folded.
    CurrentModule,
    /// Always focus on current module with parents also expanded.
    RememberParentModules,
    /// Firstly, fold all modules. Once a module is opened, remember it till it's closed.
    RememberExpandedModules,
}

/// Fold based on module tree.
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
}
