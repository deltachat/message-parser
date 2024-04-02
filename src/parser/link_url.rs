use std::ops::RangeInclusive;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::{char, u8},
    combinator::{opt, recognize},
    error::{ErrorKind, ParseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{tuple, delimited},
    IResult,
};

use super::parse_from_text::{
    base_parsers::{is_not_white_space, CustomError},
    find_range::is_in_one_of_ranges,
};

// Link syntax here is according to RFC 3986 & 3987 --Farooq

///! Parsing / Validation of URLs
///
/// - hyperlinks (:// scheme)
/// - whitelisted scheme (: scheme)
///
/// for hyperlinks it also checks whether the domain contains punycode

// There are two kinds of Urls
// - Common Internet Scheme https://datatracker.ietf.org/doc/html/rfc1738#section-3.1
// - Every other url (like mailto)

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct LinkDestination<'a> {
    pub target: &'a str,
    /// hostname if it was found
    pub hostname: Option<&'a str>,
    /// contains data for the punycode warning if punycode was detected
    /// (the host part contains non ascii unicode characters)
    pub punycode: Option<PunycodeWarning>,
    /// scheme
    pub scheme: &'a str,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct PunycodeWarning {
    pub original_hostname: String,
    pub ascii_hostname: String,
    pub punycode_encoded_url: String,
}

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

impl LinkDestination<'_> {
    /// parse a link that is not in a delimited link or a labled link, just a part of normal text
    /// it has a whitelist of schemes, because otherwise
    /*
    pub(crate) fn parse_standalone_with_whitelist(
        input: &str,
    ) -> IResult<&str, LinkDestination, CustomError<&str>> {
        if let Ok((rest, link_destination)) = parse_link(input) {
            if link_destination.hostname.is_none() {
                // if it's a generic url like geo:-15.5,41.1
                if !is_allowed_generic_scheme(link_destination.scheme) {
                    Err(nom::Err::Error(CustomError::InvalidLink))
                } else {
                    Ok((rest, link_destination))
                }
            } else {
                Ok((
                    rest,
                    link_destination
                ))
            }
        } else {
            Err(nom::Err::Error(CustomError::InvalidLink))
        }
    }
*/
    pub fn parse(input: &str) -> IResult<&str, LinkDestination, CustomError<&str>> {
        if let Ok((rest, link_destination)) = parse_link(input) {
            Ok((
                rest,
                link_destination 
            ))
        } else {
            Err(nom::Err::Error(CustomError::InvalidLink))
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum LinkParseError<I> {
    Nom(I, ErrorKind),
    ThisIsNotPercentEncoding,
}

impl<I> ParseError<I> for LinkParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        LinkParseError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
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
    is_in_one_of_ranges(c as u32, &UCSCHAR_RANGES[..])
}

fn is_unreserved(c: char) -> bool {
    is_alpha(c) || is_digit(c) || is_other_unreserved(c)
}

fn is_iunreserved(c: char) -> bool {
    is_ucschar(c) || is_unreserved(c)
}

fn is_other_unreserved(c: char) -> bool {
    matches!(c, '_' | '.' | '-' | '~')
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
    is_alpha(c) || is_digit(c) || is_other_scheme(c)
}

fn is_other_scheme(c: char) -> bool {
    matches!(c, '+' | '-' | '.')
}

fn ipv4(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    let (input, ipv4_) =
        recognize(tuple((u8, char('.'), u8, char('.'), u8, char('.'), u8)))(input)?;
    Ok((input, ipv4_))
}

fn is_ireg_name_not_pct_encoded(c: char) -> bool {
    is_iunreserved(c) || is_sub_delim(c)
}

fn h16(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    take_while_m_n(1, 4, is_hex_digit)(input)
}

fn ls32(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    let result = recognize(tuple((h16, char(':'), h16)))(input);
    if result.is_err() {
        ipv4(input)
    } else {
        result
    }
}

fn h16_and_period(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(tuple((h16, char(':'))))(input)
}

fn double_period(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    tag("::")(input)
}

fn ipv6(input: &str) -> IResult<&str, &str, CustomError<&str>> {
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

fn ipvfuture(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(tuple((
        char('v'),
        take_while_m_n(1, 1, is_hex_digit),
        char('.'),
        take_while_m_n(1, 1, is_ipvfuture_last),
    )))(input)
}

fn ip_literal(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    delimited(char('['), alt((ipv6, ipvfuture)), char(']'))(input)
}

/// Parse host
///
/// # Description
///
/// Parse host. Returns the rest, the host string and a boolean indicating
/// if it is IPvFuture or IPv6.
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
    alt((
        recognize(many0(take_while_pct_encoded)),
        take_while(is_ireg_name_not_pct_encoded),
    ))(input)
}

fn iauthority(input: &str) -> IResult<&str, (&str, &str, bool), CustomError<&str>> /* (iauthority, host, bool) */
{
    let i = <&str>::clone(&input);
    let (input, userinfo) = opt(recognize(tuple((take_while_iuserinfo, char('@')))))(input)?;
    let (input, (host, is_ipv6_or_future)) = parse_host(input)?;
    let (input, port) = opt(recognize(tuple((char(':'), take_while(is_digit)))))(input)?;
    let userinfo = userinfo.unwrap_or("");
    let port = port.unwrap_or("");
    let len = userinfo.len().saturating_add(host.len()).saturating_add(port.len());
    if let Some(out) = i.get(0..len) {
        Ok((input, (out, host, is_ipv6_or_future)))
    } else {
        Err(nom::Err::Failure(CustomError::NoContent))
    }
}

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
        take_while(is_ipchar_not_pct_encoded),
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

fn iquery(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(many0(alt((
        take_while(is_iquery_not_pct_encoded),
        take_while_pct_encoded,
    ))))(input)
}

fn take_while_ifragment(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(many0(alt((
        take_while_ipchar,
        take_while_pct_encoded,
        tag("/"),
        tag("?"),
    ))))(input)
}

fn scheme(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    let i = <&str>::clone(&input);
    let (input, _first) = take_while_m_n(1, 1, is_alpha)(input)?;
    let (input, second) = take_while(is_scheme)(input)?;
    let len = 1usize.saturating_add(second.len());
    // "1" is for the first, its length is always 1
    if let Some(out) = i.get(0..len) {
        Ok((input, out))
    } else {
        Err(nom::Err::Failure(CustomError::NoContent))
    }
}

fn take_while_pct_encoded(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(many0(tuple((char('%'), take_while_m_n(2, 2, is_hex_digit)))))(input)
}

fn punycode_encode(host: &str) -> String {
    host.split('.')
        .map(|sub| {
            if is_puny(sub) {
                format!(
                    "xn--{}",
                    unic_idna_punycode::encode_str(sub)
                        .unwrap_or_else(|| "[punycode encode failed]".to_owned())
                )
            } else {
                sub.to_owned()
            }
        })
        .collect::<Vec<String>>()
        .join(".")
}
fn is_puny(host: &str) -> bool {
    for ch in host.chars() {
        if !(ch.is_ascii_alphanumeric() || ch == '.') {
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
            punycode_encoded_url: link.replacen(host, &ascii_hostname, 1),
        })
    } else {
        None
    }
}

