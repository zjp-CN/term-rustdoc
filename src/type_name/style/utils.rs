use super::{Punctuation, StyledType, Tag};
use itertools::intersperse;

pub fn write_colon(buf: &mut StyledType) {
    buf.write(Punctuation::Colon);
}

pub fn write_plus(buf: &mut StyledType) {
    buf.write(Punctuation::Plus);
}

pub fn write_comma(buf: &mut StyledType) {
    buf.write(Punctuation::Comma);
}

// pub fn write_nothing(_: &mut StyledType) {}

impl StyledType {
    /// Write a colon and bounds concatenated by `+`.
    /// Make sure the iter is non-empty, because this function writes contents anyway.
    pub(super) fn write_bounds<T>(&mut self, iter: impl IntoIterator<Item = T>)
    where
        Tag: From<T>,
    {
        self.write(Punctuation::Colon);
        let iterable = iter.into_iter().map(Tag::from);
        for tag in intersperse(iterable, Punctuation::Plus.into()) {
            self.write(tag);
        }
    }

    /// Write multiple `repeat` separated by `sep` if slice is not empty.
    /// Won't write anything if slice is empty.
    ///
    /// Sometimes slice length check is still done before calling this method,
    /// say a slice of generic parameter bound needs an extra starting colon and
    /// angle brackes if it's non-empty, but does not need them if empty.
    pub(super) fn write_slice<T>(
        &mut self,
        slice: &[T],
        repeat: impl Fn(&T, &mut Self),
        sep: impl Fn(&mut Self),
    ) {
        let mut iter = slice.iter();
        let Some(t) = iter.next() else {
            return;
        };
        repeat(t, self);
        for t in iter {
            sep(self);
            repeat(t, self);
        }
    }
}
