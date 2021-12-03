use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/char_offset.rs"]
mod char_offset;
use char_offset::char_offset_impl;

pub fn char_offset_impl_original(c: char) -> usize {
    match c {
        ' '..='~' => c as usize,
        '\u{0000}' => 0x00,
        '☺' => 0x01,
        '☻' => 0x02,
        '♥' => 0x03,
        '♦' => 0x04,
        '♣' => 0x05,
        '♠' => 0x06,
        '•' => 0x07,
        '◘' => 0x08,
        '○' => 0x09,
        '◙' => 0x0A,
        '♂' => 0x0B,
        '♀' => 0x0C,
        '♪' => 0x0D,
        '♫' => 0x0E,
        '☼' => 0x0F,
        '►' => 0x10,
        '◄' => 0x11,
        '↕' => 0x12,
        '‼' => 0x13,
        '¶' => 0x14,
        '§' => 0x15,
        '▬' => 0x16,
        '↨' => 0x17,
        '↑' => 0x18,
        '↓' => 0x19,
        '→' => 0x1A,
        '←' => 0x1B,
        '∟' => 0x1C,
        '↔' => 0x1D,
        '▲' => 0x1E,
        '▼' => 0x1F,

        '⌂' => 0x7F,
        'Ç' => 0x80,
        'ü' => 0x81,
        'é' => 0x82,
        'â' => 0x83,
        'ä' => 0x84,
        'à' => 0x85,
        'å' => 0x86,
        'ç' => 0x87,
        'ê' => 0x88,
        'ë' => 0x89,
        'è' => 0x8A,
        'ï' => 0x8B,
        'î' => 0x8C,
        'ì' => 0x8D,
        'Ä' => 0x8E,
        'Å' => 0x8F,
        'É' => 0x90,
        'æ' => 0x91,
        'Æ' => 0x92,
        'ô' => 0x93,
        'ö' => 0x94,
        'ò' => 0x95,
        'û' => 0x96,
        'ù' => 0x97,
        'ÿ' => 0x98,
        'Ö' => 0x99,
        'Ü' => 0x9A,
        '¢' => 0x9B,
        '£' => 0x9C,
        '¥' => 0x9D,
        '₧' => 0x9E,
        'ƒ' => 0x9F,
        'á' => 0xA0,
        'í' => 0xA1,
        'ó' => 0xA2,
        'ú' => 0xA3,
        'ñ' => 0xA4,
        'Ñ' => 0xA5,
        'ª' => 0xA6,
        'º' => 0xA7,
        '¿' => 0xA8,
        '⌐' => 0xA9,
        '¬' => 0xAA,
        '½' => 0xAB,
        '¼' => 0xAC,
        '¡' => 0xAD,
        '«' => 0xAE,
        '»' => 0xAF,
        '░' => 0xB0,
        '▒' => 0xB1,
        '▓' => 0xB2,
        '│' => 0xB3,
        '┤' => 0xB4,
        '╡' => 0xB5,
        '╢' => 0xB6,
        '╖' => 0xB7,
        '╕' => 0xB8,
        '╣' => 0xB9,
        '║' => 0xBA,
        '╗' => 0xBB,
        '╝' => 0xBC,
        '╜' => 0xBD,
        '╛' => 0xBE,
        '┐' => 0xBF,
        '└' => 0xC0,
        '┴' => 0xC1,
        '┬' => 0xC2,
        '├' => 0xC3,
        '─' => 0xC4,
        '┼' => 0xC5,
        '╞' => 0xC6,
        '╟' => 0xC7,
        '╚' => 0xC8,
        '╔' => 0xC9,
        '╩' => 0xCA,
        '╦' => 0xCB,
        '╠' => 0xCC,
        '═' => 0xCD,
        '╬' => 0xCE,
        '╧' => 0xCF,
        '╨' => 0xD0,
        '╤' => 0xD1,
        '╥' => 0xD2,
        '╙' => 0xD3,
        '╘' => 0xD4,
        '╒' => 0xD5,
        '╓' => 0xD6,
        '╫' => 0xD7,
        '╪' => 0xD8,
        '┘' => 0xD9,
        '┌' => 0xDA,
        '█' => 0xDB,
        '▄' => 0xDC,
        '▌' => 0xDD,
        '▐' => 0xDE,
        '▀' => 0xDF,
        'α' => 0xE0,
        'ß' => 0xE1,
        'Γ' => 0xE2,
        'π' => 0xE3,
        'Σ' => 0xE4,
        'σ' => 0xE5,
        'µ' => 0xE6,
        'τ' => 0xE7,
        'Φ' => 0xE8,
        'Θ' => 0xE9,
        'Ω' => 0xEA,
        'δ' => 0xEB,
        '∞' => 0xEC,
        'φ' => 0xED,
        'ε' => 0xEE,
        '∩' => 0xEF,
        '≡' => 0xF0,
        '±' => 0xF1,
        '≥' => 0xF2,
        '≤' => 0xF3,
        '⌠' => 0xF4,
        '⌡' => 0xF5,
        '÷' => 0xF6,
        '≈' => 0xF7,
        '°' => 0xF8,
        '∙' => 0xF9,
        '·' => 0xFA,
        '√' => 0xFB,
        'ⁿ' => 0xFC,
        '²' => 0xFD,
        '■' => 0xFE,
        '\u{00A0}' => 0xFF,
        _ => '?' as usize,
    }
}

fn benchmark_original_ascii(c: &mut Criterion) {
    let chars = "The quick brown fox jumps over the lazy dog".chars();

    c.bench_function("Original mapping, ASCII characters", |b| {
        b.iter(|| {
            for c in chars.clone() {
                let _ = char_offset_impl_original(black_box(c));
            }
        })
    });
}

fn benchmark_current_ascii(c: &mut Criterion) {
    let chars = "The quick brown fox jumps over the lazy dog".chars();

    c.bench_function("Remapped, ASCII characters", |b| {
        b.iter(|| {
            for c in chars.clone() {
                let _ = char_offset_impl(black_box(c));
            }
        })
    });
}

fn benchmark_original_all_chars(c: &mut Criterion) {
    let chars = include_str!("../src/Characters_src.txt")
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect::<Vec<char>>();

    c.bench_function("Original mapping", |b| {
        b.iter(|| {
            for c in chars.iter() {
                let _ = char_offset_impl_original(black_box(*c));
            }
        })
    });
}

fn benchmark_current_all_chars(c: &mut Criterion) {
    let chars = include_str!("../doc/Characters.txt")
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect::<Vec<char>>();

    c.bench_function("Remapped", |b| {
        b.iter(|| {
            for c in chars.iter() {
                let _ = char_offset_impl(black_box(*c));
            }
        })
    });
}

criterion_group!(
    char_offset_bench,
    benchmark_original_ascii,
    benchmark_current_ascii,
    benchmark_original_all_chars,
    benchmark_current_all_chars
);
criterion_main!(char_offset_bench);
