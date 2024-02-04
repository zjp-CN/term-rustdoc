use super::Page;

impl Page {
    pub fn outline_fold(&mut self) {
        let outline = self.outline();
        if let Some(id) = outline.get_id() {
            outline.lines.expand_current_module_only(id.into());
            outline.update_maxwidth();
            self.update_area_inner(self.area);
            self.outline().check_if_can_return_to_previous_cursor();
        }
    }
}
