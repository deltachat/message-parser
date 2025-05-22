use std::ops::RangeInclusive;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1, take_while_m_n},
    character::complete::char,
    combinator::{opt, recognize},
    multi::{many0, many1},
    sequence::{pair, tuple},
    IResult, Slice,
};

use crate::parser::{
    link_url::{
        ip::{ip_literal::ip_literal, ipv4::ipv4},
        LinkDestination,
    },
    parse_from_text::base_parsers::CustomError,
    utils::{
        is_alpha, is_digit, is_hex_digit, is_in_one_of_ranges, is_not_white_space, is_sub_delim,
        is_unreserved,
    },
};

use super::{
    parenthesis_counter::count_chars_in_complete_parenthesis,
    punycode_warning::get_puny_code_warning,
};

/// determines which generic schemes (without '://') get linkifyed
fn is_allowed_generic_scheme(scheme: &str) -> bool {
    matches!(
        scheme.to_ascii_lowercase().as_ref(),
        "mailto"
            | "news"
            | "feed"
            | "tel"
            | "sms"
            | "geo"
            | "maps"
            | "bitcoin"
            | "bitcoincash"
            | "eth"
            | "ethereum"
            | "magnet"
    )
}

const ALLOWED_TOP_LEVEL_DOMAINS: &[&str] = &[
    // originals from RFC920 + net
    ".com", ".org", ".net", ".edu", ".gov", ".mil",
    // for deltachat
    ".chat",
    // !todo country codes here next
];

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
    is_in_one_of_ranges(c as u32, &UCSCHAR_RANGES[..])
}

fn is_iunreserved(c: char) -> bool {
    is_unreserved(c) || is_ucschar(c)
}

// Here again, order is important. As URLs/IRIs have letters in them
// most of the time and less digits or other characters. --Farooq
fn is_scheme(c: char) -> bool {
    is_alpha(c) || is_digit(c) || is_other_scheme(c)
}

fn is_other_scheme(c: char) -> bool {
    matches!(c, '+' | '-' | '.')
}

/**
 * allowed chars in host names (except for pct encoded)
 */
fn is_ireg_name_not_pct_encoded(c: char) -> bool {
    is_iunreserved(c)
}

