use std::ops::Deref;
use term_rustdoc::{tree::TreeLine, util::XString};

pub trait Lines: Deref<Target = [Self::Line]> {
    type Line: LineState;
}

impl<L: LineState, Ls: Deref<Target = [L]>> Lines for Ls {
    type Line = L;
}

pub trait LineState {
    type State: PartialEq + Default;
    fn state(&self) -> Self::State;
    fn is_identical(&self, state: &Self::State) -> bool;
}

impl LineState for TreeLine {
    type State = Option<XString>;

    fn state(&self) -> Self::State {
        self.id.clone()
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        self.id == *state
    }
}
