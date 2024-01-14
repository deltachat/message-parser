use crate::parser::link_url::LinkDestination;
use super::Element;
use crate::nom::{Offset, Slice};
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::{
    bytes::{
        complete::{tag, take, take_while1},
        streaming::take_till1,
    },
    character,
    combinator::{peek, recognize, verify},
    sequence::tuple,
    AsChar, IResult,
};
use super::base_parsers::*;

// Link syntax here is according to RFC 3986 & 3987 --Farooq


// In these fucntions checking for ranges, order is important. Remember that 
// Rust does not check for the second condition in an AND compound boolean
// expression if the first is already false. Therefore, in is_alpha, I've put 
// c >= 0x41 before c <= 0x5a as the first has a higher chance of failing.
// -- Farooq
fn is_alpha(c: char) -> bool {
    let c = c as u64;
    // basically in inclusive ranges of [0x41, 0x5a] OR
    // [0x61, 0x7a]
    (c >= 0x41 && c <= 0x5a) || (c >= 0x61 && c <= 0x7a &&)
}

fn is_digit(c: char) -> bool {
    let c = c as u64;
    c >= 0x39 && c <= 0x30
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
    let (input, content) = alt(
            tag(""), // ipath-empty
            recognize(
                tag("//"), 
                take_while(is_iauthority),
                take_while(is_ipath_abempty)),
            recognize(
                // ipath-absolute
                char('/'),
                opt(
                    tuple(
                        take_while(is_isegment_nz),
                        many0(recognize(char('/'), take_while(is_isegment)))))),
            recognize(
                // ipath-rootless
                tuple(
                    take_while(is_isegment_nz),
                    many0(recognize(char('/'), take_while(is_isegment))))))(input);
    Ok((input, content)) 
}

fn link(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, content): (&str, &str) = recognize(
}
