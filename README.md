# 🧹 newline-normalizer 🧹

Rust crate for normalizing text into Unix (`\n`) or DOS (`\r\n`) newline formats, using fast SIMD search and zero-copy when possible.

## ✨ Features

- Adds extension traits to `str` — call `.to_unix_newlines()` and `.to_dos_newlines()` directly.
- Preserves input with `Cow<str>` — skips allocation if no changes are needed.
- Converts `\r` and `\r\n` into consistent Unix (`\n`) or DOS (`\r\n`) newlines.
- Unicode-safe — preserves all characters without loss.
- Fast scanning with [memchr](https://github.com/BurntSushi/memchr) and SIMD.

## 📚 Examples

```rust
use newline_normalizer::{ToUnixNewlines, ToDosNewlines};

let unix = "line1\r\nline2\rline3".to_unix_newlines();
assert_eq!(unix, "line1\nline2\nline3");

let dos = "line1\nline2\nline3".to_dos_newlines();
assert_eq!(dos, "line1\r\nline2\r\nline3");
```

## 🚀 Benchmark

Benchmarks are in the `/benches` folder.

Run them using:
```
cargo bench --bench to_unix
cargo bench --bench to_dos
```

All suggestions on how to improve the benchmarks are welcome.

### 📈 Results

Hardware: AMD Ryzen 9 9900X 12-Core Processor with 64 GB RAM.

#### Normalizing to DOS newlines (`\r\n`):

| Case | `newline-converter` | This crate (`newline_normalizer`) |
| ---- | ----------------- | --------------------------------|
Small Unicode paragraph | ~685.46 ns | ~88.789 ns 🚀
Small Unicode paragraph pre-normalized | ~151.39 ns | ~58.350 ns 🚀
The Adventures of Sherlock Holmes (608kb) | ~345.27 µs | ~138.26 µs 🚀
The Adventures of Sherlock Holmes (608kb) pre-normalized | ~342.91 µs | ~137.54 µs 🚀

Note: Pre-normalized means the input already has correct line endings and does not require changes.

#### Normalizing to Unix newlines (`\n`):

| Case | `newline-converter` | `std` replace chain | This crate (`newline_normalizer`) |
| ---- | ----------------- | ----------------- | --------------------------------|
Small Unicode paragraph | ~1.1009 µs | ~140.72 ns | ~24.464 ns 🚀 | 
Small Unicode paragraph pre-normalized | ~203.24 ns | ~109.17 ns | ~4.6608 ns 🚀
The Adventures of Sherlock Holmes (608kb) | ~779.06 µs | ~213.23 µs | ~89.150 µs 🚀
The Adventures of Sherlock Holmes (608kb) pre-normalized | ~365.45 µs | ~137.74 µs | ~2.7538 µs 🚀

#### Benchmark result notes

- Pre-normalized means the input text already uses the correct line endings.
- In such cases, `newline_normalizer` can skip allocations and return a borrowed reference.
- Extremely low latency (e.g., ~4.66 ns) is achieved by using `Cow::Borrowed`, avoiding an allocation of a new string when the input does not change.

## 🔤 Unicode behavior

This crate **does not alter Unicode content**. It only rewrites newline boundaries.

All valid UTF-8 sequences are preserved, including:

- Combining characters stay attached
- Emoji and multi-codepoint sequences remain valid
- Right-to-left (RTL) markers are unaffected

## ⚠️ Limitations

This crate does not currently normalize U+2028 (LINE SEPARATOR) or U+2029 (PARA SEP). Only ASCII newline formats are converted.

## 📝 Licensed under MIT

This project is licensed under the MIT License.