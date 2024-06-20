use nom::{
    bytes::complete::{is_not, tag, take, take_while},
    character::complete::alphanumeric1,
    combinator::{opt, peek, recognize},
    sequence::{delimited, tuple},
    IResult,
};

use super::{base_parsers::*, parse_all};
use crate::parser::{
    link_url::LinkDestination,
    parse_from_text::{
        base_parsers::direct_delimited,
        text_elements::{email_address, parse_text_element},
        Element,
    },
    utils::{
        is_unicode_punctuation, is_unicode_white_space, is_white_space,
        is_white_space_but_not_linebreak,
    },
};

mod label_elements;
use label_elements::parse_label_elements;

pub(crate) fn inline_code(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    delimited(tag("`"), is_not("`"), tag("`"))(input)
}

pub(crate) fn code_block(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, content): (&str, &str) = delimited(tag("```"), is_not("```"), tag("```"))(input)?;

    // parse language
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

    // expect white_space or new line after language or beginning (if no language is defined)
    let char_in_question = content
        .chars()
        .next()
        .ok_or(nom::Err::Error(CustomError::NoContent))?;

    // remove starting white_space and first newline (if there is any).
    let content = if is_white_space_but_not_linebreak(char_in_question) {
        // remove white_spaces until newline or non white_spaces
        let (content, _) = take_while(is_white_space_but_not_linebreak)(content)?;
        // remove new line if there is one
        let (content, _) = opt(tag("\n"))(content)?;
        content
    } else {
        // remove new line if there is one
        let (content, _) = tag("\n")(content)?;
        content
    };

    // remove spaces and newlines at end of content
    let mut offset: usize = 0;
    let mut c_iter = content.chars().rev();
    while is_white_space(
        c_iter
            .next()
            .ok_or(nom::Err::Error(CustomError::NoContent))?,
    ) {
        offset = offset.saturating_add(1);
    }
    Ok((
        input,
        Element::CodeBlock {
            language: lang,
            content: content
                .get(0..content.len().saturating_sub(offset))
                .into_result()?,
        },
    ))
}

// <hello@delta.chat>
pub(crate) fn delimited_email_address(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, content): (&str, &str) = delimited(tag("<"), is_not(">"), tag(">"))(input)?;
    if content.is_empty() {
        return Err(nom::Err::Error(CustomError::NoContent));
    }
    let (rest, email) = email_address(content)?;
    if !rest.is_empty() {
        return Err(nom::Err::Error(CustomError::UnexpectedContent));
    }
    Ok((input, email))
}

// <https://link>
pub(crate) fn delimited_link(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, (_, destination, _)): (&str, (&str, LinkDestination, &str)) =
        tuple((tag("<"), LinkDestination::parse_labelled, tag(">")))(input)?;
    Ok((input, Element::Link { destination }))
}

// [labeled](https://link)
pub(crate) fn labeled_link(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, raw_label): (&str, &str) = delimited(tag("["), is_not("]"), tag("]"))(input)?;
    if raw_label.is_empty() {
        return Err(nom::Err::Error(CustomError::NoContent));
    }
    // the list of elements that can appear inside of a label is restricted
    // clickable elements make no sense there.
    let label = parse_label_elements(raw_label);

    let (input, (_, destination, _)) =
        tuple((tag("("), LinkDestination::parse_labelled, tag(")")))(input)?;

    Ok((input, Element::LabeledLink { label, destination }))
}

/*
 * For description on how these functions(parse_italics and parse_bold) work
 * refer to this link: https://spec.commonmark.org/0.31.2/#emphasis-and-strong-emphasis
 */


macro_rules! left_flanking {
    ($next_char: expr, $prev_char: expr) => {
        is_unicode_white_space($next_char) && ( // followed by whitespace and ...
            !is_unicode_punctuation($next_char) || ( // not followed by punct or ...
                is_unicode_punctuation($next_char) && ( // followed by punct or ...
                    $prev_char.is_none() || 
                    is_unicode_punctuation($prev_char.unwrap()) ||
                    is_unicode_white_space($prev_char.unwrap())
                    // preceded by whitespace or punct
                    /* 
                     * Note that order here is important. Iff prev_char is Some(=not None),
                     * unwrap will be executed and won't panic.
                     * On the other hand, iff it's None, the rest won't be run and again
                     * no panic happens. The same goes for right flanking. --Farooq
                     */
                )
            )
        )
    }
}

