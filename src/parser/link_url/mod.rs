mod ip;
mod parse_link;

use nom::{
    error::{ErrorKind, ParseError},
    IResult, Slice,
};

use crate::parser::{
    link_url::parse_link::{ifragment, parse_link},
    parse_from_text::base_parsers::CustomError,
};

///! Parsing / Validation of URLs
///
/// - hyperlinks (:// scheme) according to RFC3987 and RFC3988
/// - whitelisted scheme (: scheme) according to our own simple thing :)
///
/// for hyperlinks it also checks whether the domain contains punycode

// There are two kinds of Urls
// - Common Internet Scheme[1]
// - Every other url (like mailto)
// [1] RFC1738(Section 3.1), RFC3987, RFC3988 --Farooq

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
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

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct PunycodeWarning {
    pub original_hostname: String,
    pub ascii_hostname: String,
    pub punycode_encoded_url: String,
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
            Ok((rest, link_destination))
        } else {
            Err(nom::Err::Error(CustomError::InvalidLink))
        }
    }

    // This is for parsing markdown labelled links.
    pub fn parse_labelled(input: &str) -> IResult<&str, LinkDestination, CustomError<&str>> {
        match Self::parse(input) {
            Ok((mut remaining, mut link)) => {
                if let Some(first) = remaining.chars().next() {
                    if matches!(first, ';' | '.' | ',' | ':') {
                        // ^ markdown labelled links can include one of these characters at the end
                        // and it's therefore part of the link
                        let point = link.target.len().saturating_add(1);
                        link.target = input.slice(..point);
                        remaining = input.slice(point..);
                    }
                }
                Ok((remaining, link))
            }
            Err(..) => {
                let (remaining, target) = ifragment(input)?;
                Ok((
                    remaining,
                    LinkDestination {
                        target,
                        scheme: "",
                        hostname: None,
                        punycode: None,
                    },
                ))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LinkParseError<I> {
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for LinkParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        LinkParseError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}
