use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till1, take_while, take_while1},
    character::complete::char,
    character::complete::{anychar, digit1},
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
    /// contains data for the punycode warning if punycode was detected
    /// (the host part contains non ascii unicode characters)
    /// Is only shown
    punycode: Option<PunycodeWarning>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct PunycodeWarning {
    original_hostname: String,
    ascii_hostname: String,
}

/// determines which schemes get linkifyed
fn is_allowed_scheme(scheme: &str) -> bool {
    match scheme.to_ascii_lowercase().as_ref() {
        "mailto" | "news" => true,
        _ => false,
    }
}

impl LinkDestination<'_> {
    /// parse a link that is not in a delimited link or a labled link, just a part of normal text
    /// it has a whitelist of schemes, because otherwise
    pub(crate) fn parse_standalone_with_whitelist<'a>(
        input: &'a str,
    ) -> IResult<&'a str, LinkDestination<'a>, CustomError<&'a str>> {
        if let Ok((rest, (link, info))) = parse_url(input) {
            let punycode = match info {
                UrlInfo::CommonInternetSchemeURL {
                    has_puny_code_in_host_name,
                    hostname,
                    ascii_hostname,
                } => {
                    if has_puny_code_in_host_name {
                        Some(PunycodeWarning {
                            original_hostname: hostname.to_owned(),
                            ascii_hostname,
                        })
                    } else {
                        None
                    }
                }
                UrlInfo::GenericUrl { scheme } => {
                    if !is_allowed_scheme(scheme) {
                        return Err(nom::Err::Error(CustomError::InvalidLink));
                    }
                    None
                }
            };

            Ok((
                rest,
                LinkDestination {
                    target: link,
                    punycode,
                },
            ))
        } else {
            Err(nom::Err::Error(CustomError::InvalidLink))
        }
    }

    #[cfg(test)]
    pub(crate) fn for_testing<'a>(trusted_real_url: &'a str) -> LinkDestination {
        LinkDestination {
            target: trusted_real_url,
            punycode: None,
        }
    }

    pub(crate) fn parse<'a>(
        input: &'a str,
    ) -> IResult<&'a str, LinkDestination<'a>, CustomError<&'a str>> {
        if let Ok((rest, (link, info))) = parse_url(input) {
            let punycode = match info {
                UrlInfo::CommonInternetSchemeURL {
                    has_puny_code_in_host_name,
                    hostname,
                    ascii_hostname,
                } => {
                    if has_puny_code_in_host_name {
                        Some(PunycodeWarning {
                            original_hostname: hostname.to_owned(),
                            ascii_hostname,
                        })
                    } else {
                        None
                    }
                }
                UrlInfo::GenericUrl { .. } => None,
            };

            Ok((
                rest,
                LinkDestination {
                    target: link,
                    punycode,
                },
            ))
        } else {
            return Err(nom::Err::Error(CustomError::InvalidLink));
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
    InvalidDomain,
    NotIdnaLabel,
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
    match char {
        ';' | '/' | '?' | ':' | '@' | '&' | '=' => true,
        _ => false,
    }
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn escaped_char<'a>(input: &'a str) -> IResult<&'a str, &'a str, LinkParseError<&'a str>> {
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
    match char {
        '$' | '-' | '_' | '.' | '+' => true,
        _ => false,
    }
}

fn is_extra(char: char) -> bool {
    match char {
        '!' | '*' | '\'' | '(' | ')' | ',' => true,
        _ => false,
    }
}

fn is_unreserved(char: char) -> bool {
    char.is_alphanum() || is_safe(char) || is_extra(char)
}

fn x_char_sequence<'a>(input: &'a str) -> IResult<&'a str, &'a str, LinkParseError<&'a str>> {
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

fn user_or_password<'a>(input: &'a str) -> IResult<&'a str, &'a str, LinkParseError<&'a str>> {
    recognize(many0(alt((
        take_while(is_user_or_password_char),
        escaped_char,
    ))))(input)
}

fn login<'a>(input: &'a str) -> IResult<&'a str, (), LinkParseError<&'a str>> {
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

fn toplabel<'a>(input: &'a str) -> IResult<&'a str, (), LinkParseError<&'a str>> {
    let (input, first_char) = take(1usize)(input)?;
    if !first_char.chars().next().unwrap_or('0').is_alpha() {
        return Err(nom::Err::Error(LinkParseError::InvalidDomain));
    }
    let (input, rest_of_label) = opt(take_while(is_alphanum_or_hyphen_minus))(input)?;
    if let Some(rest) = rest_of_label {
        if rest.ends_with("-") {
            return Err(nom::Err::Error(LinkParseError::InvalidDomain));
        }
    }
    Ok((input, ()))
}

