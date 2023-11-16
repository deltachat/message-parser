// thanks to https://medium.com/reactnative/emojis-in-javascript-f693d0eb79fb for figuring the main details and ranges.

use nom::{
    branch::alt,
    bytes::{complete::tag, streaming::take_while_m_n},
    character::complete::{self, satisfy},
    combinator::{opt, recognize},
    multi::{many1, many_m_n},
    sequence::tuple,
    IResult,
};

fn variant_selector(c: char) -> bool {
    matches!(c, '\u{fe00}'..='\u{fe0f}')
}

fn zero_width_joiner(c: char) -> bool {
    c == '\u{200d}'
}

fn single_char_emoji_core(c: char) -> bool {
    matches!(c,
        // Dingbats
        | '\u{2700}'..='\u{27bf}'
        // miscSymbols
        | '\u{2600}'..='\u{26ff}'
        // cjkLettersAndMonths
        | '\u{3299}' | '\u{3297}'
        // cjkSymbolsAndPunctuation
        | '\u{303d}' | '\u{3030}'
        // enclosedAlphanumerics
        | '\u{24c2}'
        // generalPunctuation
        | '\u{203c}' | '\u{2049}'
        // geometricShapes
        | '\u{25aa}'..='\u{25ab}' | '\u{25b6}' | '\u{25c0}' | '\u{25fb}'..='\u{25fe}'
        // latin1Supplement
        | '\u{00a9}' | '\u{00ae}'
        // letterLikeSymbols
        | '\u{2122}' | '\u{2139}'
        // miscSymbolsAndArrows
        | '\u{2b05}' | '\u{2b06}' | '\u{2b07}' | '\u{2b1b}' | '\u{2b1c}' | '\u{2b50}' | '\u{2b55}'
        // miscTechnical
        | '\u{231a}' | '\u{231b}' | '\u{2328}' | '\u{23cf}' | '\u{23e9}'..='\u{23f3}' | '\u{23f8}'..='\u{23fa}'
        // supplementalArrows
        | '\u{2934}' | '\u{2935}'
        // arrows
        | '\u{2190}'..='\u{2199}'
        // Unicode Block “Enclosed Alphanumeric Supplement”
        | '🅰' | '🅱' | '🅾'| '🅿' | '🆎' | '🆑'..='🆚'
        // Unicode Block “Enclosed Ideographic Supplement” https://www.compart.com/en/unicode/block/U+1F200
        | '🈁' | '🈚'| '🈯' | '🈲'..='🈶' | '🈸'..='🈺' | '🉐' | '🉑'
        // Unicode Block “Miscellaneous Symbols and Pictographs” https://www.compart.com/en/unicode/block/U+1F300
        | '🌀'..='🌡' | '🌤'..='🎓' | '🎖'..='🎗'| '🎙'..='🎛' | '🎞'..='🏰' | '🏳'..='🏵' | '🏷'..='📽' | '📿'..='🔽'
        | '🕉'..='🕎' | '🕐'..='🕧'| '🕯' | '🕰' | '🕳'..='🕺' | '🖇' | '🖊'..='🖍' | '🖐' | '🖕' | '🖖' | '🖤' | '🖥'
        | '🖨' | '🖱' | '🖲' | '🖼' | '🗂'..='🗄' | '🗑'..='🗓' | '🗜' | '🗞' | '🗡' |'🗣' | '🗨' | '🗯' | '🗳' | '🗺'..='🗿'
        // Unicode Block “Emoticons” https://www.compart.com/en/unicode/block/U+1F600
        | '😀'..='🙏'
        // Unicode Block “Transport and Map Symbols” https://www.compart.com/en/unicode/block/U+1F680
        | '🚀'..='🛅' | '🛋'..='🛒' | '🛕'..='🛥' | '🛩' | '🛫'..='🛰' | '🛳'..='🛼'
        // Unicode Block “Geometric Shapes Extended” https://www.compart.com/en/unicode/block/U+1F780
        | '🟠'..='🟫'
        // Unicode Block “Supplemental Symbols and Pictographs” https://www.compart.com/en/unicode/block/U+1F900
        | '🤌'..='🤺' | '🤼'..='🥅' | '🥇'..='🧿'
        // Unicode Block “Symbols and Pictographs Extended-A” https://www.compart.com/en/unicode/block/U+1FA70
        | '🩰'..='🫸'
        // other
        | '🗝' | '🟰'
    )
}

fn emoji_core(input: &str) -> IResult<&str, &str> {
    alt((
        // region flags
        recognize(tuple((
            complete::char('🏴'),
            many1(satisfy(|c| matches!(c, '\u{e0061}'..='\u{e007a}'))),
            complete::char('\u{e007f}'),
        ))),
        // Regional -> Flags
        take_while_m_n(2, 2, |c| ('🇦'..='🇿').contains(&c)),
        // standard emoji chars
        recognize(satisfy(single_char_emoji_core)),
        // SurrPair -> normal emojis?
        // recognize(tuple((
        //     satisfy(|c| ('\u{d800}'..='\u{dbff}').contains(&(c as u32))),
        //     satisfy(|c| ('\u{dc00}'..='\u{dfff}').contains(&(c as u32))),
        // ))),
        // keycap
        recognize(tuple((
            satisfy(|c| ('\u{0023}'..='\u{0039}').contains(&c)),
            opt(complete::char('\u{fe0f}')),
            complete::char('\u{20e3}'),
        ))),
        // mahjongTile
        tag("🀄"),
        // playingCard
        tag("🃏"),
        // other
        tag("🈂️"),
        tag("🈷️"),
        tag("↩️"),
        tag("↪️"),
    ))(input)
}

