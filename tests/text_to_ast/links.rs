use super::*;
use deltachat_message_parser::parser::{parse_link, LinkDestination};

#[test]
fn link() {
    let test_cases = vec![
        "http://delta.chat",
        "http://delta.chat:8080",
        "http://localhost",
        "http://127.0.0.0",
        "https://delta.chat",
        "ftp://delta.chat",
        "https://delta.chat/en/help",
        "https://delta.chat/en/help?hi=5&e=4",
        "https://delta.chat?hi=5&e=4",
        "https://delta.chat/en/help?hi=5&e=4#section2.0",
        "https://delta#section2.0",
        "http://delta.chat:8080?hi=5&e=4#section2.0",
        "http://delta.chat:8080#section2.0",
        "mailto:delta@example.com",
        "mailto:delta@example.com?subject=hi&body=hello%20world",
        "mailto:foö@ü.chat",
        "https://ü.app#help", // TODO add more urls for testing
    ];

}