/// Parse host
///
/// # Description
///
/// Parse host. Returns the rest, the host string and a boolean indicating
/// if it is IPvFuture or IPv6.
///
/// A host is either an IP-Literal(IPv6 or vFuture) or an
/// IPv4 or an Ireg name(e.g. far.chickenkiller.com :)
///
/// # Return value
///  - `(host, true)` if host is IP-Literal
///  - `(host, false)` if it's ipv4 or ireg-name
fn parse_host(input: &str) -> IResult<&str, (&str, bool), CustomError<&str>> {
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

fn take_while_ireg(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    let (input, result) = recognize(many0(alt((
        recognize(many1(take_while_pct_encoded)),
        take_while1(is_ireg_name_not_pct_encoded),
    ))))(input)?;

    Ok((input, result.trim_end_matches('.')))
}

/// Parse the iauthority block
/// # Description
///  An iauthority is...
///  `[iuserinfo] <host> [:port]`
/// # Return value
///  unconsumed string AND `(iauthority, host, is_ipliteral)` where `ipliteral` is a boolean
fn iauthority(input: &str) -> IResult<&str, (&str, &str, bool), CustomError<&str>> /* (iauthority, host, bool) */
{
    let i = <&str>::clone(&input);
    let (input, userinfo) = opt(recognize(tuple((take_while_iuserinfo, char('@')))))(input)?;
    let (input, (host, is_ipv6_or_future)) = parse_host(input)?;
    let (input, port) = opt(recognize(tuple((char(':'), take_while(is_digit)))))(input)?;
    let userinfo = userinfo.unwrap_or("");
    let port = port.unwrap_or("");
    let len = userinfo.len().saturating_add(port.len());
    if let Some(out) = i.get(0..len) {
        Ok((input, (out, host, is_ipv6_or_future)))
    } else {
        Err(nom::Err::Failure(CustomError::NoContent))
    }
}

/// Consume an iuserinfo
fn take_while_iuserinfo(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    alt((
        recognize(many0(take_while_pct_encoded)),
        take_while(is_iuserinfo_not_pct_encoded),
    ))(input)
}

fn is_iuserinfo_not_pct_encoded(c: char) -> bool {
    is_iunreserved(c) || is_sub_delim(c) || c == ':'
}

fn is_ipchar_not_pct_encoded(c: char) -> bool {
    is_iunreserved(c) || is_sub_delim(c) || matches!(c, ':' | '@')
}

fn take_while_ipchar(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(many0(alt((
        take_while(is_ipchar_not_pct_encoded),
        take_while_pct_encoded,
    ))))(input)
}

fn take_while_ipchar1(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(many1(alt((
        take_while1(is_ipchar_not_pct_encoded),
        take_while_pct_encoded,
    ))))(input)
}

const IPRIVATE_RANGES: [RangeInclusive<u32>; 3] =
    [0xe000..=0xf8ff, 0xf0000..=0xffffd, 0x100000..=0x10fffd];

fn is_iprivate(c: char) -> bool {
    is_in_one_of_ranges(c as u32, &IPRIVATE_RANGES[..])
}

fn is_iquery_not_pct_encoded(c: char) -> bool {
    is_iprivate(c) || is_ipchar_not_pct_encoded(c) || matches!(c, '/' | '?')
}

/// Consume an iquery block
fn iquery(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(many0(alt((
        take_while1(is_iquery_not_pct_encoded),
        take_while_pct_encoded,
    ))))(input)
}

fn take_while_ifragment(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(many0(alt((take_while_ipchar1, tag("/"), tag("?")))))(input)
}

/// Consume scheme characters from input and then :// or :
///
/// # Description
/// This function as it can be seen, consumes exactly one alpha and as many
/// scheme characters as there are. then it gets a slice of input(as cloned to i)
fn scheme_and_separator(input: &str) -> IResult<&str, (&str, &str), CustomError<&str>> {
    let _input = <&str>::clone(&input);
    let (input, (_first, second)) =
        pair(take_while_m_n(1, 1, is_alpha), take_while(is_scheme))(input)?;
    // "1" is for the first, its length is always 1
    let len = 1usize.saturating_add(second.len());
    if let Some(scheme) = _input.get(0..len) {
        // important that we test :// before we test for lone :
        let (input, separator) = alt((tag("://"), tag(":")))(input)?;
        return Ok((input, (scheme, separator)));
    }
    Err(nom::Err::Failure(CustomError::NoContent))
}

#[test]
fn scheme_with_separator() {
    let result = opt(scheme_and_separator)("scheme:host/path");
    assert_eq!(Ok(("host/path", Some(("scheme", ":")))), result);

    let result = opt(scheme_and_separator)("scheme://host/path");
    assert_eq!(Ok(("host/path", Some(("scheme", "://")))), result);

    let result = opt(scheme_and_separator)("no_scheme/host/path");
    assert_eq!(Ok(("no_scheme/host/path", None)), result);
}

/// Take as many pct encoded blocks as there are. a block is %XX where X is a hex digit
fn take_while_pct_encoded(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(many1(tuple((
        char('%'),
        take_while_m_n(2, 2, is_hex_digit),
    ))))(input)
}

fn ifragment(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(many0(tuple((char('#'), take_while_ifragment))))(input)
}

fn parse_ipath_abempty(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(many0(tuple((char('/'), opt(take_while_ipchar1)))))(input)
}

#[test]
fn test_ipath_abempty() {
    assert_eq!(parse_ipath_abempty("///foo/bar"), Ok(("", "///foo/bar")));
}

fn parse_ipath_absolute(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(tuple((
        char('/'),
        opt(tuple((
            take_while_ipchar1,
            many0(tuple((char('/'), opt(take_while_ipchar1)))),
        ))),
    )))(input)
}

#[test]
fn test_ipath_absolute() {
    assert_eq!(parse_ipath_absolute("/foo"), Ok(("", "/foo")));
    assert_eq!(parse_ipath_absolute("/foo/bar"), Ok(("", "/foo/bar")));
    assert_eq!(parse_ipath_absolute("/foo//bar"), Ok(("", "/foo//bar")));
}

// IRI links per RFC3987 and RFC3986
fn parse_iri(input: &str) -> IResult<&str, LinkDestination, CustomError<&str>> {
    let input_ = <&str>::clone(&input);

    // A link is [scheme] ['://'] <iauthority> [ipath] [iquery] [ifragment]
    let (input, scheme_parts) = opt(scheme_and_separator)(input)?;
    let (scheme, separator) = scheme_parts.unwrap_or(("", ""));

    // host is actually part of authority but we need it separately
    // see iauthority function description for more information
    let (input, (authority, mut host, is_ipv6_or_future)) = iauthority(input)?;

    // now with host, if we dont have a scheme we need to check it for TLD
    if scheme.is_empty() {
        ALLOWED_TOP_LEVEL_DOMAINS
            .iter()
            .find(|&&tld| host.ends_with(tld))
            .ok_or(nom::Err::Failure(CustomError::<&str>::InvalidLink))?;
    }

    let (input, path) = opt(alt((
        parse_ipath_abempty,
        parse_ipath_absolute,
        recognize(tuple((
            take_while_ipchar,
            many0(tuple((char('/'), opt(take_while_ipchar1)))),
        ))), // ipath-rootless
    )))(input)?;
    // ^ parse one of ipath-absolute or ipath-rootless or none
    // which in the third case it's down to ipath-empty(see below)
    let path = path.unwrap_or(""); // it's ipath-empty

    let (input, query) = opt(recognize(tuple((char('?'), iquery))))(input)?;
    let query = query.unwrap_or("");

    let (_, fragment) = opt(ifragment)(input)?;
    let fragment = fragment.unwrap_or("");
    let ihier_len = authority
        .len()
        .saturating_add(host.len())
        .saturating_add(path.len());
    if ihier_len == 0 {
        return Err(nom::Err::Error(CustomError::InvalidLink));
    }
    // compute final length of scheme + separator + ihier + path + query + fragment
    let mut len = scheme
        .len()
        .saturating_add(separator.len())
        .saturating_add(ihier_len)
        .saturating_add(query.len())
        .saturating_add(fragment.len());
    if let Some(link) = input_.get(0..len) {
        if link.ends_with([':', ';', '.', ',', '!']) {
            len = len.saturating_sub(1);
            if path.is_empty() && query.is_empty() && fragment.is_empty() {
                host = input_.slice(
                    scheme.len().saturating_add(separator.len())..input_.len().saturating_sub(1),
                );
            }
        }
        len = count_chars_in_complete_parenthesis(link).unwrap_or(len);
        let link = input_.slice(0..len);
        let input = input_.slice(len..);

        return Ok((
            input,
            LinkDestination {
                target: link,
                hostname: if host.is_empty() { None } else { Some(host) },
                punycode: if is_ipv6_or_future {
                    None
                } else {
                    get_puny_code_warning(link, host)
                },
                scheme: if scheme.is_empty() {
                    None
                } else {
                    Some(scheme)
                },
            },
        ));
    }
    Err(nom::Err::Failure(CustomError::NoContent))
}

// White listed links in this format: scheme:some_char like tel:+989164364485
fn parse_generic(input: &str) -> IResult<&str, LinkDestination, CustomError<&str>> {
    let i = <&str>::clone(&input);
    let (input, scheme_parts) = opt(scheme_and_separator)(input)?;
    let (scheme, _separator) = scheme_parts.unwrap_or(("", ""));
    if !is_allowed_generic_scheme(scheme) {
        return Err(nom::Err::Error(CustomError::InvalidLink));
    }

    let (input, rest) = take_while1(is_not_white_space)(input)?;
    let len = scheme.len().saturating_add(1).saturating_add(rest.len());
    if let Some(target) = i.get(0..len) {
        return Ok((
            input,
            LinkDestination {
                scheme: Some(scheme),
                target,
                hostname: None,
                punycode: None,
            },
        ));
    }
    Err(nom::Err::Failure(CustomError::NoContent))
}

pub(super) fn parse_link(input: &str) -> IResult<&str, LinkDestination, CustomError<&str>> {
    alt((parse_generic, parse_iri))(input)
}