fn domainlabel<'a>(input: &'a str) -> IResult<&'a str, (), LinkParseError<&'a str>> {
    let (input, first_char) = take(1usize)(input)?;
    if !first_char.chars().next().unwrap_or('0').is_alphanum() {
        return Err(nom::Err::Error(LinkParseError::InvalidDomain));
    }
    let (input, rest_of_label) = opt(take_while(is_alphanum_or_hyphen_minus))(input)?;
    if let Some(rest) = rest_of_label {
        if rest.ends_with("-") {
            return Err(nom::Err::Error(LinkParseError::InvalidDomain));
        }
    }
    Ok((input, ()))
}

fn hostname<'a>(input: &'a str) -> IResult<&'a str, &'a str, LinkParseError<&'a str>> {
    recognize(tuple((many0(tuple((domainlabel, char('.')))), toplabel)))(input)
}

fn is_forbidden_in_idnalabel(char: char) -> bool {
    is_reserved(char) || is_extra(char) || char == '>'
}

/// creates possibility for punycodedecoded/unicode/internationalized domains
/// takes everything until reserved, extra or '>'
fn idnalabel<'a>(input: &'a str) -> IResult<&'a str, &'a str, LinkParseError<&'a str>> {
    let (input, label) = take_till1(is_forbidden_in_idnalabel)(input)?;
    // make sure there is atleast one non ascii char in it, fail otherwise
    let mut has_non_ascii_char = false;
    for char in label.chars() {
        if !is_alphanum_or_hyphen_minus(char) {
            has_non_ascii_char = true;
            break;
        }
    }
    if !has_non_ascii_char {
        return Err(nom::Err::Error(LinkParseError::NotIdnaLabel));
    }
    Ok((input, label))
}

fn host<'a>(input: &'a str) -> IResult<&'a str, (&'a str, bool), LinkParseError<&'a str>> {
    if let Ok((input, host)) = recognize::<_, _, LinkParseError<&'a str>, _>(delimited(
        char('['),
        take_while1(is_ipv6_char),
        char(']'),
    ))(input)
    {
        // ipv6 hostnumber
        // sure the parsing here could be more specific and correct -> TODO
        Ok((input, (host, false)))
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
        Ok((input, (host, false)))
    } else if let Ok((input, host)) = hostname(input) {
        // normal hostname
        Ok((input, (host, false)))
    } else {
        // idna hostname (valid chars until ':' or '/')
        // sure the parsing here could be more specific and correct -> TODO
        let (input, host) =
            recognize(tuple((many0(tuple((idnalabel, char('.')))), idnalabel)))(input)?;
        Ok((input, (host, true)))
    }
}

fn url_intern<'a>(input: &'a str) -> IResult<&'a str, UrlInfo<'a>, LinkParseError<&'a str>> {
    let (input, scheme) = take_while1(scheme_char)(input)?;
    let (input, _) = tag(":")(input)?;

    if let Ok((input, _)) = tag::<&'a str, &'a str, LinkParseError<&'a str>>("//")(input) {
        // ip-schemepart
        // parse login
        let (input, _) = opt(login)(input)?;
        // parse host
        let (input, (host, is_puny)) = host(input)?;
        // parse port
        let (input, _) = opt(tuple((char(':'), digit1)))(input)?;
        // parse urlpath
        let (input, _) = opt(tuple((
            alt((char('/'), char('?'), char('#'))),
            x_char_sequence,
        )))(input)?;

        Ok((
            input,
            UrlInfo::CommonInternetSchemeURL {
                hostname: host,
                has_puny_code_in_host_name: is_puny,
                ascii_hostname: if is_puny {
                    host.split('.')
                        .map(|sub| {
                            unic_idna_punycode::encode_str(sub)
                                .unwrap_or("[punycode encode failed]".to_owned())
                        })
                        .collect::<Vec<String>>()
                        .join(".")
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

fn parse_url<'a>(
    input: &'a str,
) -> IResult<&'a str, (&'a str, UrlInfo<'a>), LinkParseError<&'a str>> {
    consumed(url_intern)(input)
}

// TODO testcases

// ipv6 https://[::1]/

// invalid ascii domain (without non ascii char: https://-test-/hi )

#[cfg(test)]
mod test {
    use crate::parser::link_url::parse_url;

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
        ];

        for input in &test_cases {
            println!("testing {}", input);

            let (rest, (url, _)) = parse_url(input.clone()).unwrap();

            assert_eq!(input, &url);
            assert_eq!(rest.len(), 0);
        }
    }

    #[test]
    fn invalid_domains() {
        let test_cases = vec!["https://-test-/hi", ";?:/hi"];

        for input in &test_cases {
            println!("testing {}", input);
            assert!(parse_url(input.clone()).is_err());
        }
    }
}
