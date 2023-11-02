use super::base_parsers::*;
use super::Element;

use nom::bytes::complete::take;
use nom::bytes::complete::{tag, take_while, take_while_m_n};
use nom::character::complete::satisfy;
use nom::combinator::opt;
use nom::sequence::{delimited, tuple};
use nom::AsChar;
use nom::{combinator::recognize, IResult};

const MAX_COUNTRY_LEN: usize = 3;
const MAX_AREA_LEN: usize = 10; // TODO find real number?
const MAX_LOCAL_LEN: usize = 15; // TODO find real number?

/// spaces, dots, or dashes
fn is_sdd(input: char) -> bool {
    matches!(input, ' ' | '.' | '-')
}

fn is_digit(input: char) -> bool {
    input.is_digit(10)
}

fn is_digit_or_ssd(input: char) -> bool {
    is_digit(input) || is_sdd(input)
}

fn eat_while_digit_or_sdd_but_spare_last_digit(
    input: &str,
) -> IResult<&str, &str, CustomError<&str>> {
    let (_, result) = take_while_m_n(1, MAX_LOCAL_LEN, is_digit_or_ssd)(input)?;

    for (offset, char) in result.chars().rev().enumerate() {
        // find index of last digit
        if is_digit(char.as_char()) {
            // take everything but the last digit
            let consumed_count = result
                .chars()
                .count()
                .saturating_sub(offset.saturating_add(1));
            let (remainder, digits) = take(consumed_count)(input)?;
            return Ok((remainder, digits));
        }
    }

    Err(nom::Err::Error(CustomError::UnexpectedContent))
}

fn internal_telephone_number(input: &str) -> IResult<&str, String, CustomError<&str>> {
    // reimplement the android regex rules: from PHONE in android/util/Patterns.java
    let (input, (country, area, local)) = tuple((
        opt(tuple((
            opt(tag("+")),
            take_while_m_n(1, MAX_COUNTRY_LEN, is_digit),
            take_while(is_sdd),
        ))), // +<digits><sdd>*
        opt(tuple((
            delimited(
                tag("("),
                take_while_m_n(1, MAX_AREA_LEN, is_digit),
                tag(")"),
            ),
            take_while(is_sdd),
        ))), // (<digits>)<sdd>*
        recognize(delimited(
            satisfy(is_digit),
            eat_while_digit_or_sdd_but_spare_last_digit,
            satisfy(is_digit),
        )), // <digit><digit|sdd>+<digit>
    ))(input)?;

    // construct the telephone number uri (currently used by the test in this file)
    let country = country
        .map(|(plus, digits, _)| format!("{}{digits}", plus.unwrap_or("")))
        .unwrap_or_else(|| "".to_owned());
    let area = area.map(|(digits, _)| digits).unwrap_or("");
    let local = local.replace(is_sdd, "");
    let telephone_number_uri = format!("tel:{}{}{}", country, area, local);
    Ok((input, telephone_number_uri))
}

pub(crate) fn telephone_number(input: &str) -> IResult<&str, Element, CustomError<&str>> {
    let (input, original_number) = recognize(internal_telephone_number)(input)?;
    let (_, tel_link) = internal_telephone_number(original_number)?;
    Ok((
        input,
        Element::TelephoneNumber {
            number: original_number,
            tel_link,
        },
    ))
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]

    use crate::parser::{parse_from_text::phone_numbers::telephone_number, Element};

    #[test]
    fn test_phone_numbers() {
        // from https://stackoverflow.com/a/29767609/7655232
        let test_cases = vec![
            ("(123) 456-7890", "1234567890"),
            ("(123)456-7890", "1234567890"),
            ("123-456-7890", "1234567890"),
            ("123.456.7890", "1234567890"),
            // ("1234567890", "1234567890"),
            //("+31636363634", "+31636363634"),
            ("+31 636363634", "+31636363634"),
            ("075-63546725", "07563546725"),
            // from wikipedia https://de.wikipedia.org/w/index.php?title=Rufnummer&oldid=236385081#Nationales
            ("089 1234567", "0891234567"),
            // https://www.bundesnetzagentur.de/SharedDocs/Downloads/DE/Sachgebiete/Telekommunikation/Unternehmen_Institutionen/Nummerierung/Rufnummern/Mittlg148_2021.pdf?__blob=publicationFile&v=1
            ("(0)152 28817386", "015228817386"),
            ("69 90009000", "6990009000"),
            // ("90009000", "90009000"),
            // https://en.wikipedia.org/w/index.php?title=E.123&oldid=1181303803
            ("(0607) 123 4567", "06071234567"),
            ("+22 607 123 4567", "+226071234567"),
        ];

        for (number, expected_uri) in test_cases {
            println!("testing {number}");
            assert_eq!(
                telephone_number(number).unwrap().1,
                Element::TelephoneNumber {
                    number,
                    tel_link: format!("tel:{expected_uri}")
                }
            )
        }
    }
}
