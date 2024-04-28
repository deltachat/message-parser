use nom::{
    bytes::complete::take_while_m_n,
    character::complete::char,
    combinator::recognize,
    sequence::tuple,
    IResult,
};

use crate::parser::{
    utils::{
        is_hex_digit,
        is_sub_delim,
        is_unreserved,
    },
    parse_from_text::base_parsers::CustomError,
};

fn is_ipvfuture_last(ch: char) -> bool {
    is_sub_delim(ch) || is_unreserved(ch) || ch == ':'
}

pub fn ipvfuture(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(tuple((char('v'), take_while_m_n(1, 1, is_hex_digit), char('.'), take_while_m_n(1, 1, is_ipvfuture_last))))(input)
}
