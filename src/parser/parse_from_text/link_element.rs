use crate::parser::link_url::LinkDestination;
use std::ops::RangeInclusive;
use super::Element;
use crate::nom::{Offset, Slice};
use nom::{
    bytes::{
        complete::{tag, take, take_while1, take_while},
    },
    character::{is_alphabetic as is_alpha, char},
    combinator::{peek, recognize, verify},
    sequence::{tuple, preceded},
    multi::{many_m_n, count},
    AsChar, IResult,
};
use super::base_parsers::*;

// Link syntax here is according to RFC 3986 & 3987 --Farooq


fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn is_digit(c: char) -> bool {
    c.is_digit()
}

// These ranges have been extracted from RFC3987, Page 8.
const ucschar_ranges: [RangeInclusive<u32>; 17] = [
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

fn is_unreserved(c: char) -> bool {
    is_alpha(c) || is_digit(c) || is_other_unreserved(c)
}

fn is_iunreserved(c: char) -> bool {
    is_ucschar(c) || is_unreserved(c)
}

fn is_other_unreserved(c: char) -> bool {
    matches!(c, '_' | '.' | '_' | '~')
}

fn is_pct_encoded(c: [char; 3]) -> bool {
    c[0] == '%' && is_hex_digit(c[1]) && is_hex_digit(c[2])
}

fn is_sub_delim(c: char) -> bool {
    matches!(c, '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=')
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
    let (input, ipv4_) = recognize(tuple((
        complete::u8,
        char('.'),
        complete::u8,
        char('.'),
        complete::u8,
        char('.'),
        complete::u8
    )))(input)?;
    Ok((input, ipv4_))
}

fn is_ireg_name(c: char) -> bool {
    is_iunreserved(c) || is_pct_encoded(c) || is_sub_delims(c)
}

fn h16(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 4, is_hex_digit)(input)
}

fn ls32(input: &str) -> IResult<&str, &str> {
    alt((tuple((h16, char(':'), h16)), ipv4))(input)
}

fn h16_and_period(input: &str) -> IResult<&str, &str> {
    tuple((h16, char(':')))(input)
}

fn double_period(input: &str) -> IResult<&str, &str> {
    tag("::")(input)
}

fn ipv6(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(tuple((count(h16_and_period, 6), ls32))),
        recognize(tuple((double_period, many_m_n(5, 5, h16_and_period), ls32))),
        recognize(tuple((opt(h16), double_period, many_m_n(4, 4, h16_and_period), ls32))),
        recognize(tuple((opt(tuple((many_m_n(0, 1, h16_and_period), ))), double_period, count(h16_and_period, 3), ls32))),
        recognize(tuple((opt(tuple((many_m_n(0, 2, h16_and_period), h16))), double_period, count(h16_and_period, 2), ls32))),
        recognize(tuple((opt(tuple((many_m_n(0, 3, h16_and_period), h16))), double_period, count(h16_and_period, 1), ls32))),
        recognize(tuple((opt(tuple((many_m_n(0, 4, h16_and_period), h16))), double_period, ls32))),
        recognize(tuple((opt(tuple((many_m_n(0, 5, h16_and_period), h16))), double_period, h16))),
        recognize(tuple((opt(tuple((many_m_n(0, 6, h16_and_period), h16))), double_period)))))(input)
}


fn is_ipvfuture_last(c: char) -> bool {
    is_unreserved(c) || is_sub_delims(c) || c == ':'
}

fn ipvfuture(input: &str) -> IResult<&str, &str> {
    tuple((char('v'), take_while_m_n(1, 1, is_hex_digit), char('.'), take_while_m_n(1, 1, is_ipvfuture_last)))(input)
}

fn ip_literal(input: &str) -> IResult<&str, &str> {
    delimited(char('['), alt(ipv6, ipvfuture), char(']'))(input)
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

fn iauthority(input: &str) -> IResult<&str, (&str, &str, &str, bool)> {
    let (input, userinfo) = opt(take_while(is_userinfo), char('@'))(input)?;
    let (input, (host, is_ipv6_or_future)) = parse_host(input)?;
    let (input, port) = preceded(char(':'), take_while(is_digit))(input)?;
    Ok((input, (userinfo, host, port, is_ipv6_or_future)))
}

fn ihier_part(input: &str) -> IResult<&str, (&str, &str, &str, &str, bool)> {
    let (input, (userinfo, host, port, is_ipv6_or_future)) = preceded(tag("//"), iauthority)(input)?;
    let (input, path) = opt(alt(
        take_while(is_ipath_abempty),
        take_while(is_ipath_absolute),
        take_while(is_ipath_rootless)
    ))(input)?;
    Ok((input, (userinfo, host, port, path, is_ipv6_or_future)))
}

fn is_ipchar(c: char) -> bool {
    is_iunreserved(c) || is_pct_encoded(c) || is_sub_delims(c) || matches!(c, ':' | '@')
}

const IPRIVATE_RANGES: [RangeInclusive<u32>; 3]  = [
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

fn is_alphanum_or_hyphen_minus(char: char) -> bool {
    match char {
        '-' => true,
        _ => char.is_alphanum(),
    }
}

pub fn link(input: &str) -> IResult<&str, Element> {
    let (input, scheme) = scheme(input)?;
    let (input, (userinfo, host, port, path, is_ipv6_or_future)) = ihier_part(input)?;
    let (input, query) = opt(preceed(char('?'), take_while(is_query)))(input)?;
    let (input, fragment) = opt(preceed(char('#'), take_while(is_ifragment)))(input)?;
    let mut s = format!("{scheme}://{userinfo}@{host}:{port}{path}{query}{fragment}");
    Ok((input, Element::Link {
        destination: LinkDestination {
            target: &s,
            hostname: Some(hostport),
            punycode: None,
            scheme: scheme
        }
    }))
}
