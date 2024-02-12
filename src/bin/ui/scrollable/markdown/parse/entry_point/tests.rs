use super::{markdown_iter, parse};
use insta::{assert_debug_snapshot as snap, assert_display_snapshot as shot};

#[test]
fn parse_markdown() {
    let doc = r#"
# h1 `code`

aaa b *c* d **e**. ~xxx~ z.

1 *c **ss** d sadsad xxx* `yyyy`

```
let a = 1;
```

> rrr sss 
> tt

- [x] done!
    - nested list
- [ ] undone
    1. *a*
    2. `b`
"#;
    snap!(markdown_iter(doc).collect::<Vec<_>>());
    let parsed = parse(doc);
    snap!("parse_markdown-parsed", parsed);
    shot!(parsed, @r###"
    # h1 `code`

    aaa b c d e. xxx z.

    1 c ss d sadsad xxx `yyyy`

    let a = 1;

    rrr sss tt

    * [x] done!
      * nested list
    * [ ] undone
      1. a
      2. `b`

    "###);

    let mut lines = Vec::new();
    parsed.write_styled_lines(7.0, &mut lines);
    snap!("parse_markdown-StyledLines", lines);
}

/// This test is used to quickly test text wrapping.
#[test]
fn parse_markdown_dbg() {
    let doc = r#"
"#;
    const WIDTH: f64 = 70.0;
    let mut slines = Vec::new();
    parse(doc).write_styled_lines(WIDTH, &mut slines);
    dbg!(slines);
}

#[test]
fn parse_markdown_links() {
    let doc = "
[a](b), [c], [e][c].

[c]: d

[`f`][c].

## h2 [c] [`h`][c]
";
    snap!(markdown_iter(doc).collect::<Vec<_>>());
    let blocks = parse(doc);
    snap!("parse_markdown_links-parsed", blocks);
    shot!(blocks, @r###"
    a, c, e.

    `f`.

    ## h2 c `h`

    "###);
}

#[test]
fn parse_markdown_intra_code() {
    let doc = "A `code` in a line.";
    dbg!(markdown_iter(doc).collect::<Vec<_>>(), parse(doc));
}
