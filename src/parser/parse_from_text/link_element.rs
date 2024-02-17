use super::base_parsers::*;
use super::find_range::is_in_one_of_ranges;
use super::Element;
use crate::nom::{Offset, Slice};
use crate::parser::link_url::LinkDestination;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while, take_while1, take_while_m_n},
    character::complete::{char, u8},
    combinator::{opt, peek, recognize, verify},
    multi::{count, many0, many_m_n},
    sequence::{delimited, preceded, tuple},
    AsChar, IResult,
};
use std::ops::RangeInclusive;

// Link syntax here is according to RFC 3986 & 3987 --Farooq

fn is_alpha(c: char) -> bool{
    c.is_alphabetic()
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

// These ranges have been extracted from RFC3987, Page 8.
const UCSCHAR_RANGES: [RangeInclusive<u32>; 17] = [
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
    is_in_one_of_ranges(c, &UCSCHAR_RANGES[..])
}

fn is_unreserved(c: char) -> bool {
    is_alpha(c) || is_digit(c) || is_other_unreserved(c)
}

fn is_iunreserved(c: char) -> bool {
    is_ucschar(c) || is_unreserved(c)
}

fn is_other_unreserved(c: char) -> bool {
    matches!(c, '_' | '.' | '_' | '~')
}

fn is_sub_delim(c: char) -> bool {
    matches!(
        c,
        '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '='
    )
}

// Here again, order is important. As URLs/IRIs have letters in them
// most of the time and less digits or other characters. --Farooq
fn is_scheme(c: char) -> bool {
    is_alpha(c) || is_digit(c) || is_scheme(c)
}

fn is_ipv4(c: char) -> bool {
    is_digit(c) || c == '.'
}

fn ipv4(input: &str) -> IResult<&str, &str> {
    let (input, ipv4_) =
        recognize(tuple((u8, char('.'), u8, char('.'), u8, char('.'), u8)))(input)?;
    Ok((input, ipv4_))
}

fn is_ireg_name(c: char) -> bool {
    is_iunreserved(c) || is_pct_encoded(c) || is_sub_delim(c)
}

fn h16(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 4, is_hex_digit)(input)
}

fn ls32(input: &str) -> IResult<&str, &str> {
    let result = recognize(tuple((h16, char(':'), h16)))(input);
    if result.is_err() {
        ipv4(input)
    } else {
        result
    }
}

fn h16_and_period(input: &str) -> IResult<&str, &str> {
    recognize(tuple((h16, char(':'))))(input)
}

fn double_period(input: &str) -> IResult<&str, &str> {
    tag("::")(input)
}

fn ipv6(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(tuple((count(h16_and_period, 6), ls32))),
        recognize(tuple((double_period, many_m_n(5, 5, h16_and_period), ls32))),
        recognize(tuple((
            opt(h16),
            double_period,
            many_m_n(4, 4, h16_and_period),
            ls32,
        ))),
        recognize(tuple((
            opt(tuple((many_m_n(0, 1, h16_and_period),))),
            double_period,
            count(h16_and_period, 3),
            ls32,
        ))),
        recognize(tuple((
            opt(tuple((many_m_n(0, 2, h16_and_period), h16))),
            double_period,
            count(h16_and_period, 2),
            ls32,
        ))),
        recognize(tuple((
            opt(tuple((many_m_n(0, 3, h16_and_period), h16))),
            double_period,
            count(h16_and_period, 1),
            ls32,
        ))),
        recognize(tuple((
            opt(tuple((many_m_n(0, 4, h16_and_period), h16))),
            double_period,
            ls32,
        ))),
        recognize(tuple((
            opt(tuple((many_m_n(0, 5, h16_and_period), h16))),
            double_period,
            h16,
        ))),
        recognize(tuple((
            opt(tuple((many_m_n(0, 6, h16_and_period), h16))),
            double_period,
        ))),
    ))(input)
}

fn is_ipvfuture_last(c: char) -> bool {
    is_unreserved(c) || is_sub_delim(c) || c == ':'
}

fn ipvfuture(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        char('v'),
        take_while_m_n(1, 1, is_hex_digit),
        char('.'),
        take_while_m_n(1, 1, is_ipvfuture_last),
    )))(input)
}

fn ip_literal(input: &str) -> IResult<&str, &str> {
    delimited(char('['), alt((ipv6, ipvfuture)), char(']'))(input)
}

