use crate::parser::link_url::LinkDestination;
use std::ops::RangeInclusive;
use super::Element;
use crate::nom::{Offset, Slice};
use nom::{
    bytes::{
        complete::{tag, take, take_while1, take_while},
    },
    character::{is_digit, is_alphabetic as is_alpha, is_hex_digit, char},
    combinator::{peek, recognize, verify},
    sequence::{tuple, preceded},
    AsChar, IResult,
};
use super::base_parsers::*;

// Link syntax here is according to RFC 3986 & 3987 --Farooq



// These ranges have been extracted from RFC3987, Page 8.
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
    let (input, possible_ipv4) = tuple(
        complete::u8,
        char('.'),
        complete::u8,
        char('.'),
        complete::u8,
        char('.'),
        complete::u8
    )(input);
    // This might be an IPv4
    let inner_pair = separated_pair(take_while1(is_digit), char('.'), take_while1(is_digit));
    let ((part0, part1), (part2, part3)) = separated_pair(inner_pair, char('.'), inner_pair)(input)?;
    part0.parse::<u8>()?;
    part1.parse::<u8>()?;
    part2.parse::<u8>()?;
    part3.parse::<u8>()?;
    Ok((input, possible_ipv4))
}

fn is_ireg_name(c: char) -> bool {
    is_iunreserved(c) || is_pct_encoded(c) || is_sub_delims(c)
}

fn h16(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 4, is_hex_digit)
}

fn ls32(input: &str) -> IResult<&str, &str> {
    alt(tuple(h16, char(':'), h16), ipv4)
}

fn ipv6(input: &str) -> IResult<&str, &str> {
    let h16_and_period = tuple(h16, char(':'));
    let double_period = tag("::");
    tuple(
        take_while_m_n(6, 6, h16_and_period),
        alt(ls32, double_period),
        take_while(5, 5, h16_and_period),
        alt(ls32, opt(h16)),
        double_period,
        take_while(4, 4, h16_and_period),
        alt(ls32, opt(tuple(take_while_m_n(0, 1, h16_and_period)

}


fn is_ipvfuture_last(c: char) -> bool {
    is_unreserved(c) || is_sub_delims(c) || c == ':'
}

fn ipvfuture(input: &str) -> IResult<&str, &str> {
    tuple(char('v'), take_while_m_n(1, 1, is_hex_digit), char('.'), take_while_m_n(1, 1, is_ipvfuture_last))
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
fn parse_host(input: &str) -> IResult<&str, &str, bool> {
    let (input, host) = ip_literal(input)?;
    if host.is_some() {
        // It got parsed, then it's an IP Literal meaning
        // it's either IPv6 or IPvFuture
        Ok((input, host.unwrap(), true))
    } else {
        let (input, host) = alt((ipv4, take_while(is_ireg_name)))(input)?;
        Ok((input, host, false))
    }
}

fn iauthority(input: &str) -> IResult<&str, &str, &str, &str, bool> {
    let (input, userinfo) = opt(take_while(is_userinfo), char('@'))(input);
    let (input, host, is_ipvfuture) = parse_host(input);
    let (input, port) = preceded(char(':'), take_while(is_digit))(input);
    Ok((input, userinfo, host, port, is_ipv6))
}

fn ihier_part(input: &str) -> IResult<&str, &str, &str, bool> {
    let (input, authority) = preceded(tag("//"), iauthority)(input);
    let (input, path) = alt(
        take_while(is_ipath_abempty),
        char(''), // ipath-empty
        take_while(is_ipath_absolute),
        take_while(is_ipath_rootless)
    )(input);
    Ok((input, authority, path, is_ipvfuture))
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

fn is_alphanum_or_hyphen_minus(char: char) -> bool {
    match char {
        '-' => true,
        _ => char.is_alphanum(),
    }
}

fn link(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, scheme) = scheme(input)?;
    let (input, (userinfo, hostport, is_ipvfuture), path) = ihier_part(input)?;
    let (input, query) = opt(preceed(char('?'), take_while(is_query)))(input)?;
    let (input, fragment) = opt(preceed(char('#'), take_while(is_ifragment)))(input)?;
    Element::Link {
        destination: LinkDestination {
            target: input,
            hostname: Some(hostport),
            punycode: ,
            scheme: scheme
        }
    }
}
