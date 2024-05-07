use nom::{
    branch::alt, character::complete::char, combinator::recognize, sequence::tuple, IResult,
};

use crate::parser::{
    link_url::ip::{ipv6::ipv6, ipvfuture::ipvfuture},
    parse_from_text::base_parsers::CustomError,
};

pub fn ip_literal(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(tuple((char('['), alt((ipv6, ipvfuture)), char(']'))))(input)
}
