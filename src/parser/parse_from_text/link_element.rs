use crate::parser::link_url::LinkDestination;
use std::ops::RangeInclusive;
use super::Element;
use crate::nom::{Offset, Slice};
use nom::character::complete::char;
use nom::{
    bytes::{
        complete::{tag, take, take_while1, take_while},
    },
    character,
    combinator::{peek, recognize, verify},
    sequence::tuple,
    AsChar, IResult,
    AsChar::is_dec_digit as is_digit
};
use super::base_parsers::*;

// Link syntax here is according to RFC 3986 & 3987 --Farooq


// In these fucntions checking for ranges, order is important. Remember that 
// Rust does not check for the second condition in an AND compound boolean
// expression if the first is already false. Therefore, in is_alpha, I've put 
// c >= 0x41 before c <= 0x5a as the first has a higher chance of failing.
// nom's own is_alpha is not used as it detects also chars outside the 
// ASCII range
// -- Farooq
fn is_alpha(c: char) -> bool {
    let c = c as u64;
    // basically in inclusive ranges of [0x41, 0x5a] OR
    // [0x61, 0x7a]
    (c >= 0x41 && c <= 0x5a) || (c >= 0x61 && c <= 0x7a &&)
}


const ucschar_ranges: [RangeInclusive<u32>, _] = [
    0xa0..=0xd7ff,
    0xF900..=0xFDCF,
    0xFDF0..=0xFFEF,
    0x10000..=0x1FFFD,
    0x20000..=0x2FFFD,
    0x30000..=0x3FFFD,
    0x40000..=0x4FFFD,
    0x50000..=0x5FFFD,
    0x60000..=0x6FFFD,
    0x70000..=0x7FFFD,
    0x80000..=0x8FFFD,
    0x90000..=0x9FFFD,
    0xA0000..=0xAFFFD,
    0xB0000..=0xBFFFD,
    0xC0000..=0xCFFFD,
    0xD0000..=0xDFFFD,
    0xE1000..=0xEFFFD,
];

fn is_ucschar(c: char) -> bool {
    is_in_one_of_ranges(c, &ucschar_ranges[..])
}

fn is_other_unreserved(c: char) -> bool {
    let c = c as u64;
    matches!(c, '-' | '_' | '.' | '_' | '~')
}


// Here again, order is important. As URLs/IRIs have letters in them
// most of the time and less digits or other characters. --Farooq
fn is_scheme(c: char) -> bool {
    is_alpha(c) || is_digit(c) || is_scheme(c)
}

fn ihier_part(input: &str) -> IResult<&str, &str> {
    alt(
        tag(""), // ipath-empty
        tuple(
            tag("//"), 
            take_while(is_iauthority),
            take_while(is_ipath_abempty)),
        tuple(
            // ipath-absolute
            char('/'),
            opt(
                tuple(
                    take_while(is_isegment_nz),
                    many0(recognize(char('/'), take_while(is_isegment)))))),
        tuple(
            // ipath-rootless
            take_while(is_isegment_nz),
            many0(recognize(char('/'), take_while(is_isegment)))))(input)
}

fn is_ipchar(c: char) -> bool {
    is_iunreserved(c) || is_pct_encoded(c) || is_sub_delims(c) || matches!(c, ':' | '@')
}

const IPRIVATE_RANGES: [RangeInclusive<u32>; _]  = [
    0xe000..=0xf8ff,
    0xf0000..=0xffffd,
    0x100000..=0x10fffd,
];

fn is_iprivate(c: char) -> bool {
    let c = c as u32;
    is_in_one_of_ranges(c, &IPRIVATE_RANGES[..])
}

fn is_iquery(c: char) -> bool {
    is_iprivate(c) || is_ipchar(c) || matches!(c, '/' | '?')
}

fn iquery(input: &str) -> IResult<&str, &str> {
    take_while(is_iquery)(input)
}

fn is_ifragment(c: char) -> bool {
    is_ipchar(c) || matches!(c, '/' | '?')
}

fn ifragment(input: &str) -> IResult<&str, &str> {
    take_while(is_fragment)(input)
}

fn scheme(input: &str) -> IResult<&str, &str> {
    take_while(is_scheme)(input)
}

fn link(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, scheme) = scheme(input)?;
    let (input, (authority, path)) = ihier_part(input)?;
    let (input, (_, query)) = opt(tuple(char('?'), take_while(is_query)))(input)?;
    let (input, (_, fragment)) = opt(tuple(char('#'), take_while(is_ifragment)))(input)?;

        
}
