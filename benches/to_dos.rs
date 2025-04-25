use criterion::{criterion_group, criterion_main, Criterion, black_box};
use newline_normalizer::ToDosNewlines;

fn bench_to_unix_newlines(c: &mut Criterion) {
    let input = "
      Это пример параграфа с пробелами и юникодом.\r\n
    Он содержит строки на русском языке, немного английского, и даже: こんにちは世界！\r

    Here's a sentence with normal ASCII characters, leading spaces, and symbols: @$%&.\r

            مرحبا بك في عالم الترميز الموحد.
    ".to_string();

    assert_eq!(input.to_dos_newlines(), newline_converter::unix2dos(&input));

    let pre_normalized_input = input.to_dos_newlines().to_string();

    let large_input = include_str!("./files/sherlock.txt").to_string();
    let pre_normalized_large_input = large_input.to_dos_newlines().to_string();

    assert_eq!(pre_normalized_large_input, newline_converter::unix2dos(&large_input));

    c.bench_function("3rd-party crate \"newline-converter\": unix2dos()", |b| {
        let input_slice = input.as_str();
        b.iter(|| newline_converter::unix2dos(black_box(input_slice)))
    });

    c.bench_function("3rd-party crate \"newline-converter\": unix2dos() with pre-normalized text", |b| {
        let input_slice = pre_normalized_input.as_str();
        b.iter(|| newline_converter::unix2dos(black_box(input_slice)))
    });

    c.bench_function("3rd-party crate \"newline-converter\": unix2dos() for large input", |b| {
        let input_slice = large_input.as_str();
        b.iter(|| newline_converter::unix2dos(black_box(input_slice)))
    });

    c.bench_function("3rd-party crate \"newline-converter\": unix2dos() for large input with pre-normalized text", |b| {
        let input_slice = pre_normalized_large_input.as_str();
        b.iter(|| newline_converter::unix2dos(black_box(input_slice)))
    });
    
    c.bench_function("this crate: to_dos_newlines()", |b| {
        let input_slice = input.as_str();
        b.iter(|| newline_normalizer::ToDosNewlines::to_dos_newlines(black_box(input_slice)))
    });

    c.bench_function("this crate: to_dos_newlines() with pre-normalized text", |b| {
        let input_slice = pre_normalized_input.as_str();
        b.iter(|| newline_normalizer::ToDosNewlines::to_dos_newlines(black_box(input_slice)))
    });

    c.bench_function("this crate: to_dos_newlines() for large input", |b| {
        let input_slice = large_input.as_str();
        b.iter(|| newline_normalizer::ToDosNewlines::to_dos_newlines(black_box(input_slice)))
    });

    c.bench_function("this crate: to_dos_newlines() for large input with pre-normalized text", |b| {
        let input_slice = pre_normalized_large_input.as_str();
        b.iter(|| newline_normalizer::ToDosNewlines::to_dos_newlines(black_box(input_slice)))
    });
}

criterion_group!(benches, bench_to_unix_newlines);
criterion_main!(benches);

