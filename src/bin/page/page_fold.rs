use super::Page;

/// For now, the expand/fold behavior only works for Module tree.
impl Page {
    pub fn outline_fold_expand_all(&mut self) {
        if !self.outline.is_module_tree() {
            return;
        }
        self.outline().lines.expand_all();
        self.update_after_folding_outline();
    }

    pub fn outline_fold_expand_current_module_only(&mut self) {
        if !self.outline.is_module_tree() {
            return;
        }
        if let Some(id) = self.outline().get_id() {
            self.outline().lines.expand_current_module_only(id);
            self.update_after_folding_outline();
        }
    }

    pub fn outline_fold_expand_zero_level(&mut self) {
        if !self.outline.is_module_tree() {
            return;
        }
        self.outline().lines.expand_zero_level();
        self.update_after_folding_outline();
    }

    pub fn outline_fold_expand_to_first_level_modules(&mut self) {
        if !self.outline.is_module_tree() {
            return;
        }
        self.outline().lines.expand_to_first_level_modules();
        self.update_after_folding_outline();
    }

    pub fn outline_fold_expand_toggle(&mut self) {
        if !self.outline.is_module_tree() {
            return;
        }
        if let Some(id) = self.outline().get_id() {
            self.outline().lines.expand_toggle(id);
            self.update_after_folding_outline();
        }
    }

    fn update_after_folding_outline(&mut self) {
        self.update_area_inner(self.area);

        let outline = self.outline();
        if outline.visible_lines().is_none() {
            // start from the begining if nothing needs to show up
            outline.start = 0;
        }
        // try jumping to previous line
        if !outline.check_if_can_return_to_previous_cursor() {
            // if no previous line is found, jump to the first line
            outline.set_cursor(0);
        }
    }
}
