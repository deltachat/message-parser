use crate::parser::link_url::LinkDestination;

use super::text_elements::{link, parse_text_element};
use super::Element;
use super::{base_parsers::*, parse_all};
///! nom parsers for markdown elements
use nom::{
    bytes::complete::{is_not, tag, take, take_while},
    character::complete::alphanumeric1,
    combinator::{opt, peek, recognize},
    sequence::delimited,
    IResult,
};

named!(inline_code<&str, &str>, delimited!(tag!("`"), is_not!("`"), tag!("`")));

fn code_block(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, content): (&str, &str) =
        delimited(tag("```"), nom::bytes::complete::is_not("```"), tag("```"))(input)?;

    let (content, lang) = if is_white_space(
        content
            .chars()
            .next()
            .ok_or(nom::Err::Error(CustomError::NoContent))?,
    ) {
        // no language defined
        (content, None)
    } else {
        // language defined
        let (content, lang): (&str, &str) = alphanumeric1(content)?;
        (content, Some(lang))
    };

    // expect whitespace or new line after language or beginning (if no language is defined)
    let char_in_question = content
        .chars()
        .next()
        .ok_or(nom::Err::Error(CustomError::NoContent))?;

    let content = if is_white_space_but_not_linebreak(char_in_question) {
        // remove whitespaces until newline or non whitespaces
        let (content, _) = take_while(is_white_space_but_not_linebreak)(content)?;
        // remove new line if there is one
        let (content, _) = opt(tag("\n"))(content)?;
        content
    } else {
        // remove new line if there is one
        let (content, _) = tag("\n")(content)?;
        content
    };

    // remove spaces and last newline at end
    let mut offset = 0;
    let mut c_iter = content.chars().rev();
    while is_white_space_but_not_linebreak(
        c_iter
            .next()
            .ok_or(nom::Err::Error(CustomError::NoContent))?,
    ) {
        offset += 1
    }

    if content
        .chars()
        .rev()
        .nth(offset)
        .ok_or(nom::Err::Error(CustomError::NoContent))?
        == '\n'
    {
        offset += 1
    }

    Ok((
        input,
        Element::CodeBlock {
            language: lang,
            content: &content[0..content.chars().count() - offset],
        },
    ))
}

// <https://link>
pub(crate) fn delimited_link(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, content): (&str, &str) = delimited(tag("<"), is_not(">"), tag(">"))(input)?;
    if content.is_empty() {
        return Err(nom::Err::Error(CustomError::NoContent));
    }
    let (rest, link) = link(content)?;
    if !rest.is_empty() {
        return Err(nom::Err::Error(CustomError::UnexpectedContent));
    }
    Ok((input, link))
}

// [labeled](https://link)
pub(crate) fn labeled_link(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, raw_label): (&str, &str) = delimited(tag("["), is_not("]"), tag("]"))(input)?;
    if raw_label.is_empty() {
        return Err(nom::Err::Error(CustomError::NoContent));
    }
    let label = parse_all(raw_label);

    let (input, raw_link): (&str, &str) = delimited(tag("("), is_not(")"), tag(")"))(input)?;
    if raw_link.is_empty() {
        return Err(nom::Err::Error(CustomError::NoContent));
    }
    // check if result is valid link
    let (remainder, destination) = LinkDestination::parse(raw_link)?;

    if remainder.is_empty() {
        Ok((input, Element::LabeledLink { label, destination }))
    } else {
        Err(nom::Err::Error(CustomError::InvalidLink))
    }
}

pub(crate) fn parse_element(
    input: &str,
    prev_char: Option<char>,
) -> IResult<&str, Element, CustomError<&str>> {
    // the order is important
    // generaly more specific parsers that fail/return fast should be in the front
    // But keep in mind that the order can also change how and if the parser works as intended
    if let Ok((i, b)) = direct_delimited(input, "**") {
        Ok((i, Element::Bold(parse_all(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "__") {
        Ok((i, Element::Bold(parse_all(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "_") {
        Ok((i, Element::Italics(parse_all(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "*") {
        Ok((i, Element::Italics(parse_all(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "~~") {
        Ok((i, Element::StrikeThrough(parse_all(b))))
    } else if let Ok((i, elm)) = code_block(input) {
        Ok((i, elm))
    } else if let Ok((i, b)) = inline_code(input) {
        Ok((i, Element::InlineCode { content: b }))
    } else if let Ok((i, elm)) = labeled_link(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = delimited_link(input) {
        Ok((i, elm))
    } else {
        parse_text_element(input, prev_char)
    }
}

/// consumes all text until [parse_element] works again, internal use text instead
///
/// its output is useable on its own, always combinate this with [nom::combinator::recognize]
fn eat_markdown_text(input: &str) -> IResult<&str, (), CustomError<&str>> {
    let mut remaining = input;
    while !remaining.is_empty() {
        // take 1, because other parsers didn't work (text is always the last used parser)
        let (remainder, taken) = take(1usize)(remaining)?;
        remaining = remainder;
        // peek if there is an element
        if peek(|input| parse_element(input, taken.chars().next()))(remaining).is_ok() {
            break;
        }
        // take until whitespace
        //remaining = take_while(|c| not_blank_space(c))(remaining)?.0;
    }
    Ok((remaining, ()))
}

/// Consumes text until another parser of [parse_element] works again
///
/// used as last parser, if the others do not consume the input it consumes the input until another parser works again
/// (uses whitespace seperation to make the parsing faster)
pub(crate) fn markdown_text(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (rest, content) = recognize(eat_markdown_text)(input)?;
    Ok((rest, Element::Text(content)))
}