/// Parse host
///
/// # Description
///
/// Parse host. Returns the rest, the host string and a boolean indicating
/// if it is IPvFuture or IPv6.
fn parse_host(input: &str) -> IResult<&str, (&str, bool)> {
    match ip_literal(input) {
        Ok((input, host)) => {
            // It got parsed, then it's an IP Literal meaning
            // it's either IPv6 or IPvFuture
            Ok((input, (host, true)))
        }
        Err(..) => {
            let (input, host) = alt((ipv4, take_while(is_ireg_name)))(input)?;
            Ok((input, (host, false)))
        }
    }
}

fn is_userinfo(c: char) -> bool {
    is_iunreserved(c) || is_pct_encoded(c) || is_sub_delim(c)
}

fn iauthority(input: &str) -> IResult<&str, (&str, &str, &str, bool)> {
    let (input, userinfo) = opt(recognize(tuple((take_while(is_userinfo), char('@')))))(input)?;
    let (input, (host, is_ipv6_or_future)) = parse_host(input)?;
    let (input, port) = preceded(char(':'), take_while(is_digit))(input)?;
    let userinfo = userinfo.unwrap_or("");
    Ok((input, (userinfo, host, port, is_ipv6_or_future)))
}

fn ihier_part(input: &str) -> IResult<&str, (&str, &str, &str, &str, bool)> {
    let (input, (userinfo, host, port, is_ipv6_or_future)) =
        preceded(tag("//"), iauthority)(input)?;
    let (input, path) = opt(alt((
        recognize(tuple((
            char('/'),
            opt(tuple((
                take_while1(is_ipchar),
                many0(tuple((char('/'), take_while(is_ipchar)))),
            ))),
        ))), // ipath-absolute
        recognize(tuple((
            take_while_ipchar,
            many0(tuple((char('/'), take_while(is_ipchar)))),
        ))), // ipath_rootless
    )))(input)?;
    let path = path.unwrap_or(""); // it's ipath_empty
    Ok((input, (userinfo, host, port, path, is_ipv6_or_future)))
}

fn is_ipchar_not_pct_encoded(c: char) -> bool {
    is_iunreserved(c) || is_sub_delim(c) || matches!(c, ':' | '@')
}

fn take_while_ipchar(input: &str) -> IResult<&str, &str> {
    many0(alt((take_while(is_ipchar_not_pct_encoded), take_while(is_pct_encoded)))(input)
}

fn is_pct_encoded(c: [char; 3]) -> bool {
    c[0] == '%' && is_hex_digit(c[1]) && is_hex_digit(c[2])
}

const IPRIVATE_RANGES: [RangeInclusive<u32>; 3] =
    [0xe000..=0xf8ff, 0xf0000..=0xffffd, 0x100000..=0x10fffd];

fn is_iprivate(c: char) -> bool {
    is_in_one_of_ranges(c, &IPRIVATE_RANGES[..])
}

fn is_iquery_not_pct_encoded(c: char) -> bool {
    is_iprivate(c) || is_ipchar_not_pct_encoded(c) || matches!(c, '/' | '?')
}

fn iquery(input: &str) -> IResult<&str, &str> {
    recognize(many0(alt((take_while(is_iquery_not_pct_encoded), pct_encoded))))(input)
}

fn is_ifragment(c: char) -> bool {
    is_ipchar(c) || matches!(c, '/' | '?')
}

fn ifragment(input: &str) -> IResult<&str, &str> {
    take_while(is_ifragment)(input)
}

fn scheme(input: &str) -> IResult<&str, &str> {
    take_while(is_scheme)(input)
}

fn is_alphanum_or_hyphen_minus(char: char) -> bool {
    match char {
        '-' => true,
        _ => char.is_alphanum(),
    }
}

fn pct_encoded(input: &str) -> IResult<&str, &str> {
    recognize(tuple((char('%'), take_while_m_n(2, 2, is_hex_digit))))(input)
}

pub fn link(input: &str) -> IResult<&str, Element> {
    let (input, scheme) = scheme(input)?;
    let (input, (userinfo, host, port, path, is_ipv6_or_future)) = ihier_part(input)?;
    let (input, Some(query)) = opt(preceded(char('?'), iquery))(input)?;
    let (input, Some(fragment)) = opt(preceded(char('#'), take_while(is_ifragment)))(input)?;
    let mut s = format!("{scheme}://{userinfo}@{host}:{port}{path}?{query}#{fragment}");
    Ok((
        input,
        Element::Link {
            destination: LinkDestination {
                target: &s,
                hostname: Some(host),
                punycode: None,
                scheme: scheme,
            },
        },
    ))
}
