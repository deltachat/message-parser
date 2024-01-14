use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq)]
enum FindRangeResult<'a> {
    WasOnRangeStart,
    Range(&'a RangeInclusive<u32>),
}

fn find_range_for_char<'a>(code: u32, ranges: &[RangeInclusive<u32>]) -> FindRangeResult<'a> {
    let index = HASHTAG_CONTENT_CHAR_RANGES.binary_search_by_key(&code, |range| *range.start());
    match index {
        Ok(_) => FindRangeResult::WasOnRangeStart,
        Err(index) => match index {
            0 => FindRangeResult::Range(&HASHTAG_CONTENT_CHAR_RANGES[0]),
            // Since `index` can never be 0, `index - 1` will never overflow. Furthermore, the
            // maximum value which the binary search function returns is `NUMBER_OF_RANGES`.
            // Therefore, `index - 1` will never panic if we index the array with it.
            #[allow(clippy::integer_arithmetic, clippy::indexing_slicing)]
            index => FindRangeResult::Range(&HASHTAG_CONTENT_CHAR_RANGES[index - 1]),
        },
    }
}

pub fn is_in_one_of_ranges(c: char, ranges: &[RangeInclusive<u32>) -> bool {
    let c = c as u32;
    match find_range_for_char(c, ranges) {
        FindRangeResult::WasOnRangeStart => true,
        FindRangeResult::Range(range) => range.contains(&c),
    }
}
