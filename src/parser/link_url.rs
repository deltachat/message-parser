use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till1, take_while, take_while1},
    character::complete::char,
    character::complete::digit1,
    combinator::{consumed, opt, recognize},
    error::{ErrorKind, ParseError},
    multi::many0,
    sequence::delimited,
    sequence::tuple,
    AsChar, IResult,
};

use super::parse_from_text::base_parsers::{is_not_white_space, CustomError};

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
    target: &'a str,
    /// hostname if it was found
    hostname: Option<&'a str>,
    /// contains data for the punycode warning if punycode was detected
    /// (the host part contains non ascii unicode characters)
    punycode: Option<PunycodeWarning>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct PunycodeWarning {
    original_hostname: String,
    ascii_hostname: String,
    punycode_encoded_url: String,
}

/// determines which schemes get linkifyed
fn is_allowed_scheme(scheme: &str) -> bool {
    matches!(scheme.to_ascii_lowercase().as_ref(), "mailto" | "news")
}

impl LinkDestination<'_> {
    /// parse a link that is not in a delimited link or a labled link, just a part of normal text
    /// it has a whitelist of schemes, because otherwise
    pub(crate) fn parse_standalone_with_whitelist(
        input: &str,
    ) -> IResult<&str, LinkDestination, CustomError<&str>> {
        if let Ok((rest, (link, info))) = parse_url(input) {
            let (hostname, punycode) = match info {
                UrlInfo::CommonInternetSchemeURL {
                    has_puny_code_in_host_name,
                    hostname,
                    ascii_hostname,
                } => {
                    if has_puny_code_in_host_name {
                        (
                            Some(hostname),
                            Some(PunycodeWarning {
                                original_hostname: hostname.to_owned(),
                                punycode_encoded_url: link.replacen(hostname, &ascii_hostname, 1),
                                ascii_hostname,
                            }),
                        )
                    } else {
                        (Some(hostname), None)
                    }
                }
                UrlInfo::GenericUrl { scheme } => {
                    if !is_allowed_scheme(scheme) {
                        return Err(nom::Err::Error(CustomError::InvalidLink));
                    }
                    (None, None)
                }
            };

            Ok((
                rest,
                LinkDestination {
                    target: link,
                    hostname,
                    punycode,
                },
            ))
        } else {
            Err(nom::Err::Error(CustomError::InvalidLink))
        }
    }

    pub fn parse(input: &str) -> IResult<&str, LinkDestination, CustomError<&str>> {
        if let Ok((rest, (link, info))) = parse_url(input) {
            let (hostname, punycode) = match info {
                UrlInfo::CommonInternetSchemeURL {
                    has_puny_code_in_host_name,
                    hostname,
                    ascii_hostname,
                } => {
                    if has_puny_code_in_host_name {
                        (
                            Some(hostname),
                            Some(PunycodeWarning {
                                original_hostname: hostname.to_owned(),
                                punycode_encoded_url: link.replacen(hostname, &ascii_hostname, 1),
                                ascii_hostname,
                            }),
                        )
                    } else {
                        (Some(hostname), None)
                    }
                }
                UrlInfo::GenericUrl { .. } => (None, None),
            };

            Ok((
                rest,
                LinkDestination {
                    target: link,
                    hostname,
                    punycode,
                },
            ))
        } else {
            Err(nom::Err::Error(CustomError::InvalidLink))
        }
    }
}

#[derive(Debug, PartialEq)]
enum UrlInfo<'a> {
    /// wether url is an Common Internet Scheme URL (if it has `://`)
    CommonInternetSchemeURL {
        has_puny_code_in_host_name: bool,
        hostname: &'a str,
        ascii_hostname: String,
    },
    GenericUrl {
        scheme: &'a str,
    },
}

#[derive(Debug, PartialEq)]
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

