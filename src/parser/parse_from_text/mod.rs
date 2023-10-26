use nom::bytes::complete::take_until;

use self::base_parsers::CustomError;

use super::Element;

pub(crate) mod base_parsers;
mod desktop_subset;
pub mod hashtag_content_char_ranges;
mod markdown_elements;
mod text_elements;

/// parses text elements such as links and email addresses, excluding markdown
pub(crate) fn parse_only_text(input: &str) -> std::vec::Vec<Element> {
    let mut result = Vec::new();
    let mut remaining = input;
    // println!("p-{}", input);
    while !remaining.is_empty() {
        // println!("r-{}", remaining);
        if let Ok((rest, element)) = text_elements::parse_text_element(remaining, None) {
            // println!("e-{:?} - {}", element, remaining);
            remaining = rest;
            result.push(element);
        } else if let Ok((rest, element)) = text_elements::text(remaining) {
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

/// parses all kinds of elements, including markdown
pub(crate) fn parse_all(input: &str) -> std::vec::Vec<Element> {
    let mut result = Vec::new();
    let mut remaining = input;
    // println!("p-{}", input);
    while !remaining.is_empty() {
        // println!("r-{}", remaining);
        if let Ok((rest, element)) = markdown_elements::parse_element(remaining, None) {
            // println!("e-{:?} - {}", element, remaining);
            remaining = rest;
            result.push(element);
        } else if let Ok((rest, element)) = markdown_elements::markdown_text(remaining) {
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

/// parses delimited and labled links additional to the text elements
pub(crate) fn parse_desktop_set(input: &str) -> std::vec::Vec<Element> {
    let mut result = Vec::new();
    let mut remaining = input;
    // println!("p-{}", input);
    while !remaining.is_empty() {
        // println!("r-{}", remaining);
        if let Ok((rest, element)) = desktop_subset::parse_element(remaining, None) {
            // println!("e-{:?} - {}", element, remaining);
            remaining = rest;
            result.push(element);
        } else if let Ok((rest, element)) = desktop_subset::desktopset_text(remaining) {
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

/// Extract mentions as email addresses from a text
/// The addresses are sorted and deduplicated
pub(crate) fn extract_mention_addresses(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut remaining = input;
    while !remaining.is_empty() {
        if let Ok((rest, Element::Mention { address })) = text_elements::mention(remaining) {
            result.push(address.to_owned());
            remaining = rest;
            continue;
        }
        if let Ok((rest, _)) = take_until::<&str, &str, CustomError<&str>>(" @")(remaining) {
            remaining = rest;
        } else {
            // there is no mention anymore in this message
            break;
        }
    }
    result.sort_unstable();
    result.dedup();
    result
}
