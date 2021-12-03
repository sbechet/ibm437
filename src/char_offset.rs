#[warn(dead_code)]
pub const CHARS_PER_ROW: usize = 16;

fn char_offset(c: char, base: char) -> usize {
    c as usize - base as usize
}

pub fn char_offset_impl(c: char) -> usize {
    match c {
        // keep single byte characters where they are
        ' '..='~'
        | '\u{0000}'
        | '\u{a0}'..='£'
        | 'ä'..='ï'
        | 'º'..='½'
        | 'Ä'..='Ç'
        | 'ß'..='â'
        | 'ñ'..='ô'
        | 'ù'..='ü'
        | '°'..='²'
        | 'µ'..='·'
        | 'ö'..='÷'
        | '¥'
        | '§'
        | '¿'
        | 'É'
        | 'Ñ'
        | 'Ö'
        | 'Ü'
        | 'ÿ' => c as usize,

        '═'..='╬' => 0x01 + char_offset(c, '═'), // 29
        '♪'..='♫' => 0x1E + char_offset(c, '♪'), // 2

        // gap between ~ and A0
        '←'..='↕' => 0x7F + char_offset(c, '←'), // 6
        '▐'..='▓' => 0x85 + char_offset(c, '▐'), // 4
        'ª'..='¬' => 0x89 + char_offset(c, 'ª'),    // 3
        '☺'..='☼' => 0x8C + char_offset(c, '☺'), // 3
        'δ'..='ε' => 0x8F + char_offset(c, 'δ'),    // 2
        '∙'..='√' => 0x91 + char_offset(c, '∙'), // 2
        '∞'..='∟' => 0x93 + char_offset(c, '∞'), // 2
        '≤'..='≥' => 0x95 + char_offset(c, '≤'), // 2
        '⌠'..='⌡' => 0x97 + char_offset(c, '⌠'), // 2
        '◘'..='◙' => 0x99 + char_offset(c, '◘'), // 2
        '♥'..='♦' => 0x9B + char_offset(c, '♥'), // 2
        'σ'..='τ' => 0x9D + char_offset(c, 'σ'),    // 2
        'ƒ' => 0x9F,

        'Γ' => 0xA4,
        'Θ' => 0xA6,
        'Σ' => 0xA8,
        'Φ' => 0xA9,
        'Ω' => 0xAA,
        'α' => 0xAB,
        'π' => 0xAC,
        'φ' => 0xAD,
        '•' => 0xAE,
        '‼' => 0xAF,
        'ⁿ' => 0xB3,
        '₧' => 0xB4,
        '↨' => 0xB8,
        '∩' => 0xB9,
        '≈' => 0xBE,
        '≡' => 0xC0,
        '⌂' => 0xC1,
        '⌐' => 0xC2,
        '─' => 0xC3,
        '│' => 0xC8,
        '┌' => 0xCA,
        '┐' => 0xCB,
        '└' => 0xCC,
        '┘' => 0xCD,
        '├' => 0xCE,
        '┤' => 0xCF,
        '┬' => 0xD0,
        '┴' => 0xD2,
        '┼' => 0xD3,
        '▀' => 0xD4,
        '▄' => 0xD5,
        '█' => 0xD7,
        '▌' => 0xD8,
        '■' => 0xD9,
        '▬' => 0xDA,
        '▲' => 0xDB,
        '►' => 0xDD,
        '▼' => 0xDE,
        '◄' => 0xE3,
        '○' => 0xF0,
        '♀' => 0xF5,
        '♂' => 0xF8,
        '♠' => 0xFD,
        '♣' => 0xFE,
        _ => '?' as usize,
    }
}

#[cfg(test)]
mod char_offset_test {
    use super::char_offset_impl;

    #[test]
    fn test_unique_and_exhaustive() {
        let mut chars = include_str!("Characters_src.txt")
            .lines()
            .map(|l| l.chars())
            .flatten()
            .collect::<Vec<char>>();

        // Replace 2 of the 3 spaces
        chars[0] = '\u{0000}';
        let last = chars.len() - 1;
        chars[last] = '\u{A0}';

        let mut hit = [false; 256];

        for c in chars.clone().into_iter() {
            let offs = char_offset_impl(c) as usize;
            hit[offs] = true;
        }

        for (i, is_hit) in hit.iter().enumerate() {
            assert!(is_hit, "Offset has no character: {}", i);
        }

        let mut hit = [false; 256];
        for c in chars.into_iter() {
            let offs = char_offset_impl(c) as usize;

            assert_eq!(
                false, hit[offs],
                "Duplicate offset for {} ({})",
                c, c as u32
            );
            hit[offs] = true;
        }
    }
}
