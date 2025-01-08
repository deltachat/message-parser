use deltachat_message_parser::parser::Element::*;
use deltachat_message_parser::parser::{parse_desktop_set, parse_markdown_text, parse_only_text};

/// don't parse fediverse handles as email addresses.
/// as disscussed in https://github.com/deltachat/message-parser/issues/82

#[test]
fn text_only_fediverse_address_should_be_parsed_as_text() {
    assert_eq!(
        parse_only_text("you can reach me on @name@domain.tld!"),
        vec![
            Text("you can reach me on "),
            Text("@name@domain.tld"),
            Text("!")
        ]
    );
}

#[test]
fn desktop_set_fediverse_address_should_be_parsed_as_text() {
    assert_eq!(
        parse_desktop_set("you can reach me on @name@domain.tld!"),
        vec![
            Text("you can reach me on "),
            Text("@name@domain.tld"),
            Text("!")
        ]
    );
}

#[test]
fn markdown_fediverse_address_should_be_parsed_as_text() {
    assert_eq!(
        parse_markdown_text("you can reach me on @name@domain.tld!"),
        vec![
            Text("you can reach me on "),
            Text("@name@domain.tld"),
            Text("!")
        ]
    );
}
