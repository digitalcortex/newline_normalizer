# newline-normalizer

Convert between newline formats (`\n`, `\r`, `\r\n`) safely and efficiently.

## Features

- Adds extension traits to `str` — call `.to_unix_newlines()` and `.to_dos_newlines()` naturally.
- Preserves input with `Cow<str>` — avoids allocating when input is already normalized.
- Converts all common newline formats (`\n`, `\r`, `\r\n`) including classic Mac line endings.
- Unicode-safe — preserves emojis, RTL, combining marks, and other sequences.
- Fast scanning using [memchr](https://github.com/BurntSushi/memchr) utilizing SIMD for efficient large text processing.

## Benchmark

Benchmarks are located under the `/benches` folder.

Run them using:
```
cargo bench --bench to_unix
cargo bench --bench to_dos
```

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