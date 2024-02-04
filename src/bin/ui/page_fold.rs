use super::Page;

impl Page {
    pub fn outline_fold(&mut self) {
        if let Some(id) = self.outline().get_id().map(Into::into) {
            self.outline().lines.expand_current_module_only(id);
            self.update_after_folding_outline();
        }
    }

    fn update_after_folding_outline(&mut self) {
        self.outline().update_maxwidth();
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
