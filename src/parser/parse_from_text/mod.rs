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
