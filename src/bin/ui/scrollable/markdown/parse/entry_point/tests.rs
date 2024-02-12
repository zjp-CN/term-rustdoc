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
    shot!(parse(doc), @r###"
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
    shot!(parse(doc), @r###"
    a, c, e.

    `f`.

    ## h2 c `h`

    "###);
}
