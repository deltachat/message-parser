use nom::{
    character::complete::{u8, char},
    combinator::recognize,
    sequence::tuple,
    IResult,
};

use crate::parser::parse_from_text::base_parsers::CustomError;

pub fn ipv4(input: &str) -> IResult<&str, &str, CustomError<&str>> {
    let (input, ipv4_) =
        recognize(tuple((u8, char('.'), u8, char('.'), u8, char('.'), u8)))(input)?;
    Ok((input, ipv4_))
}
