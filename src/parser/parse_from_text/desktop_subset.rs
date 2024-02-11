//! desktop subset of markdown, becase this way we can already use the punycode detection of this crate
//! and also we can keep delimited and labled links in desktop

use super::base_parsers::*;
use super::base_parsers::{
    direct_delimited, is_white_space, is_white_space_but_not_linebreak, CustomError,
};
use super::markdown_elements::{delimited_email_address, delimited_link, labeled_link};
use super::text_elements::parse_text_element;
use super::Element;
use nom::{
    bytes::complete::take,
    combinator::{peek, recognize},
    IResult,
};

/// consumes all text until [parse_element] works again, internal use text instead
///
/// its output is useable on its own, always combinate this with [nom::combinator::recognize]
fn eat_desktopset_text(input: &str) -> IResult<&str, (), CustomError<&str>> {
    let mut remaining = input;
    while !remaining.is_empty() {
        // take 1, because other parsers didn't work (text is always the last used parser)
        let (remainder, taken) = take(1usize)(remaining)?;
        remaining = remainder;
        // peek if there is an element
        if peek(|input| parse_element(input, taken.chars().next()))(remaining).is_ok() {
            break;
        }
    }
    Ok((remaining, ()))
}

/// Consumes text until another parser of [parse_element] works again
///
/// used as last parser, if the others do not consume the input it consumes the input until another parser works again
/// (uses whitespace seperation to make the parsing faster)
pub(crate) fn desktopset_text(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (rest, content) = recognize(eat_desktopset_text)(input)?;
    Ok((rest, Element::Text(content)))
}

pub(crate) fn parse_element(
    input: &str,
    prev_char: Option<char>,
) -> IResult<&str, Element, CustomError<&str>> {
    // the order is important
    // generaly more specific parsers that fail/return fast should be in the front
    // But keep in mind that the order can also change how and if the parser works as intended
    if let Ok((i, elm)) = labeled_link(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = delimited_email_address(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = delimited_link(input) {
        Ok((i, elm))
    } else {
        parse_text_element(input, prev_char)
    }
}
