use nom::{
    branch::alt,
    character::complete::char,
    combinator::recognize,
    sequence::tuple,
    IResult,
};

use crate::parser::{
    parse_from_text::base_parsers::CustomError,
    link_url::ip::{
        ipvfuture::ipvfuture,
        ipv6::ipv6,
    },
};


pub fn ip_literal(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    recognize(tuple((char('['), alt((ipv6, ipvfuture)), char(']'))))(input)
}
