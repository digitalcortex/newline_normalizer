# newline-normalizer

Convert between newline formats (`\n`, `\r`, `\r\n`) safely and efficiently.

## Features

- Preserves input with `Cow<str>` — avoids allocating when input is already normalized.
- Converts all valid forms of newlines (Unix, DOS, or classic Mac).
- Unicode-safe — preserves emojis, RTL, combining marks, and other sequences.
- Fast scanning using `memchr`.

## Examples

```rust
use newline_normalizer::{ToUnixNewlines, ToDosNewlines};

let unix = "line1\r\nline2\rline3".to_unix_newlines();
assert_eq!(unix, "line1\nline2\nline3");

let dos = "line1\nline2\nline3".to_dos_newlines();
assert_eq!(dos, "line1\r\nline2\r\nline3");
```

## Unicode behavior

This crate **does not alter Unicode content**. It only rewrites newline boundaries.

All valid UTF-8 sequences are preserved, including:

- Combining characters stay attached
- Emoji and multi-codepoint sequences remain valid
- Right-to-left (RTL) markers are unaffected

## Limitations

This crate does not currently normalize U+2028 (LINE SEPARATOR) or U+2029 (PARA SEP). Only ASCII newline formats are converted.