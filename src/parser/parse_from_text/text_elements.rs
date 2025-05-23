/// nom parsers for text elements
use crate::parser::link_url::LinkDestination;

use super::hashtag_content_char_ranges::hashtag_content_char;
use super::Element;
use nom::{
    bytes::{
        complete::{tag, take, take_while, take_while1},
        streaming::take_till1,
    },
    character::complete::char,
    combinator::{peek, recognize, verify},
    sequence::tuple,
    AsChar, IResult, Offset, Slice,
};

use super::base_parsers::CustomError;

fn linebreak(input: &str) -> IResult<&str, char, CustomError<&str>> {
    char('\n')(input)
}

fn hashtag(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, content) = recognize(tuple((char('#'), take_while1(hashtag_content_char))))(input)?;

    Ok((input, Element::Tag(content)))
}

fn not_email_address_part_char(c: char) -> bool {
    matches!(
        c,
        '@' | '\n'
            | '\r'
            | '\t'
            | ' '
            | ':'
            | ';'
            | '!'
            | '?'
            | ','
            | '('
            | ')'
            | '{'
            | '}'
            | '['
            | ']'
            | '"'
    )
}

fn email_address_part_char(c: char) -> bool {
    !not_email_address_part_char(c)
}

/// rough recognition of an email, results gets checked by a real email address parser
fn email_intern(input: &str) -> IResult<&str, (), CustomError<&str>> {
    let (input, _) = take_till1(not_email_address_part_char)(input)?;
    let (input, _) = tag("@")(input)?;
    let (input, _) = take_while1(email_address_part_char)(input)?;
    Ok((input, ()))
}

pub(crate) fn email_address(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    // basically
    // let (input, content) = recognize(email_intern)(input)?;
    // but don't eat the last char if it is a dot.
    let i = <&str>::clone(&input);
    let i2 = <&str>::clone(&input);
    let i3 = <&str>::clone(&input);
    let (input, content) = match email_intern(i) {
        Ok((mut remaining, _)) => {
            let index = i2.offset(remaining);
            let mut consumed = i2.slice(..index);
            while let Some('.') = consumed.chars().last() {
                let index = input.offset(remaining).saturating_sub(1);
                consumed = i3.slice(..index);
                remaining = input.slice(index..);
            }
            Ok((remaining, consumed))
        }
        Err(e) => Err(e),
    }?;
    // check if result is valid email
    if true {
        Ok((input, Element::EmailAddress(content)))
    } else {
        Err(nom::Err::Error(CustomError::InvalidEmail))
    }
}

// see https://github.com/deltachat/message-parser/issues/82
pub(crate) fn fediverse_address_as_text(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, consumed) = recognize(tuple((tag("@"), email_address)))(input)?;
    Ok((input, Element::Text(consumed)))
}

fn is_allowed_bot_cmd_suggestion_char(char: char) -> bool {
    match char {
        '@' | '\\' | '_' | '.' | '-' | '/' => true,
        _ => char.is_alphanum(),
    }
}

/// Bot command suggestion
fn bot_command_suggestion(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    // dc-android's: regex /(?<=^|\\s)/[a-zA-Z][a-zA-Z@\\d_/.-]{0,254}/

    let (input, content) = recognize(tuple((
        char('/'),
        verify(take(1usize), |s: &str| {
            s.chars().next().unwrap_or('.').is_alphabetic()
        }),
        verify(take_while(is_allowed_bot_cmd_suggestion_char), |s: &str| {
            s.len() < 256
        }),
    )))(input)?;
    if content.slice(1..).contains('/') {
        Ok((input, Element::Text(content)))
    } else {
        Ok((input, Element::BotCommandSuggestion(content)))
    }
}

pub(crate) fn parse_text_element(
    input: &str,
    prev_char: Option<char>,
) -> IResult<&str, Element, CustomError<&str>> {
    // the order is important
    // generaly more specific parsers that fail/return fast should be in the front
    // But keep in mind that the order can also change how and if the parser works as intended
    //
    // Also as this is the text element parser,
    // text elements parsers MUST NOT call the parser for markdown elements internally

    if let Ok((i, elm)) = hashtag(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = {
        if prev_char == Some(' ') || prev_char.is_none() {
            bot_command_suggestion(input)
        } else {
            Err(nom::Err::Error(
                CustomError::<&str>::PrecedingWhitespaceMissing,
            ))
        }
    } {
        Ok((i, elm))
    } else if let Ok((i, elm)) = fediverse_address_as_text(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = email_address(input) {
        Ok((i, elm))
    } else if let Ok((i, destination)) = LinkDestination::parse(input) {
        Ok((i, Element::Link { destination }))
    } else if let Ok((i, _)) = linebreak(input) {
        Ok((i, Element::Linebreak))
    } else {
        Err(nom::Err::Error(CustomError::NoElement))
    }
}

/// consumes all text until [parse_text_element] works again, this method is only for internal use by [text]
///
/// its output is not useable on its own, always combinate this with [nom::combinator::recognize]
fn eat_text(input: &str) -> IResult<&str, (), CustomError<&str>> {
    let mut remaining = input;
    while !remaining.is_empty() {
        // take 1, because other parsers didn't work (text is always the last used parser)
        let (remainder, taken) = take(1usize)(remaining)?;
        remaining = remainder;
        // peek if there is an element
        if peek(|input| parse_text_element(input, taken.chars().next()))(remaining).is_ok() {
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
pub(crate) fn text(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (rest, content) = recognize(eat_text)(input)?;
    Ok((rest, Element::Text(content)))
}
