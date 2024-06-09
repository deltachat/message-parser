use std::ops::RangeInclusive;
use super::unicode_ranges::UNICODE_PUNCTUATION_RANGES;

#[derive(Debug, PartialEq, Eq)]
enum FindRangeResult<'a> {
    WasOnRangeStart,
    Range(&'a RangeInclusive<u32>),
}

/// Find a range which `code` might be in it.
///
/// # Description
/// This function gets a sorted slice of inclusive u32 ranges, performs
/// binary search on them and returns a FindRangeResult enum telling
/// which range the `code` might be in. It returns `FindRangeResult::WasOnRangeStart`
/// if the code was exactly on start of a range. Or a `FindRangeResult::Range(range)`
/// which indicates `code` is in `range` or in no ranges.
///
/// # Arguments
///
///  - `code` the u32 to look for a range for.
///
///  - `ranges` a refernce to a slice of `RangeInclusive<u32>`
fn find_range_for_char(code: u32, ranges: &'_ [RangeInclusive<u32>]) -> FindRangeResult<'_> {
    let index = ranges.binary_search_by_key(&code, |range| *range.start());
    match index {
        Ok(_) => FindRangeResult::WasOnRangeStart,
        Err(index) => match index {
            #[allow(clippy::arithmetic_side_effects, clippy::indexing_slicing)]
            0 => FindRangeResult::Range(&ranges[0]),
            // Since `index` can never be 0, `index - 1` will never overflow. Furthermore, the
            // maximum value which the binary search function returns is `NUMBER_OF_RANGES`.
            // Therefore, `index - 1` will never panic if we index the array with it.
            #[allow(clippy::arithmetic_side_effects, clippy::indexing_slicing)]
            index => FindRangeResult::Range(&ranges[index - 1]),
        },
    }
}

/// Returns true of `c` is one of the `ranges`, false otherwise.
///
/// # Arguments
///
///  - `c` A number(u32)
///
///  - `ranges` A sorted slice of ranges to see if `c` is in anyone of them
pub fn is_in_one_of_ranges(c: u32, ranges: &[RangeInclusive<u32>]) -> bool {
    match find_range_for_char(c, ranges) {
        FindRangeResult::WasOnRangeStart => true,
        FindRangeResult::Range(range) => range.contains(&c),
    }
}

#[inline(always)]
pub(crate) fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
}

#[inline(always)]
pub(crate) fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

#[inline(always)]
pub(crate) fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

pub(crate) fn is_sub_delim(c: char) -> bool {
    matches!(
        c,
        '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '='
    )
}

pub(crate) fn is_unreserved(c: char) -> bool {
    is_alpha(c) || is_digit(c) || matches!(c, '_' | '.' | '-' | '~')
}

pub(crate) fn is_white_space(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\t' | ' ')
}

pub(crate) fn is_unicode_white_space(c: char) -> bool {
    is_white_space(c) || 
        matches!(c as u32,
                 0x20 |
                 0xa0 |
                 0x1680..=0x1680 |
                 0x2000..=0x200a |
                 0x202f..=0x202f |
                 0x205f..=0x205f |
                 0x3000..=0x3000)
        // These ranges are extracted from unicode DB using 
        // the script /scripts/extract_unicode_whitespace_ranges.py
        // -- Farooq fkz riseup.net
        //           farooqkz testrun.org
}

pub(crate) fn is_unicode_punctuation(c: char) -> bool {
    is_in_one_of_ranges(c as u32, UNICODE_PUNCTUATION_RANGES[..])
}

pub(crate) fn is_not_white_space(c: char) -> bool {
    !is_white_space(c)
}

pub(crate) fn is_white_space_but_not_linebreak(c: char) -> bool {
    matches!(c, '\t' | ' ')
}
