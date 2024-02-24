use itertools::Itertools;
use nucleo_matcher::{
    pattern::{Atom, CaseMatching, Normalization},
    *,
};
use std::{cell::RefCell, rc::Rc};

/// A shared fuzzy matcher that only stores the source text for computing its score
/// and is used as Atom pattern.
///
/// This is cheap to clone, and appropriate for small set of source texts like a list
/// of short texts, lines in a page of documentation etc, in which case ascii conversion
/// for `Utf32Str` is free and less frequently to fuzz.
#[derive(Clone)]
pub struct Fuzzy {
    fuzzy: Rc<RefCell<FuzzyInner>>,
}

impl Fuzzy {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Fuzzy {
            fuzzy: Rc::new(RefCell::new(FuzzyInner::new())),
        }
    }

    fn fuzzy<T>(&self, f: impl FnOnce(&mut FuzzyInner) -> T) -> Option<T> {
        self.fuzzy.try_borrow_mut().ok().as_deref_mut().map(f)
    }

    pub fn parse(&self, pattern: &str) {
        self.fuzzy(|f| f.parse(pattern));
    }

    pub fn score(&self, text: &str) -> Option<u16> {
        self.fuzzy(|f| f.score(text)).flatten()
    }

    pub fn match_list<T: AsRef<str>, U: From<T>>(
        &self,
        texts: impl IntoIterator<Item = T>,
        buf: &mut Vec<U>,
    ) {
        self.fuzzy(|f| f.match_list(texts, buf));
    }
}

struct FuzzyInner {
    /// non-ascii string buffer for `Utf32Str`
    buf: Vec<char>,
    pat: Atom,
    matcher: Matcher,
}

impl FuzzyInner {
    fn new() -> Self {
        FuzzyInner {
            buf: Vec::new(),
            pat: Atom::parse("", CaseMatching::Smart, Normalization::Smart),
            matcher: Matcher::new(Config::DEFAULT),
        }
    }

    fn parse(&mut self, pattern: &str) {
        self.pat = Atom::parse(pattern, CaseMatching::Smart, Normalization::Smart);
    }

    fn score(&mut self, source_text: &str) -> Option<u16> {
        let text = Utf32Str::new(source_text, &mut self.buf);
        self.pat.score(text, &mut self.matcher)
    }

    fn match_list<T: AsRef<str>, U: From<T>>(
        &mut self,
        texts: impl IntoIterator<Item = T>,
        buf: &mut Vec<U>,
    ) {
        let output = texts
            .into_iter()
            .filter_map(|t| self.score(t.as_ref()).filter(|x| *x > 0).map(|x| (x, t)))
            .sorted_unstable_by_key(|val| std::cmp::Reverse(val.0));
        buf.clear();
        buf.extend(output.map(|(_, t)| t.into()));
    }
}