fn is_reserved(char: char) -> bool {
    matches!(char, ';' | '/' | '?' | ':' | '@' | '&' | '=')
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn escaped_char(input: &str) -> IResult<&str, &str, LinkParseError<&str>> {
    let (input, content) = take(3usize)(input)?;
    let mut content_chars = content.chars();

    if content_chars.next() == Some('%')
        && content_chars.next().map(is_hex_digit) == Some(true)
        && content_chars.next().map(is_hex_digit) == Some(true)
    {
        Ok((input, content))
    } else {
        Err(nom::Err::Error(LinkParseError::ThisIsNotPercentEncoding))
    }
}

fn is_safe(char: char) -> bool {
    matches!(char, '$' | '-' | '_' | '.' | '+')
}

fn is_extra(char: char) -> bool {
    matches!(
        char,
        '!' | '*' | '\'' | '(' | ')' | ',' | '{' | '}' | '[' | ']' | '<' | '>'
    )
}

fn is_unreserved(char: char) -> bool {
    char.is_alphanum() || is_safe(char) || is_extra(char)
}

fn x_char_sequence(input: &str) -> IResult<&str, &str, LinkParseError<&str>> {
    //xchar          = unreserved | reserved | escape
    recognize(many0(alt((
        take_while1(is_unreserved),
        take_while1(is_reserved),
        escaped_char,
        tag("#"),
    ))))(input)
}

fn scheme_char(char: char) -> bool {
    //; the scheme is in lower case; interpreters should use case-ignore
    //scheme         = 1*[ lowalpha | digit | "+" | "-" | "." ]
    match char {
        '+' | '-' | '.' => true,
        _ => char.is_alphanum(),
    }
}

fn is_user_or_password_char(char: char) -> bool {
    match char {
        ';' | '?' | '&' | '=' => true,
        _ => is_unreserved(char),
    }
}

fn user_or_password(input: &str) -> IResult<&str, &str, LinkParseError<&str>> {
    recognize(many0(alt((
        take_while(is_user_or_password_char),
        escaped_char,
    ))))(input)
}

fn login(input: &str) -> IResult<&str, (), LinkParseError<&str>> {
    // login          = user [ ":" password ] "@"
    let (input, _) = user_or_password(input)?;
    let (input, _) = opt(tuple((char(':'), user_or_password)))(input)?;
    let (input, _) = char('@')(input)?;
    Ok((input, ()))
}

fn is_ipv6_char(char: char) -> bool {
    match char {
        ':' => true,
        _ => is_hex_digit(char),
    }
}

fn is_alphanum_or_hyphen_minus(char: char) -> bool {
    match char {
        '-' => true,
        _ => char.is_alphanum(),
    }
}
fn is_forbidden_in_idnalabel(char: char) -> bool {
    is_reserved(char) || is_extra(char) || char == '>'
}

/// creates possibility for punycodedecoded/unicode/internationalized domains
/// takes everything until reserved, extra or '>'
fn idnalabel(input: &str) -> IResult<&str, &str, LinkParseError<&str>> {
    let (input, label) = take_till1(is_forbidden_in_idnalabel)(input)?;
    Ok((input, label))
}

fn host<'a>(input: &'a str) -> IResult<&'a str, &'a str, LinkParseError<&'a str>> {
    if let Ok((input, host)) = recognize::<_, _, LinkParseError<&'a str>, _>(delimited(
        char('['),
        take_while1(is_ipv6_char),
        char(']'),
    ))(input)
    {
        // ipv6 hostnumber
        // sure the parsing here could be more specific and correct -> TODO
        Ok((input, host))
    } else if let Ok((input, host)) = recognize::<_, _, LinkParseError<&'a str>, _>(tuple((
        digit1,
        char('.'),
        digit1,
        char('.'),
        digit1,
        char('.'),
        digit1,
    )))(input)
    {
        // ipv4 hostnumber
        // sure the parsing here could be more specific and correct -> TODO
        Ok((input, host))
    } else {
        // idna hostname (valid chars until ':' or '/')
        // sure the parsing here could be more specific and correct -> TODO
        let (input, host) =
            recognize(tuple((many0(tuple((idnalabel, char('.')))), idnalabel)))(input)?;
        Ok((input, host))
    }
}

