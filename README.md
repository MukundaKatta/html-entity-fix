# html-entity-fix

[![crates.io](https://img.shields.io/crates/v/html-entity-fix.svg)](https://crates.io/crates/html-entity-fix)

Decode HTML entities (`&amp;`, `&lt;`, `&gt;`, `&quot;`, `&#39;`,
`&#x27;`, common typography) inside text that was supposed to be plain.
Catches LLMs that over-correct out of an HTML context.

```rust
use html_entity_fix::fix;
assert_eq!(fix("AT&amp;T"), "AT&T");
assert_eq!(fix("&lt;tag&gt;"), "<tag>");
```

Zero deps. MIT or Apache-2.0.