// IRI links per RFC3987 and RFC3986
fn parse_iri(input: &str) -> IResult<&str, LinkDestination, CustomError<&str>> {
    let input_ = <&str>::clone(&input);
    let (input, scheme) = scheme(input)?;
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
        ))), // ipath-rootless
    )))(input)?;
    let path = path.unwrap_or(""); // it's ipath-empty
    let (input, query) = opt(recognize(tuple((char('?'), iquery))))(input)?;
    let (input_, fragment) = opt(recognize(tuple((char('#'), take_while_ifragment))))(input)?;
    let query = query.unwrap_or("");
    let fragment = fragment.unwrap_or("");
    let ihier_len = 2usize.saturating_add(authority.len()).saturating_add(host.len()).saturating_add(path.len());
    let len = scheme.len().saturating_add(ihier_len).saturating_add(query.len()).saturating_add(fragment.len());
    if let Some(link) = input_.get(0..len) {
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
                scheme,
            },
        ));
    }
    Err(nom::Err::Failure(CustomError::NoContent))
}

/*
// For future
fn parse_irelative_ref(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    todo!()
}
*/

// White listed links in this format: scheme:some_char like tel:+989164364485
fn parse_generic(input: &str) -> IResult<&str, LinkDestination, CustomError<&str>> {
    let i = <&str>::clone(&input);
    let (input, scheme) = scheme(input)?;
    if !is_allowed_generic_scheme(scheme) {
        return Err(nom::Err::Error(CustomError::InvalidLink));
    }
    let (input, rest) = take_while(is_not_white_space)(input)?;
    let len = scheme.len().saturating_add(rest.len());
    if let Some(target) = i.get(0..len) {
        return Ok((input, LinkDestination {
            scheme,
            target,
            hostname: None,
            punycode: None,
        }));
    }
    Err(nom::Err::Failure(CustomError::NoContent))
}

pub fn parse_link(input: &str) -> IResult<&str, LinkDestination, CustomError<&str>> {
    /*
    match parse_iri(input) {
        Ok((input, iri)) => Ok((input, iri)),
        Err(..) => parse_irelative_ref(input),
    }*/
    alt((parse_iri, parse_generic))(input)
}
// TODO testcases

