//! desktop subset of markdown, becase this way we can already use the punycode detection of this crate
//! and also we can keep delimited and labled links in desktop

use super::markdown_elements::{delimited_link, labeled_link};
use super::text_elements::{parse_text_element};
use super::Element;
use super::{base_parsers::*};
use nom::{
    IResult,
};

pub(crate) fn parse_element<'a>(
    input: &'a str,
) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    // the order is important
    // generaly more specific parsers that fail/return fast should be in the front
    // But keep in mind that the order can also change how and if the parser works as intended
    if let Ok((i, elm)) = labeled_link(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = delimited_link(input) {
        Ok((i, elm))
    } else {
        parse_text_element(input)
    }
}