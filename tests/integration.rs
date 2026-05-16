use html_entity_fix::fix;

#[test]
fn decodes_named_entities() {
    assert_eq!(fix("AT&amp;T"), "AT&T");
    assert_eq!(fix("&lt;a&gt;"), "<a>");
    assert_eq!(fix("she said &quot;hi&quot;"), "she said \"hi\"");
    assert_eq!(fix("don&apos;t"), "don't");
}

#[test]
fn decodes_decimal_numeric() {
    assert_eq!(fix("&#39;hello&#39;"), "'hello'");
    assert_eq!(fix("&#8211;"), "–");
}

#[test]
fn decodes_hex_numeric() {
    assert_eq!(fix("&#x27;"), "'");
    assert_eq!(fix("&#X27;"), "'");
}

#[test]
fn decodes_nbsp_and_typography() {
    assert_eq!(fix("a&nbsp;b"), "a\u{00A0}b");
    assert_eq!(fix("&hellip;"), "…");
    assert_eq!(fix("&mdash;"), "—");
    assert_eq!(fix("&ldquo;hi&rdquo;"), "“hi”");
}

#[test]
fn unknown_entity_passes_through() {
    assert_eq!(fix("&foo;"), "&foo;");
}

#[test]
fn passes_plain_text() {
    assert_eq!(fix("no entities here"), "no entities here");
}

#[test]
fn preserves_unicode() {
    assert_eq!(fix("café résumé"), "café résumé");
}

#[test]
fn open_amp_without_semi_passes() {
    assert_eq!(fix("rock & roll"), "rock & roll");
}