// ipv6 https://[::1]/

// invalid ascii domain (without non ascii char: https://-test-/hi )

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use crate::parser::link_url::{parse_link, punycode_encode, PunycodeWarning, LinkDestination};

    #[test]
    fn basic_parsing() {
        let test_cases_no_puny = vec![
            "http://delta.chat",
            "http://delta.chat:8080",
            "http://localhost",
            "http://127.0.0.0",
            "https://[::1]/",
            "https://[::1]:9000?hi#o",
            "https://delta.chat",
            "ftp://delta.chat",
            "https://delta.chat/en/help",
            "https://delta.chat/en/help?hi=5&e=4",
            "https://delta.chat?hi=5&e=4",
            "https://delta.chat/en/help?hi=5&e=4#section2.0",
            "https://delta#section2.0",
            "http://delta.chat:8080?hi=5&e=4#section2.0",
            "http://delta.chat:8080#section2.0",
            "mailto:delta@example.com",
            "mailto:delta@example.com?subject=hi&body=hello%20world",
            "mailto:foö@ü.chat",
            "ftp://test-test",
        ];

        let test_cases_with_puny = vec![
            "https://ü.app#help",
            "http://münchen.de",
        ];

        for input in &test_cases_no_puny {
            let (rest, link_destination) = parse_link(input).expect("Test failed: {input}");

            assert_eq!(input, &link_destination.target);
            assert_eq!(rest.len(), 0);
            assert!(link_destination.punycode.is_none());
        }

        for input in &test_cases_with_puny {
            let (rest, link_destination) = parse_link(input).expect("Test failed: {input}");

            assert!(link_destination.punycode.is_some());
            assert_eq!(rest.len(), 0);
            assert_eq!(input, &link_destination.target);
        }
    }

    #[test]
    fn invalid_domains() {
        let test_cases = vec![";?:/hi", "##://thing"];

        for input in &test_cases {
            println!("testing {input}");
            assert!(parse_link(input).is_err());
        }
    }
    #[test]
    fn punycode_encode_fn() {
        assert_eq!(punycode_encode("münchen.de"), "xn--mnchen-3ya.de")
    }

    #[test]
    fn punycode_detection() {
        assert_eq!(
            parse_link("http://münchen.de").unwrap().1,
            LinkDestination {
                hostname: Some("münchen.de"),
                target: "http://münchen.de",
                scheme: "http",
                punycode: Some(PunycodeWarning {
                    original_hostname: "münchen.de".to_owned(),
                    punycode_encoded_url: "xn--mnchen-3ya.de".to_owned(),
                    ascii_hostname: "muenchen.de".to_owned(),
                }),
            }
        );

        assert_eq!(
            parse_link("http://muenchen.de").unwrap().1,
            LinkDestination {
                hostname: Some("muenchen.de"),
                target: "http://muenchen.de",
                scheme: "http",
                punycode: None,
            }
        );
    }

    #[test]
    fn common_schemes() {
        assert_eq!(
            parse_link("http://delta.chat").unwrap(),
            (
                "",
                LinkDestination {
                    hostname: Some("delta.chat"),
                    target: "http://delta.chat",
                    scheme: "http",
                    punycode: None,
                }
            )
        );
        assert_eq!(
            parse_link("https://far.chickenkiller.com").unwrap(),
            (
                "",
                LinkDestination {
                    hostname: Some("far.chickenkiller.com"),
                    target: "https://far.chickenkiller.com",
                    scheme: "https",
                    punycode: None,
                }
            )
        );
    }
    #[test]
    fn generic_schemes() {
        assert_eq!(
            parse_link("mailto:someone@example.com").unwrap(),
            (
                "",
                LinkDestination {
                    hostname: None,
                    scheme: "mailto",
                    punycode: None,
                    target: "mailto:someone@example.com"
                }
                        
            )
        );
        assert_eq!(
            parse_link("bitcoin:bc1qt3xhfvwmdqvxkk089tllvvtzqs8ts06u3u6qka")
                .unwrap()
                .1,
                LinkDestination {
                    hostname: None,
                    scheme: "bitcoin",
                    target: "bitcoin:bc1qt3xhfvwmdqvxkk089tllvvtzqs8ts06u3u6qka",
                    punycode: None,
                }
            );
        assert_eq!(
            parse_link("geo:37.786971,-122.399677").unwrap().1,
            LinkDestination {
                scheme: "geo",
                punycode: None,
                target: "geo:37.786971,-122.399677",
                hostname: None
            }
        );
    }
}
