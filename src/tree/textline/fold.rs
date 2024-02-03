use super::{Rc, Tag, TreeLine, TreeLines};
use crate::tree::DocTree;

/// how to fold the text tree
enum Kind {
    /// fold everything except this item type
    ///
    /// useful to only list an item type in all nested modules.
    ///
    /// Maybe we should support multiple tags too?
    Tag(Tag),
    /// fold everything except a bunch of item types
    ///
    /// Useful for displaying nested items but with less details
    /// * for Tag::Modules, display modules tree and normal Rust items
    /// * for Tag::Structs, display all structs in all nested modules and fields
    /// * etc
    TagsInCharge(Tag),
    /// fold everything except all the items in the module where current cursor lies
    CurrentModule,
    /// fold everything except normal Rust items in the module where current cursor lies
    ///
    /// Note that for simplicity, this also filters out details (fields/impls/associated items...)
    /// in data structures types.
    CurrentModuleConsise,
    // fold what is filtered out
    // Search,
}

pub struct Fold {
    kind: Kind,
}

/// Fold a tree.
impl TreeLines {
    /// Expand with the most complete view.
    pub fn expand_all(&mut self) {
        *self = Self::new_with(self.doc(), |doc| doc.dmodule_show_prettier()).0;
    }

    pub fn fold_by_tag(&mut self, tag: Tag) {
        match tag {
            Tag::Module => {}
            _ => warn!(?tag, "not supported in outline's fold_by_tag"),
        };
        self.fold = Some(Fold {
            kind: Kind::Tag(tag),
        });
    }

    pub fn fold_by_tag_in_charge(&mut self, tag: Tag) {
        self.fold = Some(Fold {
            kind: Kind::Tag(tag),
        });
    }
}
