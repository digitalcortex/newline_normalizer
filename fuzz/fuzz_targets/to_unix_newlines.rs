#![no_main]
extern crate newline_normalizer;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let lf = newline_normalizer::ToUnixNewlines::to_unix_newlines(s);
    }
});
