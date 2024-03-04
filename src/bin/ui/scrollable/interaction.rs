use super::{LineState, Lines, Scroll};
use ratatui::prelude::Rect;

/// Scroll by fixed rows or half/full screen
pub enum ScrollOffset {
    Fixed(usize),
    HalfScreen,
    FullScreen,
}

/// Trait object that is used when a widget containing multiple scrollable components
/// needs to unifiy the behavior of scrolling or moving the cursor.
pub trait Scrollable {
    fn scroll_down(&mut self, offset: ScrollOffset);
    fn scroll_up(&mut self, offset: ScrollOffset);
    fn scroll_home(&mut self);
    fn scroll_end(&mut self);
    fn move_forward_cursor(&mut self);
    fn move_backward_cursor(&mut self);
    fn move_top_cursor(&mut self);
    fn move_bottom_cursor(&mut self);
    fn move_middle_cursor(&mut self);
    fn set_cursor(&mut self, y: u16);
    fn area(&self) -> Rect;
    /// position in (x, y)
    fn contains(&mut self, position: (u16, u16)) -> bool {
        self.area().contains(position.into())
    }
}

impl<Ls: Lines> Scrollable for Scroll<Ls> {
    fn scroll_down(&mut self, offset: ScrollOffset) {
        self.scroll_down(offset);
    }

    fn scroll_up(&mut self, offset: ScrollOffset) {
        self.scroll_up(offset);
    }

    fn scroll_home(&mut self) {
        self.scroll_home();
    }

    fn scroll_end(&mut self) {
        self.scroll_end();
    }

    fn move_forward_cursor(&mut self) {
        self.move_forward_cursor();
    }

    fn move_backward_cursor(&mut self) {
        self.move_backward_cursor();
    }

    fn move_top_cursor(&mut self) {
        self.move_top_cursor();
    }

    fn move_bottom_cursor(&mut self) {
        self.move_bottom_cursor();
    }

    fn move_middle_cursor(&mut self) {
        self.move_middle_cursor();
    }

    fn set_cursor(&mut self, y: u16) {
        self.set_cursor(y);
    }

    fn area(&self) -> Rect {
        self.area
    }
}

/// Scrolling
impl<Ls: Lines> Scroll<Ls> {
    pub fn scroll_down(&mut self, offset: ScrollOffset) {
        let height = self.area.height as usize;
        let len = self.total_len();
        let nrows = match offset {
            ScrollOffset::Fixed(n) => n,
            ScrollOffset::HalfScreen => height.saturating_sub(1) / 2,
            ScrollOffset::FullScreen => height,
        };
        let maybe_outside = self.start + nrows;
        // don't let the last row leave the bottom
        if maybe_outside > len {
            return;
        }
        // set new positions for first row to be displayed
        self.set_cursor_state();
        self.start = (self.start + nrows).min(len.saturating_sub(height));
        self.check_if_can_return_to_previous_cursor();
    }

    pub fn scroll_up(&mut self, offset: ScrollOffset) {
        let height = self.area.height as usize;
        let nrows = match offset {
            ScrollOffset::Fixed(n) => n,
            ScrollOffset::HalfScreen => height.saturating_sub(1) / 2,
            ScrollOffset::FullScreen => height,
        };
        // set new positions for first row to be displayed
        self.set_cursor_state();
        self.start = self.start.saturating_sub(nrows);
        self.check_if_can_return_to_previous_cursor();
    }

    pub fn scroll_home(&mut self) {
        self.start = 0;
        self.cursor.y = 0;
        self.set_cursor_state();
    }

    pub fn scroll_end(&mut self) {
        let height = self.area.height;
        let heightu = height as usize;
        let len = self.total_len();
        self.start = len.saturating_sub(heightu);
        let bot = if len < heightu { len as u16 } else { height };
        self.cursor.y = bot.saturating_sub(1);
        self.set_cursor_state();
    }
}

/// Cursor state
impl<Ls: Lines> Scroll<Ls> {
    /// Check the cursor position and its state after redrawing.
    ///
    /// NOTE: this should always be called when the length of lines is changed,
    /// because cursor will be out of scope if not check or set the cursor.
    ///
    /// This also moves the cursor in two ways:
    /// * coerce the cursor to the last line if it lies outside the line length
    /// * coerce the cursor to the bottom line if it lies outside the height
    /// * coerce the cursor to last position if the screen contains the previous state
    pub fn check_if_can_return_to_previous_cursor(&mut self) -> bool {
        let height = self.area.height;
        let maximum = self.total_len().try_into().unwrap_or(height).min(height);
        if self.cursor.y >= maximum {
            self.cursor.y = maximum.saturating_sub(1);
        }
        if let Some(lines) = self.visible_lines() {
            if let Some(pos) = lines
                .iter()
                .enumerate()
                .find_map(|(pos, line)| line.is_identical(&self.cursor.state).then_some(pos))
            {
                self.cursor.y = pos as u16;
                return true;
            }
        }
        false
    }

    /// Remember the cursor postion which may be called after cursor movement or before scrolling.
    fn set_cursor_state(&mut self) {
        if let Some(l) = self.get_line_of_current_cursor() {
            self.cursor.state = l.state();
        } else if let Some(last) = self.all_lines().last() {
            // the cursor is beyond the scope, thus set it to the last line
            self.cursor.state = last.state();
            self.cursor.y = self.total_len().saturating_sub(1) as u16;
        } // still possible to see empty lines
          // (like forget to reset or check_if_can_return_to_previous_cursor
          // when getting empty result)
    }
}

/// Cursor movement
impl<Ls: Lines> Scroll<Ls> {
    pub fn move_forward_cursor(&mut self) {
        let height = self.area.height;
        let reach_sceen_bottom = (self.cursor.y + 1) == height;
        if reach_sceen_bottom {
            // scroll down for a new line
            self.scroll_down(ScrollOffset::Fixed(1));
        }
        // still be careful to cross the screen's bottom
        self.cursor.y += if (self.cursor.y + 1) == height { 0 } else { 1 };
        self.set_cursor_state();
    }

    pub fn move_backward_cursor(&mut self) {
        if self.cursor.y == 0 {
            // scroll up for a new line
            self.scroll_up(ScrollOffset::Fixed(1));
        }
        // still be careful to cross the screen's top
        self.cursor.y = self.cursor.y.saturating_sub(1);
        self.set_cursor_state();
    }

    pub fn move_top_cursor(&mut self) {
        self.cursor.y = 0;
        self.set_cursor_state();
    }

    pub fn move_bottom_cursor(&mut self) {
        self.cursor.y = (self.area.height as usize)
            .min(self.total_len())
            .saturating_sub(1) as u16;
        self.set_cursor_state();
    }

    pub fn move_middle_cursor(&mut self) {
        self.move_bottom_cursor();
        self.cursor.y /= 2;
        self.set_cursor_state();
    }

    pub fn set_cursor(&mut self, y: u16) {
        if y < self.area.height && (y as usize) < self.all_lines().len() {
            self.cursor.y = y;
            self.set_cursor_state();
        }
    }
}
