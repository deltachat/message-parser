use crate::parser::link_url::LinkDestination;

use super::base_parsers::*;
use super::Element;
use nom::AsChar;
///! nom parsers for text elements
use nom::{
    bytes::{
        complete::{tag, take, take_while1},
        streaming::take_till1,
    },
    character::{self},
    combinator::{peek, recognize},
    IResult,
};

named!(linebreak<&str, char>, char!('\n'));

fn hashtag_content_char(c: char) -> bool {
    // !(is_white_space(c) || c == '#')
    // simpler parsing for now, see https://github.com/deltachat/message-parser/issues/8
    c.is_alphanum()
}

fn hashtag<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    let (input, _) = character::complete::char('#')(input)?;
    let (input, content) = take_while1(hashtag_content_char)(input)?;

    Ok((input, Element::Tag(content)))
}

fn not_email_address_part_char(c: char) -> bool {
    match c {
        '@' | '\n' | '\r' | '\t' | ' ' | ':' => true,
        _ => false,
    }
}

fn email_address_part_char(c: char) -> bool {
    !not_email_address_part_char(c)
}

/// rough recognition of an email, results gets checked by a real email address parser
fn email_intern<'a>(input: &'a str) -> IResult<&'a str, (), CustomError<&'a str>> {
    let (input, _) = take_till1(not_email_address_part_char)(input)?;
    let (input, _) = tag("@")(input)?;
    let (input, _) = take_while1(email_address_part_char)(input)?;
    Ok((input, ()))
}

fn email_address<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    let (input, content) = recognize(email_intern)(input)?;
    // check if result is valid email
    if true {
        Ok((input, Element::EmailAddress(content)))
    } else {
        Err(nom::Err::Error(CustomError::InvalidEmail))
    }
}

fn not_link_part_char(c: char) -> bool {
    match c {
        ':' | '\n' | '\r' | '\t' | ' ' => false,
        _ => true,
    }
}

/// rough recognition of an link, results gets checked by a real link parser
fn link_intern<'a>(input: &'a str) -> IResult<&'a str, (), CustomError<&'a str>> {
    let (input, _) = take_while1(not_link_part_char)(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = take_while1(is_not_white_space)(input)?;
    Ok((input, ()))
}

pub(crate) fn link<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    let (input, content) = recognize(link_intern)(input)?;

    // check if result is valid link
    let (remainder, destination) = LinkDestination::parse_standalone_with_whitelist(content)?;

    if remainder.len() == 0 {
        Ok((input, Element::Link { destination }))
    } else {
        Err(nom::Err::Error(CustomError::InvalidLink))
    }
}

pub(crate) fn parse_text_element<'a>(
    input: &'a str,
) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    // the order is important
    // generaly more specific parsers that fail/return fast should be in the front
    // But keep in mind that the order can also change how and if the parser works as intended
    //
    // Also as this is the text element parser,
    // text elements parsers MUST NOT call the parser for markdown elements internally

    if let Ok((i, elm)) = hashtag(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = email_address(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = link(input) {
        Ok((i, elm))
    } else if let Ok((i, _)) = linebreak(input) {
        Ok((i, Element::Linebreak))
    } else {
        Err(nom::Err::Error(CustomError::NoElement))
    }
}

/// consumes all text until [parse_text_element] works again, internal use text instead
///
/// its output is useable on its own, always combinate this with [nom::combinator::recognize]
fn eat_text<'a>(input: &'a str) -> IResult<&'a str, (), CustomError<&'a str>> {
    let mut remaining = input;
    while remaining.len() > 0 {
        // take 1, because other parsers didn't work (text is always the last used parser)
        remaining = take(1usize)(remaining)?.0;
        // peek if there is an element
        if peek(parse_text_element)(remaining).is_ok() {
            break;
        }
        // take until whitespace
        //remaining = take_while(|c| not_blank_space(c))(remaining)?.0;
    }
    Ok((remaining, ()))
}

/// Consumes text until another parser of [parse_text_element] works again
///
/// used as last parser, if the others do not consume the input it consumes the input until another parser works again
/// (uses whitespace seperation to make the parsing faster)
pub(crate) fn text<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    let (rest, content) = recognize(eat_text)(input)?;
    Ok((rest, Element::Text(content)))
}
