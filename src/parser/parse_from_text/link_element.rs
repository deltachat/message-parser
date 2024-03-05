use std::ops::RangeInclusive;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::{char, u8},
    combinator::{opt, recognize},
    multi::{count, many0, many1, many_m_n},
    sequence::{delimited, tuple},
    AsChar, IResult,
};

use super::find_range::is_in_one_of_ranges;
use super::Element;
use crate::parser::link_url::{LinkDestination, PunycodeWarning};

// Link syntax here is according to RFC 3986 & 3987 --Farooq

fn is_alpha(c: char) -> bool {
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

fn is_ireg_name_not_pct_encoded(c: char) -> bool {
    is_iunreserved(c) || is_sub_delim(c)
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
            let (input, host) = alt((ipv4, take_while_ireg))(input)?;
            Ok((input, (host, false)))
        }
    }
}

fn take_while_ireg(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(many0(take_while_pct_encoded)),
        take_while(is_ireg_name_not_pct_encoded),
    ))(input)
}

fn is_userinfo_not_pct_encoded(c: char) -> bool {
    is_iunreserved(c) || is_sub_delim(c)
}

fn iauthority(input: &str) -> IResult<&str, (&str, &str, bool)> /* (iauthority, host, bool) */ {
    let i = <&str>::clone(&input);
    let (input, userinfo) = opt(recognize(tuple((take_while_iuserinfo, char('@')))))(input)?;
    let (input, (host, is_ipv6_or_future)) = parse_host(input)?;
    let (input, port) = opt(recognize(tuple((char(':'), take_while(is_digit)))))(input)?;
    let userinfo = userinfo.unwrap_or("");
    let port = port.unwrap_or("");
    let len = userinfo.len() + host.len() + port.len();
    Ok((input, (&i[0..len], host, is_ipv6_or_future)))
}

fn take_while_iuserinfo(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(many0(take_while_pct_encoded)),
        take_while(is_iuserinfo_not_pct_encoded),
    ))(input)
}

fn is_iuserinfo_not_pct_encoded(c: char) -> bool {
    is_iunreserved(c) || is_sub_delim(c) || c == ':'
}

fn ihier_part(input: &str) -> IResult<&str, (&str, &str, bool)> {
    let i = <&str>::clone(&input);
    let (input, _double_slash) = tag("//")(input)?;
    let (input, (authority, host, is_ipv6_or_future)) = iauthority(input)?;
    let (input, path) = opt(alt((
        recognize(tuple((
            char('/'),
            opt(tuple((
                take_while_ipchar1,
                many0(tuple((char('/'), take_while_ipchar))),
            ))),
        ))), // ipath-absolute
        recognize(tuple((
            take_while_ipchar,
            many0(tuple((char('/'), take_while_ipchar))),
        ))), // ipath_rootless
    )))(input)?;
    let path = path.unwrap_or(""); // it's ipath_empty
    let len = 2 + authority.len() + path.len();
    // 2 is for double_slash
    Ok((input, (&i[0..len], host, is_ipv6_or_future)))
}

fn is_ipchar_not_pct_encoded(c: char) -> bool {
    is_iunreserved(c) || is_sub_delim(c) || matches!(c, ':' | '@')
}

fn take_while_ipchar(input: &str) -> IResult<&str, &str> {
    recognize(many0(alt((
        take_while(is_ipchar_not_pct_encoded),
        take_while_pct_encoded,
    ))))(input)
}

fn take_while_ipchar1(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((
        take_while(is_ipchar_not_pct_encoded),
        take_while_pct_encoded,
    ))))(input)
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
    recognize(many0(alt((
        take_while(is_iquery_not_pct_encoded),
        take_while_pct_encoded,
    ))))(input)
}

fn take_while_ifragment(input: &str) -> IResult<&str, &str> {
    recognize(many0(alt((
        take_while_ipchar,
        take_while_pct_encoded,
        tag("/"),
        tag("?"),
    ))))(input)
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

fn take_while_pct_encoded(input: &str) -> IResult<&str, &str> {
    recognize(tuple((char('%'), take_while_m_n(2, 2, is_hex_digit))))(input)
}

fn punycode_encode(host: &str) -> String {
    host.split('.')
        .map(|sub| {
            format!(
                "xn--{}",
                unic_idna_punycode::encode_str(sub)
                    .unwrap_or_else(|| "[punycode encode failed]".to_owned())
            )
        })
        .collect::<Vec<String>>()
        .join(".")
}

fn is_puny(host: &str) -> bool {
    for ch in host.chars() {
        if !(is_alphanum_or_hyphen_minus(ch) || ch == '.') {
            return true;
        }
    }
    false
}

fn get_puny_code_warning(link: &str, host: &str) -> Option<PunycodeWarning> {
    if is_puny(host) {
        let ascii_hostname = punycode_encode(host);
        Some(PunycodeWarning {
            original_hostname: host.to_owned(),
            ascii_hostname: ascii_hostname.to_owned(),
            punycode_encoded_url: link.replacen(host, &ascii_hostname, 1)
        })
    } else {
        None
    }
}

pub fn link(input: &str) -> IResult<&str, Element> {
    let input_ = <&str>::clone(&input);
    let (input, scheme) = scheme(input)?;
    let (input, (ihier, host, is_ipv6_or_future)) = ihier_part(input)?;
    let (input, query) = opt(recognize(tuple((char('?'), iquery))))(input)?;
    let (input_, fragment) = opt(recognize(tuple((char('#'), take_while_ifragment))))(input)?;
    let query = query.unwrap_or("");
    let fragment = fragment.unwrap_or("");
    let len = scheme.len() + ihier.len() + query.len() + fragment.len();
    let link = &input_[0..len];
    Ok((
        input,
        Element::Link {
            destination: LinkDestination {
                target: link,
                hostname: if host.len() == 0 { None } else { Some(host) },
                punycode: if is_ipv6_or_future { None } else { get_puny_code_warning(link, host) } ,
                scheme,
            },
        },
    ))
}
