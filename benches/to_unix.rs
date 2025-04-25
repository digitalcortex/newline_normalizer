use criterion::{criterion_group, criterion_main, Criterion, black_box};
use newline_normalizer::{ToDosNewlines, ToUnixNewlines};

fn bench_to_unix_newlines(c: &mut Criterion) {
    let input = "
      Это пример параграфа с пробелами и юникодом.\r\n
    Он содержит строки на русском языке, немного английского, и даже: こんにちは世界！\r

    Here's a sentence with normal ASCII characters, leading spaces, and symbols: @$%&.\r

            مرحبا بك في عالم الترميز الموحد.
    ".to_string();
    let pre_normalized_input = input.to_unix_newlines().to_string();
    assert_eq!(input.to_unix_newlines(), newline_converter::dos2unix(&input));

    let large_input = include_str!("./files/sherlock.txt").to_dos_newlines().to_string(); // input text has all new lines in Unix format, so we have to convert it to DOS format first for the sake of the benchmark
    let pre_normalized_large_input = large_input.to_unix_newlines().to_string();

    assert_eq!(large_input.to_unix_newlines(), newline_converter::dos2unix(&large_input));
    

    c.bench_function("std lib: chained replace", |b| {
        b.iter(|| str::replace(black_box(&input), "\r\n", "\n").replace("\r", "\n"))
    });

    c.bench_function("std lib: chained replace with pre-normalized text", |b| {
        b.iter(|| str::replace(black_box(&pre_normalized_input), "\r\n", "\n").replace("\r", "\n"))
    });

    c.bench_function("std lib: chained replace with large ASCII text", |b| {
        b.iter(|| str::replace(black_box(&large_input), "\r\n", "\n").replace("\r", "\n"))
    });

    c.bench_function("std lib: chained replace with large pre-normalized ASCII text", |b| {
        b.iter(|| str::replace(black_box(&pre_normalized_large_input), "\r\n", "\n").replace("\r", "\n"))
    });


    c.bench_function("3rd party crate \"newline-converter\": dos2unix()", |b| {
        b.iter(|| newline_converter::dos2unix(black_box(&input)))
    });

    c.bench_function("3rd party crate \"newline-converter\": dos2unix() with pre-normalized text", |b| {
        b.iter(|| newline_converter::dos2unix(black_box(&pre_normalized_input)))
    });
    
    c.bench_function("3rd party crate \"newline-converter\": dos2unix() with large ASCII text", |b| {
        b.iter(|| newline_converter::dos2unix(black_box(&large_input)))
    });

    c.bench_function("3rd party crate \"newline-converter\": dos2unix() with large pre-normalized ASCII text", |b| {
        b.iter(|| newline_converter::dos2unix(black_box(&pre_normalized_large_input)))
    });
    

    c.bench_function("this crate: to_unix_newlines()", |b| {
        let input_slice = input.as_str();
        b.iter(|| newline_normalizer::ToUnixNewlines::to_unix_newlines(black_box(input_slice)))
    });

    c.bench_function("this crate: to_unix_newlines() with pre-normalized text", |b| {
        let input_slice = pre_normalized_input.as_str();
        b.iter(|| newline_normalizer::ToUnixNewlines::to_unix_newlines(black_box(input_slice)))
    });

    c.bench_function("this crate: to_unix_newlines() with large ASCII text", |b| {
        let input_slice = large_input.as_str();
        b.iter(|| newline_normalizer::ToUnixNewlines::to_unix_newlines(black_box(input_slice)))
    });

    c.bench_function("this crate: to_unix_newlines() with pre-normalized large ASCII text", |b| {
        let input_slice = pre_normalized_large_input.as_str();
        b.iter(|| newline_normalizer::ToUnixNewlines::to_unix_newlines(black_box(input_slice)))
    });
}

criterion_group!(benches, bench_to_unix_newlines);
criterion_main!(benches);

