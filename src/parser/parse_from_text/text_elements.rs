///! nom parsers for text elements
use crate::parser::link_url::LinkDestination;

use super::base_parsers::*;
use super::Element;
use crate::nom::{Offset, Slice};
use nom::bytes::complete::take_while;
use nom::{
    bytes::{
        complete::{tag, take, take_while1},
        streaming::take_till1,
    },
    character::{self, streaming::char},
    combinator::{peek, recognize, verify},
    sequence::tuple,
    AsChar, IResult,
};

named!(linebreak<&str, char>, char!('\n'));

fn hashtag_content_char(c: char) -> bool {
    // !(is_white_space(c) || c == '#')
    // simpler parsing for now, see https://github.com/deltachat/message-parser/issues/8
    c.is_alphanum()
}

fn hashtag(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, content) = recognize(tuple((
        character::complete::char('#'),
        take_while1(hashtag_content_char),
    )))(input)?;

    Ok((input, Element::Tag(content)))
}

fn not_email_address_part_char(c: char) -> bool {
    matches!(c, '@' | '\n' | '\r' | '\t' | ' ' | ':')
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

fn email_address(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, content) = recognize(email_intern)(input)?;
    // check if result is valid email
    if true {
        Ok((input, Element::EmailAddress(content)))
    } else {
        Err(nom::Err::Error(CustomError::InvalidEmail))
    }
}

fn not_link_part_char(c: char) -> bool {
    !matches!(c, ':' | '\n' | '\r' | '\t' | ' ')
}

/// rough recognition of an link, results gets checked by a real link parser
fn link_intern(input: &str) -> IResult<&str, (), CustomError<&str>> {
    let (input, _) = take_while1(not_link_part_char)(input)?;
    let (input, _) = tag(":")(input)?;
    let i = <&str>::clone(&input);
    let (remaining, consumed) = take_while1(is_not_white_space)(i)?;

    let mut parentheses_count = 0usize; // ()
    let mut curly_brackets_count = 0usize; // {}
    let mut brackets_count = 0usize; // []
    let mut angle_brackets = 0usize; // <>

    let mut alternative_offset = None;
    for (i, char) in consumed.chars().enumerate() {
        match char {
            '(' => {
                parentheses_count += 1;
                // if there is no closing bracket in the link, then don't take the bracket as a part of the link
                if (<&str>::clone(&consumed)).slice(i..).find(')').is_none() {
                    alternative_offset = Some(i);
                    break;
                }
            }
            '{' => {
                curly_brackets_count += 1;
                // if there is no closing bracket in the link, then don't take the bracket as a part of the link
                if (<&str>::clone(&consumed)).slice(i..).find('}').is_none() {
                    alternative_offset = Some(i);
                    break;
                }
            }
            '[' => {
                brackets_count += 1;
                // if there is no closing bracket in the link, then don't take the bracket as a part of the link
                if (<&str>::clone(&consumed)).slice(i..).find(']').is_none() {
                    alternative_offset = Some(i);
                    break;
                }
            }
            '<' => {
                angle_brackets += 1;
                // if there is no closing bracket in the link, then don't take the bracket as a part of the link
                if (<&str>::clone(&consumed)).slice(i..).find('>').is_none() {
                    alternative_offset = Some(i);
                    break;
                }
            }
            ')' => {
                if parentheses_count == 0 {
                    alternative_offset = Some(i);
                    break;
                } else {
                    parentheses_count -= 1;
                }
            }
            '}' => {
                if curly_brackets_count == 0 {
                    alternative_offset = Some(i);
                    break;
                } else {
                    curly_brackets_count -= 1;
                }
            }
            ']' => {
                if brackets_count == 0 {
                    alternative_offset = Some(i);
                    break;
                } else {
                    brackets_count -= 1;
                }
            }
            '>' => {
                if angle_brackets == 0 {
                    alternative_offset = Some(i);
                    break;
                } else {
                    angle_brackets -= 1;
                }
            }
            _ => continue,
        }
    }

    if let Some(offset) = alternative_offset {
        let remaining = input.slice(offset..);
        Ok((remaining, ()))
    } else {
        Ok((remaining, ()))
    }
}

pub(crate) fn link(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    // basically
    //let (input, content) = recognize(link_intern)(input)?;
    // but don't eat the last char if it is one of these: `.,;:`
    let i = <&str>::clone(&input);
    let i2 = <&str>::clone(&input);
    let i3 = <&str>::clone(&input);
    let (input, content) = match link_intern(i) {
        Ok((remaining, _)) => {
            let index = i2.offset(remaining);
            let consumed = i2.slice(..index);
            match consumed.chars().last() {
                Some(c) => match c {
                    '.' | ',' | ':' | ';' => {
                        let index = input.offset(remaining) - 1;
                        let consumed = i3.slice(..index);
                        let remaining = input.slice(index..);
                        Ok((remaining, consumed))
                    }
                    _ => Ok((remaining, consumed)),
                },
                _ => Ok((remaining, consumed)),
            }
        }
        Err(e) => Err(e),
    }?;

    // check if result is valid link
    let (remainder, destination) = LinkDestination::parse_standalone_with_whitelist(content)?;

    if remainder.is_empty() {
        Ok((input, Element::Link { destination }))
    } else {
        Err(nom::Err::Error(CustomError::InvalidLink))
    }
}

fn is_allowed_bot_cmd_suggestion_char(char: char) -> bool {
    match char {
        '@' | '\\' | '_' | '/' | '.' | '-' => true,
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

    Ok((input, Element::BotCommandSuggestion(content)))
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
    } else if let Ok((i, elm)) = email_address(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = link(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = {
        if prev_char == Some(' ') || prev_char == None {
            bot_command_suggestion(input)
        } else {
            Err(nom::Err::Error(CustomError::PrecedingWhitespaceMissing))
        }
    } {
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