fn emoji_modifier(c: char) -> bool {
    matches!(c, '🏻'..='🟿')
}

const USIZE_MAX_COMPOSITE_LEN: usize = 10;

macro_rules! emoji_with_variant {
    () => {
        tuple((
            emoji_core,
            opt(satisfy(variant_selector)),
            opt(satisfy(emoji_modifier)),
        ))
    };
}

// nom parser that eats one emoji
pub fn emoji(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        emoji_with_variant!(),
        many_m_n(
            0,
            USIZE_MAX_COMPOSITE_LEN,
            tuple((satisfy(zero_width_joiner), emoji_with_variant!())),
        ),
    )))(input)
}

/// returns first emoji from text if text begins with an emoji
pub fn get_first_emoji(text: &str) -> Option<&str> {
    if let Ok((_, emoji)) = emoji(text) {
        Some(emoji)
    } else {
        None
    }
}

/// If string contains only emojis count the emojis otherwise retuns None
pub fn count_emojis_if_only_contains_emoji(input: &str) -> Option<u32> {
    let mut remainder = input;
    let mut count: u32 = 0;

    while let Ok((new_remainder, _)) = emoji(remainder) {
        remainder = new_remainder;
        count = count.saturating_add(1);
    }

    if !remainder.is_empty() {
        // string contains not only emojis
        return None;
    }

    if count == 0 {
        None
    } else {
        Some(count)
    }
}

#[cfg(test)]
mod emoji_test {
    mod emoji_char {
        use crate::parser::is_emoji::emoji;

        #[test]
        fn some_emojis() {
            assert!(emoji("🔥").is_ok());
        }

        #[test]
        fn not_emoji() {
            // println!("{:?}", emoji("A"));
            assert!(emoji("A").is_err());
        }

        #[test]
        fn keycap() {
            // keycap emojis
            assert!(emoji("#️⃣").is_ok());
        }

        #[test]
        fn flag() {
            // flag emojis
            assert!(emoji("🇦🇨").is_ok());
        }

        #[test]
        fn mahjong() {
            // mahjongTiles
            assert!(emoji("🀄").is_ok());
        }

        #[test]
        fn playing_card() {
            // playingCard
            assert!(emoji("🃏").is_ok());
        }

        #[test]
        fn supplemental_arrows() {
            // supplementalArrows
            assert!(emoji("⤴").is_ok());
            assert!(emoji("⤵").is_ok());
        }

        #[test]
        fn test_variant_emoji() {
            assert!(emoji("🏋️‍♀️").is_ok());
            assert!(emoji("🤹🏽").is_ok());
            assert!(emoji("🛌🏿").is_ok());
        }
        // composite with zero width joiner
        #[test]
        fn test_composite_emoji() {
            assert!(emoji("❤️‍🔥").is_ok());
            assert!(emoji("🐕‍🦺").is_ok());
            assert!(emoji("👩‍👩‍👧").is_ok());
            assert!(emoji("🧑🏿‍🤝‍🧑🏿").is_ok());
            assert!(emoji("👩🏽‍❤️‍👨🏽").is_ok());
        }
    }

    mod exported_methods {
        use crate::parser::is_emoji::{count_emojis_if_only_contains_emoji, get_first_emoji};

        #[test]
        fn test_get_first_emoji() {
            assert_eq!(get_first_emoji("#️⃣ Hashtag"), Some("#️⃣"));
            assert_eq!(get_first_emoji("#️⃣Hashtag"), Some("#️⃣"));
            assert_eq!(get_first_emoji("#️⃣🃏Hashtag"), Some("#️⃣"));
            assert_eq!(get_first_emoji("Hashtag #️⃣"), None);
            assert_eq!(get_first_emoji("'#️⃣"), None);
            assert_eq!(get_first_emoji("❤️‍🔥Hashtag"), Some("❤️‍🔥"));
            assert_eq!(get_first_emoji("👩🏽‍❤️‍👨🏽Hashtag"), Some("👩🏽‍❤️‍👨🏽"));
            assert_eq!(get_first_emoji("🇪🇸🚧"), Some("🇪🇸"));
        }

        #[test]
        fn test_string_contains_only_emojis_and_count() {
            assert_eq!(count_emojis_if_only_contains_emoji("#️⃣"), Some(1));
            assert_eq!(
                count_emojis_if_only_contains_emoji("👩🏽‍❤️‍👨🏽Hashtag"),
                None
            );
            assert_eq!(count_emojis_if_only_contains_emoji("❤️‍🔥"), Some(1));
            assert_eq!(count_emojis_if_only_contains_emoji("👩🏽‍❤️‍👨🏽"), Some(1));
            assert_eq!(
                count_emojis_if_only_contains_emoji("👩🏽‍❤️‍👨🏽👩🏽‍❤️‍👨🏽"),
                Some(2)
            );
            assert_eq!(
                count_emojis_if_only_contains_emoji("👩🏽‍❤️‍👨🏽❤️‍🔥👩🏽‍❤️‍👨🏽"),
                Some(3)
            );
            // hair color
            assert_eq!(count_emojis_if_only_contains_emoji("👨‍🦰"), Some(1));
            assert_eq!(count_emojis_if_only_contains_emoji("👨‍🦳"), Some(1));
            assert_eq!(
                count_emojis_if_only_contains_emoji("🇪🇸🚧🚧🚧🚧🚧🚧🚧"),
                Some(8)
            );
        }
    }
}
