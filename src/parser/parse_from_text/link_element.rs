use crate::parser::link_url::LinkDestination;
use super::Element;
use crate::nom::{Offset, Slice};
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::{
    bytes::{
        complete::{tag, take, take_while1},
        streaming::take_till1,
    },
    character,
    combinator::{peek, recognize, verify},
    sequence::tuple,
    AsChar, IResult,
};
use super::base_parsers::*;

// Link syntax here is according to RFC 3986 & 3987 --Farooq


fn is_alpha(c: char) -> bool {
    let c = c as u64;
    // basically in inclusive ranges of [0x40, 0x5a] OR
    // [0x61, 0x7a]
    // TODO: order the conditions for better performance
    c >= 0x41 && c <= 0x7a && c <= 0x5a && c >= 0x61 
}
