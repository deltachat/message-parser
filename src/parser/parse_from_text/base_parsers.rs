use std::fmt::Debug;

// Base utility parsers, used by both text and markdown parsers
use nom::{
    bytes::complete::tag,
    error::{ErrorKind, ParseError},
    sequence::delimited,
    IResult,
};

use crate::parser::utils::is_white_space;

#[derive(Debug, PartialEq, Eq)]
pub enum CustomError<I> {
    NoContent,
    InvalidWhiteSpaceFound,
    NoElement,
    Nom(I, ErrorKind),
    InvalidEmail,
    InvalidLink,
    InvalidLinkNoTLD,
    UnexpectedContent,
    PrecedingWhitespaceMissing,
    OptionIsUnexpectedNone,
    UnxepectedError(String),
}

impl<I> ParseError<I> for CustomError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        CustomError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

pub trait IntoCustomError<I, T> {
    fn into_result(self) -> Result<T, nom::Err<CustomError<I>>>;
}

impl<I, T> IntoCustomError<I, T> for Option<T> {
    fn into_result(self: Option<T>) -> Result<T, nom::Err<CustomError<I>>> {
        match self {
            Some(v) => Ok(v),
            None => Err(nom::Err::Error(CustomError::OptionIsUnexpectedNone)),
        }
    }
}

impl<I, T, E: Debug> IntoCustomError<I, T> for Result<T, E> {
    fn into_result(self: Result<T, E>) -> Result<T, nom::Err<CustomError<I>>> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(nom::Err::Error(CustomError::UnxepectedError(format!(
                "{:?}",
                err
            )))),
        }
    }
}

/// delimited no whitespace start or end
pub(crate) fn direct_delimited<'a>(
    input: &'a str,
    tag_str: &str,
) -> IResult<&'a str, &'a str, CustomError<&'a str>> {
    let (input, content): (&str, &str) = delimited(
        tag(tag_str),
        nom::bytes::complete::is_not(tag_str),
        tag(tag_str),
    )(input)?;
    if content.is_empty() {
        return Err(nom::Err::Error(CustomError::NoContent));
    }
    if is_white_space(content.chars().next().into_result()?)
        || is_white_space(content.chars().last().into_result()?)
    {
        return Err(nom::Err::Error(CustomError::InvalidWhiteSpaceFound));
    }
    Ok((input, content))
}

/*
impl From<PropertiesError> for Err<CustomError<I>> {
    fn from(_: I, perror: PropertiesError) {
        nom::Err(CustomError::ICUError(perror))
    }
}
*/
/*
impl From<nom::Err<nom::err::Error<I>> for nom::Err<CustomError<I>> {
    fn from(input: I, code: ErrorKind) -> nom::Err<CustomError<I>> {
        nom::Err(CustomError::Nom(input, code)
    }
}*/
