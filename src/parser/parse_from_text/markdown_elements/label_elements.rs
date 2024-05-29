use nom::{
    bytes::complete::take,
    combinator::{peek, recognize},
    IResult,
};

use crate::parser::{
    parse_from_text::{
        base_parsers::{direct_delimited, CustomError},
        markdown_elements::inline_code,
    },
    Element,
};

/// Parsers for label in labelled links and later also labeled hashtags
/// parse elements inside of label in markdown set
pub(crate) fn parse_label_elements(input: &str) -> Vec<Element> {
    let mut result = Vec::new();
    let mut remaining = input;
    // println!("p-{}", input);
    while !remaining.is_empty() {
        // println!("r-{}", remaining);
        if let Ok((rest, element)) = parse_markdown_label_element(remaining) {
            // println!("e-{:?} - {}", element, remaining);
            remaining = rest;
            result.push(element);
        } else if let Ok((rest, element)) = markdown_label_text(remaining) {
            // println!("e-{:?} - {}", element, remaining);
            result.push(element);
            remaining = rest;
        } else {
            // println!("e-textDefault-{}", remaining);
            result.push(Element::Text(remaining));
            break;
        }
    }
    result
}

pub(crate) fn parse_markdown_label_element(
    input: &str,
) -> IResult<&str, Element, CustomError<&str>> {
    // the order is important
    // generaly more specific parsers that fail/return fast should be in the front
    // But keep in mind that the order can also change how and if the parser works as intended
    if let Ok((i, b)) = direct_delimited(input, "**") {
        Ok((i, Element::Bold(parse_label_elements(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "__") {
        Ok((i, Element::Bold(parse_label_elements(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "_") {
        Ok((i, Element::Italics(parse_label_elements(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "*") {
        Ok((i, Element::Italics(parse_label_elements(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "~~") {
        Ok((i, Element::StrikeThrough(parse_label_elements(b))))
    } else if let Ok((i, b)) = inline_code(input) {
        Ok((i, Element::InlineCode { content: b }))
    } else {
        Err(nom::Err::Error(CustomError::NoElement))
    }
}
/// consumes all text until [parse_label_element] works again, internal use text instead
///
/// its output is useable on its own, always combinate this with [nom::combinator::recognize]
fn eat_markdown_label_text(input: &str) -> IResult<&str, (), CustomError<&str>> {
    let mut remaining = input;
    while !remaining.is_empty() {
        // take 1, because other parsers didn't work (text is always the last used parser)
        let (remainder, _taken) = take(1usize)(remaining)?;
        remaining = remainder;
        // peek if there is an element
        if peek(|input| parse_markdown_label_element(input))(remaining).is_ok() {
            break;
        }
        // take until whitespace
        //remaining = take_while(|c| not_blank_space(c))(remaining)?.0;
    }
    Ok((remaining, ()))
}

/// Consumes text until another parser of [parse_label_element] works again
///
/// used as last parser, if the others do not consume the input it consumes the input until another parser works again
/// (uses whitespace seperation to make the parsing faster)
fn markdown_label_text(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (rest, content) = recognize(eat_markdown_label_text)(input)?;
    Ok((rest, Element::Text(content)))
}
