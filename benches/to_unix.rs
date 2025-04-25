use criterion::{criterion_group, criterion_main, Criterion, black_box};
use newline_normalizer::ToUnixNewlines;

fn bench_to_unix_newlines(c: &mut Criterion) {
    let input = "
      Это пример параграфа с пробелами и юникодом.\r\n
    Он содержит строки на русском языке, немного английского, и даже: こんにちは世界！\r

    Here's a sentence with normal ASCII characters, leading spaces, and symbols: @$%&.\r

            مرحبا بك في عالم الترميز الموحد.
    ".to_string();

    let pre_normalized_input = input.to_unix_newlines().to_string();

    c.bench_function("std lib: chained replace", |b| {
        b.iter(|| str::replace(black_box(&input), "\r\n", "\n").replace("\r", "\n"))
    });

    c.bench_function("3rd party crate \"newline-converter\": dos2unix()", |b| {
        b.iter(|| newline_converter::dos2unix(black_box(&input)))
    });

    c.bench_function("3rd party crate \"newline-converter\": dos2unix() with pre-normalized text", |b| {
        b.iter(|| newline_converter::dos2unix(black_box(&pre_normalized_input)))
    });
    
    c.bench_function("this crate: to_unix_newlines()", |b| {
        let input_slice = input.as_str();
        b.iter(|| newline_normalizer::ToUnixNewlines::to_unix_newlines(black_box(input_slice)))
    });

    c.bench_function("this crate: to_unix_newlines() with pre-normalized text", |b| {
        let input_slice = pre_normalized_input.as_str();
        b.iter(|| newline_normalizer::ToUnixNewlines::to_unix_newlines(black_box(input_slice)))
    });
}

criterion_group!(benches, bench_to_unix_newlines);
criterion_main!(benches);

