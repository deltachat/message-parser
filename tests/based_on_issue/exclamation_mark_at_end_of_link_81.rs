use deltachat_message_parser::parser::Element::*;
use deltachat_message_parser::parser::{parse_desktop_set, parse_markdown_text, parse_only_text};

use crate::text_to_ast::https_link_no_puny;

/// don't eat/consume the ! at the end of a link
/// as disscussed in https://github.com/deltachat/message-parser/issues/81

#[test]
fn text_only() {
    assert_eq!(
        parse_only_text("This is an my site: https://delta.chat!"),
        vec![
            Text("This is an my site: "),
            Link {
                destination: https_link_no_puny("https://delta.chat", "delta.chat",)
            },
            Text("!")
        ]
    );
    assert_eq!(
        parse_only_text("This is an my site: https://delta.chat#!test"),
        vec![
            Text("This is an my site: "),
            Link {
                destination: https_link_no_puny("https://delta.chat#!test", "delta.chat",)
            }
        ]
    );
}

#[test]
fn desktop_set() {
    assert_eq!(
        parse_desktop_set("This is an my site: https://delta.chat!"),
        vec![
            Text("This is an my site: "),
            Link {
                destination: https_link_no_puny("https://delta.chat", "delta.chat",)
            },
            Text("!")
        ]
    );
}

#[test]
fn desktop_set_negative() {
    assert_eq!(
        parse_desktop_set("This is an my site: https://delta.chat#!test"),
        vec![
            Text("This is an my site: "),
            Link {
                destination: https_link_no_puny("https://delta.chat#!test", "delta.chat",)
            }
        ]
    );
}

#[test]
fn markdown() {
    assert_eq!(
        parse_markdown_text("This is an my site: https://delta.chat!"),
        vec![
            Text("This is an my site: "),
            Link {
                destination: https_link_no_puny("https://delta.chat", "delta.chat",)
            },
            Text("!")
        ]
    );
}
#[test]
fn markdown_negative() {
    assert_eq!(
        parse_markdown_text("This is an my site: https://delta.chat#!test"),
        vec![
            Text("This is an my site: "),
            Link {
                destination: https_link_no_puny("https://delta.chat#!test", "delta.chat",)
            }
        ]
    );
}

#[test]
fn still_take_whole_link_in_labled_links() {
    assert_eq!(
        parse_markdown_text("This is an my [site](https://delta.chat/!)"),
        vec![
            Text("This is an my "),
            LabeledLink {
                label: vec![Text("site")],
                destination: https_link_no_puny("https://delta.chat/!", "delta.chat",)
            }
        ]
    );
}
