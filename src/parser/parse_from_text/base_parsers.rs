use std::fmt::Debug;
use icu_properties::PropertiesError;

///! Base utility parsers, used by both text and markdown parsers
use nom::{
    bytes::complete::tag,
    error::{ErrorKind, ParseError},
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
struct PropertiesErrorWrapper {
    inner: PropertiesError
}

impl PartialEq for PropertiesErrorWrapper {
    fn eq(&self, other: &Self) -> bool {
        match self.inner {
            PropertiesError::UnexpectedPropertyName => {
                match other.inner {
                    PropertiesError::UnexpectedPropertyName => { true }
                    _ => { false}
                }
            }
            PropertiesError::PropDataLoad(data_error) => {
                match other.inner  {
                    PropertiesError::PropDataLoad(data_error2) => {
                        data_error == data_error2
                    }
                    _ => { false }
                }
            }
            PropertiesError::UnknownScriptId(id) => {
                match other.inner  {
                    PropertiesError::UnknownScriptId(id2) => {
                        id == id2
                    }
                    _ => { false }
                }
            }
            PropertiesError::UnknownGeneralCategoryGroup(group) => {
                match other.inner  {
                    PropertiesError::UnknownGeneralCategoryGroup(group2) => {
                        group == group2
                    }
                    _ => { false }
                }
            }
            _ => { false }
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum CustomError<I> {
    NoContent,
    InvalidWhiteSpaceFound,
    NoElement,
    Nom(I, ErrorKind),
    InvalidEmail,
    InvalidLink,
    UnexpectedContent,
    PrecedingWhitespaceMissing,
    OptionIsUnexpectedNone,
    UnxepectedError(String),
    ICUError(PropertiesErrorWrapper),
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

pub(crate) fn is_white_space(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\t' | ' ')
}

pub(crate) fn is_not_white_space(c: char) -> bool {
    !is_white_space(c)
}

pub(crate) fn is_white_space_but_not_linebreak(c: char) -> bool {
    matches!(c, '\t' | ' ')
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
