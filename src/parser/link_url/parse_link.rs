use std::ops::RangeInclusive;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1, take_while_m_n},
    character::complete::char,
    combinator::{opt, recognize},
    multi::{many0, many1},
    sequence::tuple,
    IResult, Slice,
};

use crate::parser::{
    link_url::{
        ip::{ip_literal::ip_literal, ipv4::ipv4},
        LinkDestination, PunycodeWarning,
    },
    parse_from_text::base_parsers::CustomError,
    utils::{
        is_alpha, is_digit, is_hex_digit, is_in_one_of_ranges, is_not_white_space, is_sub_delim,
        is_unreserved,
    },
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

fn is_ireg_name_not_pct_encoded(c: char) -> bool {
    is_iunreserved(c) || is_sub_delim(c)
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
    recognize(many0(alt((
        recognize(many1(take_while_pct_encoded)),
        take_while1(is_ireg_name_not_pct_encoded),
    ))))(input)
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

/// Consume scheme characters from input
///
/// # Description
/// This function as it can be seen, consumes exactly an alpha and as many
/// scheme characters as there are. then it gets a slice of input(as cloned to i)
///
/// # Arguments
///
///  - `input` the input string
///
/// # Return value
///  (unconsumed input AND the scheme string in order) OR Error
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

/// Take as many pct encoded blocks as there are. a block is %XX where X is a hex digit
fn take_while_pct_encoded(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(many1(tuple((
        char('%'),
        take_while_m_n(2, 2, is_hex_digit),
    ))))(input)
}

/// encode a host to punycode encoded string
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

/// Returns true if host string contains non ASCII characters
fn is_puny(host: &str) -> bool {
    for ch in host.chars() {
        if !(ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-')) {
            return true;
        }
    }
    false
}

/// Return a PunycodeWarning struct if host need punycode encoding else None
pub fn get_puny_code_warning(link: &str, host: &str) -> Option<PunycodeWarning> {
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

fn ifragment(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(tuple((char('#'), take_while_ifragment)))(input)
}

// IRI links per RFC3987 and RFC3986
#[allow(clippy::arithmetic_side_effects)]
fn parse_iri(input: &str) -> IResult<&str, LinkDestination, CustomError<&str>> {
    let input_ = <&str>::clone(&input);
    // a link is <scheme> :// <iauthority> [ipath] [iquery] [ifragment]
    let (input, scheme) = scheme(input)?;
    // ^ parse scheme
    let (input, _period_double_slash) = tag("://")(input)?;
    // ^ hey do I need to explain this, too?
    let (input, (authority, mut host, is_ipv6_or_future)) = iauthority(input)?;
    // host is actually part of authority but we need it separately
    // see iauthority function description for more information
    let (input, path) = opt(alt((
        recognize(tuple((
            char('/'),
            opt(tuple((
                take_while_ipchar1,
                many0(tuple((char('/'), opt(take_while_ipchar1)))),
            ))),
        ))), // ipath-absolute
        recognize(tuple((
            take_while_ipchar,
            many0(tuple((char('/'), opt(take_while_ipchar1)))),
        ))), // ipath-rootless
    )))(input)?;
    // ^ parse one of ipath-absolute or ipath-rootless or none
    // which in the third case it's down to ipath-empty(see below)
    let path = path.unwrap_or(""); // it's ipath-empty
    let (input, query) = opt(recognize(tuple((char('?'), iquery))))(input)?;
    let (_, fragment) = opt(ifragment)(input)?;
    let query = query.unwrap_or(""); // in the case of no iquery
    let fragment = fragment.unwrap_or(""); // in the case of no ifragment
    let ihier_len = 3usize
        .saturating_add(authority.len())
        .saturating_add(host.len())
        .saturating_add(path.len());
    // compute length of authority + host + path
    let mut len = scheme
        .len()
        .saturating_add(ihier_len)
        .saturating_add(query.len())
        .saturating_add(fragment.len());
    // compute length of link which is ihier_len + scheme + query + fragment
    if let Some(link) = input_.get(0..len) {
        if link.ends_with([':', ';', '.', ',']) {
            len -= 1;
            if path.is_empty() && query.is_empty() && fragment.is_empty() {
                host = input_.slice(scheme.len() + 3..input_.len() - 1);
            }
        }

        let mut parenthes = 0usize; // ()
        let mut curly_bracket = 0usize; // {}
        let mut bracket = 0usize; // []
        let mut angle = 0usize; // <>

        for (i, ch) in link.chars().enumerate() {
            match ch {
                '(' => {
                    parenthes = parenthes.saturating_add(1);
                    if link.slice(i..).find(')').is_none() {
                        len = i;
                        break;
                    }
                }
                '{' => {
                    curly_bracket = curly_bracket.saturating_add(1);
                    if link.slice(i..).find('}').is_none() {
                        len = i;
                        break;
                    }
                }
                '[' => {
                    bracket = bracket.saturating_add(1);
                    if link.slice(i..).find(']').is_none() {
                        len = i;
                        break;
                    }
                }
                '<' => {
                    angle = angle.saturating_add(1);
                    if link.slice(i..).find('>').is_none() {
                        len = i;
                        break;
                    }
                }
                ')' => {
                    if parenthes == 0 {
                        len = i;
                        break;
                    } else {
                        parenthes -= 1;
                    }
                }
                ']' => {
                    if bracket == 0 {
                        len = i;
                        break;
                    } else {
                        bracket -= 1;
                    }
                }
                '}' => {
                    if curly_bracket == 0 {
                        len = i;
                        break;
                    } else {
                        curly_bracket -= 1;
                    }
                }
                '>' => {
                    if angle == 0 {
                        len = i;
                        break;
                    } else {
                        angle -= 1;
                    }
                }
                _ => continue,
            }
        }

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
        return Ok((
            input,
            LinkDestination {
                scheme,
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

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use crate::parser::{
        link_url::parse_link::{punycode_encode, PunycodeWarning},
        LinkDestination,
    };

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

        let test_cases_with_puny = vec!["https://ü.app#help", "http://münchen.de"];

        for input in &test_cases_no_puny {
            let (rest, link_destination) =
                LinkDestination::parse(input).expect(&format!("Test failed: {input}"));

            assert_eq!(input, &link_destination.target);
            assert_eq!(rest.len(), 0);
            assert!(link_destination.punycode.is_none());
        }

        for input in &test_cases_with_puny {
            let (rest, link_destination) =
                LinkDestination::parse(input).expect("Test failed: {input}");

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
            assert!(LinkDestination::parse(input).is_err());
        }
    }
    #[test]
    fn punycode_encode_fn() {
        assert_eq!(punycode_encode("münchen.de"), "xn--mnchen-3ya.de")
    }

    #[test]
    fn punycode_detection() {
        assert_eq!(
            LinkDestination::parse("http://münchen.de").unwrap().1,
            LinkDestination {
                hostname: Some("münchen.de"),
                target: "http://münchen.de",
                scheme: "http",
                punycode: Some(PunycodeWarning {
                    original_hostname: "münchen.de".to_owned(),
                    ascii_hostname: "xn--mnchen-3ya.de".to_owned(),
                    punycode_encoded_url: "http://xn--mnchen-3ya.de".to_owned(),
                }),
            }
        );

        assert_eq!(
            LinkDestination::parse("http://muenchen.de").unwrap().1,
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
            LinkDestination::parse("http://delta.chat").unwrap(),
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
            LinkDestination::parse("https://far.chickenkiller.com").unwrap(),
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
            LinkDestination::parse("mailto:someone@example.com").unwrap(),
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
            LinkDestination::parse("bitcoin:bc1qt3xhfvwmdqvxkk089tllvvtzqs8ts06u3u6qka")
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
            LinkDestination::parse("geo:37.786971,-122.399677")
                .unwrap()
                .1,
            LinkDestination {
                scheme: "geo",
                punycode: None,
                target: "geo:37.786971,-122.399677",
                hostname: None
            }
        );
    }
}
