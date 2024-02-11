use crate::snap;
use syntect::parsing::SyntaxSet;

#[test]
fn syntax_set() {
    let set = SyntaxSet::load_defaults_newlines();
    let v = set
        .syntaxes()
        .iter()
        .map(|s| (&s.name, &s.file_extensions))
        .collect::<Vec<_>>();
    snap!(v);
}