fn punycode_encode(host: &str) -> String {
    host.split('.')
        .map(|sub| {
            let mut has_non_ascii_char = false;
            for char in sub.chars() {
                if !is_alphanum_or_hyphen_minus(char) {
                    has_non_ascii_char = true;
                    break;
                }
            }
            if has_non_ascii_char {
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

fn url_intern<'a>(input: &'a str) -> IResult<&'a str, UrlInfo<'a>, LinkParseError<&'a str>> {
    let (input, scheme) = take_while1(scheme_char)(input)?;
    let (input, _) = tag(":")(input)?;

    if let Ok((input, _)) = tag::<&'a str, &'a str, LinkParseError<&'a str>>("//")(input) {
        // ip-schemepart
        // parse login
        let (input, _) = opt(login)(input)?;
        // parse host
        let (input, host) = host(input)?;
        // parse port
        let (input, _) = opt(tuple((char(':'), digit1)))(input)?;
        // parse urlpath
        let (input, _) = opt(tuple((
            alt((char('/'), char('?'), char('#'))),
            x_char_sequence,
        )))(input)?;

        let mut is_puny = false;
        for char in host.chars() {
            if !(is_alphanum_or_hyphen_minus(char) || char == '.') {
                is_puny = true;
                break;
            }
        }

        Ok((
            input,
            UrlInfo::CommonInternetSchemeURL {
                hostname: host,
                has_puny_code_in_host_name: is_puny,
                ascii_hostname: if is_puny {
                    punycode_encode(host)
                } else {
                    host.to_string()
                },
            },
        ))
    } else {
        // schemepart
        let (input, _) = take_while(is_not_white_space)(input)?;

        Ok((input, UrlInfo::GenericUrl { scheme }))
    }
}

fn parse_url(input: &str) -> IResult<&str, (&str, UrlInfo), LinkParseError<&str>> {
    consumed(url_intern)(input)
}

// TODO testcases

// ipv6 https://[::1]/

// invalid ascii domain (without non ascii char: https://-test-/hi )

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use crate::parser::link_url::{parse_url, punycode_encode, UrlInfo};

    #[test]
    fn basic_parsing() {
        let test_cases = vec![
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
            "https://ü.app#help",
            "ftp://test-test",
            "http://münchen.de",
        ];

        for input in &test_cases {
            // println!("testing {}", input);

            let (rest, (url, _)) = parse_url(input).unwrap();

            assert_eq!(input, &url);
            assert_eq!(rest.len(), 0);
        }
    }

    #[test]
    fn invalid_domains() {
        let test_cases = vec![";?:/hi", "##://thing"];

        for input in &test_cases {
            // println!("testing {}", input);
            assert!(parse_url(input).is_err());
        }
    }
    #[test]
    fn punycode_encode_fn() {
        assert_eq!(punycode_encode("münchen.de"), "xn--mnchen-3ya.de")
    }

    #[test]
    fn punycode_detection() {
        assert_eq!(
            parse_url("http://münchen.de").unwrap().1,
            (
                "http://münchen.de",
                UrlInfo::CommonInternetSchemeURL {
                    hostname: "münchen.de",
                    has_puny_code_in_host_name: true,
                    ascii_hostname: "xn--mnchen-3ya.de".to_owned()
                }
            )
        );

        assert_eq!(
            parse_url("http://muenchen.de").unwrap().1,
            (
                "http://muenchen.de",
                UrlInfo::CommonInternetSchemeURL {
                    hostname: "muenchen.de",
                    has_puny_code_in_host_name: false,
                    ascii_hostname: "muenchen.de".to_owned()
                }
            )
        );
    }
}
