//! desktop subset of markdown, becase this way we can already use the punycode detection of this crate
//! and also we can keep delimited and labled links in desktop
use nom::{
    bytes::complete::{is_not, tag, take},
    combinator::{peek, recognize},
    sequence::{delimited, tuple},
    IResult,
};

use crate::parser::LinkDestination;

use super::base_parsers::CustomError;
use super::markdown_elements::{delimited_email_address, delimited_link};
use super::text_elements::parse_text_element;
use super::Element;

// [labeled](https://link)
pub(crate) fn labeled_link(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, raw_label) = delimited(tag("["), is_not("]"), tag("]"))(input)?;
    if raw_label.is_empty() {
        return Err(nom::Err::Error(CustomError::NoContent));
    }

    // in desktop set there is no element that can appear inside of a lablel
    let label = vec![Element::Text(raw_label)];

    let (input, (_, destination, _)) =
        tuple((tag("("), LinkDestination::parse_labelled, tag(")")))(input)?;

    Ok((input, Element::LabeledLink { label, destination }))
}

/// consumes all text until [parse_element] works again, this method is only for internal use by [desktopset_text]
///
/// its output is not useable on its own, always combinate this with [nom::combinator::recognize]
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
