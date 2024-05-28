use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deltachat_message_parser::parser::{
    parse_desktop_set,
    parse_markdown_text,
    parse_only_text,
    LinkDestination,
    is_emoji::get_first_emoji,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let testdata = include_str!("testdata.md");
    let lorem_ipsum_txt = include_str!("lorem_ipsum.txt");
    let r10s_update_message = include_str!("r10s_update_message.txt");
    let links = include_str!("moar_links.txt");
    let emojies = vec![
        "not an emoji",
        "\u{3299}\u{3300}",
        "🟠",
        "",
        "🟠y",
        "🍏",
        "lorem",
        "😀",
    ];

    c.bench_function("only_text_lorem_ipsum.txt", |b| {
        b.iter(|| parse_only_text(black_box(lorem_ipsum_txt)))
    });
    c.bench_function("desktop_set_lorem_ipsum.txt", |b| {
        b.iter(|| parse_desktop_set(black_box(lorem_ipsum_txt)))
    });
    c.bench_function("markdown_lorem_ipsum.txt", |b| {
        b.iter(|| parse_markdown_text(black_box(lorem_ipsum_txt)))
    });

    c.bench_function("only_text_testdata.md", |b| {
        b.iter(|| parse_only_text(black_box(testdata)))
    });
    c.bench_function("desktop_set_testdata.md", |b| {
        b.iter(|| parse_desktop_set(black_box(testdata)))
    });
    c.bench_function("markdown_testdata.md", |b| {
        b.iter(|| parse_markdown_text(black_box(testdata)))
    });

    c.bench_function("only_text_r10s_update_message.txt", |b| {
        b.iter(|| parse_only_text(black_box(r10s_update_message)))
    });
    c.bench_function("desktop_set_r10s_update_message.txt", |b| {
        b.iter(|| parse_desktop_set(black_box(r10s_update_message)))
    });
    c.bench_function("markdown_r10s_update_message.txt", |b| {
        b.iter(|| parse_markdown_text(black_box(r10s_update_message)))
    });

    c.bench_function("parse_link_moar_links.txt", |b| {
        b.iter(|| LinkDestination::parse(black_box(links)))
    });
    c.bench_function("emoji", |b| {
        b.iter(|| emojies.iter().map(|s| get_first_emoji(black_box(s))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
