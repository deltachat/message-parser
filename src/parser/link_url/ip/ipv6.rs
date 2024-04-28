use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::char,
    combinator::{opt, recognize},
    multi::{count, many_m_n},
    sequence::tuple,
    IResult,
};

use crate::parser::{parse_from_text::base_parsers::CustomError, utils::is_hex_digit};

use super::ipv4::ipv4;

// consume 1 to 4 hex digit(s)
// TODO These 4 functions should be macros instead
fn h16(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    take_while_m_n(1, 4, is_hex_digit)(input)
}

// consume <h16> <period> <h16> or an ipv4
fn ls32(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    let result = recognize(tuple((h16, char(':'), h16)))(input);
    if result.is_err() {
        ipv4(input)
    } else {
        result
    }
}

// consume <h16> <period>
fn h16_and_period(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(tuple((h16, char(':'))))(input)
}

fn double_period(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    tag("::")(input)
}

pub fn ipv6(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    // an IPv6 is one of these:
    alt((
        // <6 h16_and_period> <ls32>
        recognize(tuple((count(h16_and_period, 6), ls32))),
        // :: <5 h16_and_period> <ls32>
        recognize(tuple((double_period, many_m_n(5, 5, h16_and_period), ls32))),
        // [h16] :: <4 h16_and_period> <ls32>
        recognize(tuple((
            opt(h16),
            double_period,
            count(h16_and_period, 4),
            ls32,
        ))),
        // [h16_and_period] :: <3*h16_and_period> <ls32>
        recognize(tuple((
            opt(tuple((many_m_n(0, 1, h16_and_period),))),
            double_period,
            count(h16_and_period, 3),
            ls32,
        ))),
        // [<0 to 2 h16_and_period> <h16>] :: <2*h16_and_period> <ls32>
        recognize(tuple((
            opt(tuple((many_m_n(0, 2, h16_and_period), h16))),
            double_period,
            count(h16_and_period, 2),
            ls32,
        ))),
        // [<0 to 3 h16_and_period>] <h16> :: <ls32>
        recognize(tuple((
            opt(tuple((many_m_n(0, 3, h16_and_period), h16))),
            double_period,
            ls32,
        ))),
        // [<0 to 4 h16_and_period>] <h16> :: <ls32>
        recognize(tuple((
            opt(tuple((many_m_n(0, 4, h16_and_period), h16))),
            double_period,
            ls32,
        ))),
        // [<0 to 5 h16_and_period>] <h16> :: <ls32>
        recognize(tuple((
            opt(tuple((many_m_n(0, 5, h16_and_period), h16))),
            double_period,
            h16,
        ))),
        // [<0 to 6 h16_and_period>] <h16> :: <ls32>
        recognize(tuple((
            opt(tuple((many_m_n(0, 6, h16_and_period), h16))),
            double_period,
        ))),
    ))(input)
}
