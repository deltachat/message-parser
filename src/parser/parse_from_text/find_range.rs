use std::ops::RangeInclusive;

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
fn find_range_for_char<'a>(code: u32, ranges: &'a [RangeInclusive<u32>]) -> FindRangeResult<'a> {
    let index = ranges.binary_search_by_key(&code, |range| *range.start());
    match index {
        Ok(_) => FindRangeResult::WasOnRangeStart,
        Err(index) => match index {
            0 => FindRangeResult::Range(&ranges[0]),
            // Since `index` can never be 0, `index - 1` will never overflow. Furthermore, the
            // maximum value which the binary search function returns is `NUMBER_OF_RANGES`.
            // Therefore, `index - 1` will never panic if we index the array with it.
            #[allow(clippy::integer_arithmetic, clippy::indexing_slicing)]
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
