//! # html-entity-fix
//!
//! Decode HTML entities (`&amp;`, `&lt;`, `&gt;`, `&quot;`, `&#39;`,
//! `&#x27;`, etc.) inside text that was supposed to be plain.
//!
//! LLMs sometimes emit HTML-escaped text into JSON or chat output —
//! usually because they over-corrected from a prior HTML context.
//! This crate decodes the common cases without pulling in a full
//! HTML parser.
//!
//! ## Example
//!
//! ```
//! use html_entity_fix::fix;
//! assert_eq!(fix("AT&amp;T"), "AT&T");
//! assert_eq!(fix("&lt;tag&gt;"), "<tag>");
//! assert_eq!(fix("&#39;hello&#39;"), "'hello'");
//! ```

#![deny(missing_docs)]

/// Decode named and numeric HTML entities in `s`. Unknown entities are
/// passed through unchanged.
pub fn fix(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'&' {
            if let Some((decoded, len)) = decode_entity(&s[i..]) {
                out.push(decoded);
                i += len;
                continue;
            }
        }
        // Safe to push a single byte directly only if it's ASCII; we
        // fall back to taking one char in the general case so multi-byte
        // UTF-8 sequences stay intact.
        if bytes[i] < 0x80 {
            out.push(bytes[i] as char);
            i += 1;
        } else {
            let c = s[i..].chars().next().unwrap();
            out.push(c);
            i += c.len_utf8();
        }
    }
    out
}

/// Returns `Some((decoded_char, bytes_consumed))` on a successful decode
/// starting at `s` (which must begin with `&`).
fn decode_entity(s: &str) -> Option<(char, usize)> {
    let bytes = s.as_bytes();
    if bytes.first() != Some(&b'&') {
        return None;
    }
    let semi = s[1..].find(';')?;
    let inside = &s[1..1 + semi];
    let consumed = semi + 2; // & ... ;

    // Numeric: &#NNN; or &#xHH;
    if let Some(rest) = inside.strip_prefix('#') {
        let code = if let Some(hex) = rest.strip_prefix('x').or(rest.strip_prefix('X')) {
            u32::from_str_radix(hex, 16).ok()?
        } else {
            rest.parse::<u32>().ok()?
        };
        let c = char::from_u32(code)?;
        return Some((c, consumed));
    }

    let c = match inside {
        "amp" => '&',
        "lt" => '<',
        "gt" => '>',
        "quot" => '"',
        "apos" => '\'',
        "nbsp" => '\u{00A0}',
        "copy" => '©',
        "reg" => '®',
        "trade" => '™',
        "hellip" => '…',
        "mdash" => '—',
        "ndash" => '–',
        "lsquo" => '‘',
        "rsquo" => '’',
        "ldquo" => '“',
        "rdquo" => '”',
        _ => return None,
    };
    Some((c, consumed))
}