macro_rules! right_flanking {
    ($prev_char: expr, $next_char: expr) => {
        is_unicode_white_space($next_char) && (
            !is_unicode_punctuation($next_char) || (
                is_unicode_punctuation($next_char) && (
                    is_unicode_white_space($prev_char) || 
                    is_unicode_punctuation($prev_char)
                )
            )
        )
    }
}
fn parse_italics(input: &str, prev_char: Option<char>) -> IResult<&str, &str, CustomError<&str>> {
    let (input_, (content, tag_str)) = direct_delimited(input, &["_", "*"][..])?;
    let is_wspace = is_unicode_white_space;
    let is_punct = is_unicode_punctuation;
    let is_start_left_flanking: bool = left_flanking!(content.chars().last().unwrap_or('\0'), prev_char);
    let is_start_right_flanking: bool = right_flanking!(
        prev_char.unwrap_or('\0'),
        content.chars().next().unwrap_or('\0')
    );
    let is_end_left_flanking: bool = left_flanking!(input_.chars().last().unwrap_or('\0'), content.chars().next());
    let is_end_right_flanking: bool = right_flanking!(content.chars().last().unwrap_or('\0'), input_.chars().next().unwrap_or('\0'));
    if tag_str == "*" && is_start_left_flanking && is_end_right_flanking {
        return Ok((input_, content));
    } else if is_start_left_flanking && (!is_start_right_flanking || (is_start_right_flanking && (prev_char.is_some() && is_unicode_punctuation(prev_char.unwrap())))) && is_end_right_flanking && (!is_end_left_flanking || (is_end_left_flanking && prev_char.is_some() && input_.ends_with(is_unicode_punctuation))) {
        return Ok((input_, content));
    }
    Err(nom::Err::Error(CustomError::UnexpectedContent))
}

fn parse_bold(input: &str, prev_char: Option<char>) -> IResult<&str, &str, CustomError<&str>> {}

pub(crate) fn parse_element(
    input: &str,
    prev_char: Option<char>,
) -> IResult<&str, Element, CustomError<&str>> {
    // the order is important
    // generaly more specific parsers that fail/return fast should be in the front
    // But keep in mind that the order can also change how and if the parser works as intended
    if let Ok((i, b)) = parse_bold(input, prev_char) {
        Ok((i, Element::Bold(parse_all(b))))
    } else if let Ok((i, b)) = parse_italics(input, prev_char) {
        Ok((i, Element::Italics(parse_all(b))))
    } else if let Ok((i, (b, _tag_str))) = direct_delimited(input, &["~~"][..]) {
        Ok((i, Element::StrikeThrough(parse_all(b))))
    } else if let Ok((i, elm)) = code_block(input) {
        Ok((i, elm))
    } else if let Ok((i, b)) = inline_code(input) {
        Ok((i, Element::InlineCode { content: b }))
    } else if let Ok((i, elm)) = labeled_link(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = delimited_email_address(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = delimited_link(input) {
        Ok((i, elm))
    } else {
        parse_text_element(input, prev_char)
    }
}

/// consumes all text until [parse_element] works again, this method is only for internal use by [markdown_text]
///
/// its output is not useable on its own, always combinate this with [nom::combinator::recognize]
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
        // take until white_space
        //remaining = take_while(|c| not_blank_space(c))(remaining)?.0;
    }
    Ok((remaining, ()))
}

/// Consumes text until another parser of [parse_element] works again
///
/// used as last parser, if the others do not consume the input it consumes the input until another parser works again
/// (uses white_space seperation to make the parsing faster)
pub(crate) fn markdown_text(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (rest, content) = recognize(eat_markdown_text)(input)?;
    Ok((rest, Element::Text(content)))
}
