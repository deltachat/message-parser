use super::*;
use deltachat_message_parser::parser::parse_desktop_set;

#[test]
fn do_not_parse_markdown_elements() {
    assert_eq!(
        parse_desktop_set("**bold** world"),
        vec![Text("**bold** world")]
    );
    assert_eq!(
        parse_desktop_set("**_strange_ hello** world"),
        vec![Text("**_strange_ hello** world")]
    );
    assert_eq!(
        parse_desktop_set("**_strange_ hello** world"),
        vec![Text("**_strange_ hello** world")]
    );
    assert_eq!(
        parse_desktop_set("~~strikethrough~~ text ~~ notstrikethrough~~ text"),
        vec![Text("~~strikethrough~~ text ~~ notstrikethrough~~ text")]
    );
    assert_eq!(
        parse_desktop_set("~~strikethrough and **bold**, jo!~~"),
        vec![Text("~~strikethrough and **bold**, jo!~~")]
    );
    assert_eq!(
        parse_desktop_set(
            "hi there, you need to `cargo run` it.\nhi there, you need to ` cargo run ` it."
        ),
        vec![
            Text("hi there, you need to `cargo run` it."),
            Linebreak,
            Text("hi there, you need to ` cargo run ` it.")
        ]
    );
}

#[test]
fn command_suggestions() {
    let input = "/yes\n/move_a5_a6 \n/answer2_gameid or /answer__no";
    assert_eq!(
        parse_desktop_set(input),
        vec![
            BotCommandSuggestion("/yes"),
            Linebreak,
            BotCommandSuggestion("/move_a5_a6"),
            Text(" "),
            Linebreak,
            BotCommandSuggestion("/answer2_gameid"),
            Text(" or "),
            BotCommandSuggestion("/answer__no")
        ]
    );
}

#[test]
fn invalid_command_suggestions() {
    let input = "/1\n /hello world";
    assert_eq!(
        parse_desktop_set(input),
        vec![
            Text("/1"),
            Linebreak,
            Text(" "),
            BotCommandSuggestion("/hello"),
            Text(" world")
        ]
    );
}

#[test]
fn invalid_command_suggestions_too_long() {
    let input = "/dfshadfshlhjkldfskhjlsdafhkjdkhflkdfalsklhdsfdfadfsadsfuresdffdssdfsdsd\
fjhkdsfhkhdafskhjdafshkljerwnmsdbcxzgkhjdsaljwieoqruyitohsjbdgfisdyhbjasdkhaegrykasbdhfglhawefdhlj\
ghbsfznhlkrhszfdhflsdahadjsfhlkjdfaslhkdfsahljdfashjdhjskafkhljdfashjkldafshjadsfjhdasfjkldjkhfsabcnxbkzjadsfhhd";
    assert_eq!(
            parse_desktop_set(input),
            vec![
                Text("/dfshadfshlhjkldfskhjlsdafhkjdkhflkdfalsklhdsfdfadfsadsfuresdffdssdfsdsd\
fjhkdsfhkhdafskhjdafshkljerwnmsdbcxzgkhjdsaljwieoqruyitohsjbdgfisdyhbjasdkhaegrykasbdhfglhawefdhlj\
ghbsfznhlkrhszfdhflsdahadjsfhlkjdfaslhkdfsahljdfashjdhjskafkhljdfashjkldafshjadsfjhdasfjkldjkhfsabcnxbkzjadsfhhd")
            ]
        );
}

#[test]
fn invalid_command_suggestions_should_be_text() {
    let input = "read/write";
    assert_eq!(parse_desktop_set(input), vec![Text("read/write")]);
}

#[test]
fn hashtag() {
    let input =
        "#hashtag\nWhen your new here look for #noob\nIf your already an expert look for #expert";
    assert_eq!(
        parse_desktop_set(input),
        vec![
            Tag("#hashtag"),
            Linebreak,
            Text("When your new here look for "),
            Tag("#noob"),
            Linebreak,
            Text("If your already an expert look for "),
            Tag("#expert")
        ]
    );
}

#[test]
fn german_umlaut_hashtag() {
    let input = "#bücher #Ängste";
    assert_eq!(
        parse_desktop_set(input),
        vec![Tag("#bücher"), Text(" "), Tag("#Ängste")]
    );
}

#[test]
fn two_adjacent_hashtags() {
    let input = "#1#topic2";
    assert_eq!(parse_desktop_set(input), vec![Tag("#1"), Tag("#topic2")]);
}

#[test]
fn two_hashtags_seperated_by_linebreak() {
    let input = "#1\n#topic2";
    assert_eq!(
        parse_desktop_set(input),
        vec![Tag("#1"), Linebreak, Tag("#topic2")]
    );
}

#[test]
fn two_hashtags_seperated_by_tab() {
    let input = "#1\t#topic2";
    assert_eq!(
        parse_desktop_set(input),
        vec![Tag("#1"), Text("\t"), Tag("#topic2")]
    );
}

#[test]
fn email_address_standalone() {
    let test_cases = vec![
        "message.parser@example.com",
        "message-parser@delta.chat",
        "message+parser@delta.chat",
        "parser@127.0.0.0",
        "message+parser+67543@delta.chat",
        "243432mmdfsa3234@example.com",
        "617b5772c6d10feda41fc6e0e43b976c4cc9383d3729310d3dc9e1332f0d9acd@yggmail", // TODO add email test
    ];

    for input in test_cases {
        println!("testing {}", &input);
        assert_eq!(parse_desktop_set(input), vec![EmailAddress(input)]);
    }
}

#[test]
fn email_address_example() {
    assert_eq!(
        parse_desktop_set("This is an email address: message.parser@example.com\nMessage me there"),
        vec![
            Text("This is an email address: "),
            EmailAddress("message.parser@example.com"),
            Linebreak,
            Text("Message me there")
        ]
    );
}
#[test]
fn link() {
    let test_cases_no_puny = vec![
        (
            "http://delta.chat",
            http_link_no_puny("http://delta.chat", "delta.chat"),
        ),
        (
            "http://delta.chat:8080",
            http_link_no_puny("http://delta.chat:8080", "delta.chat"),
        ),
        (
            "http://localhost",
            http_link_no_puny("http://localhost", "localhost"),
        ),
        (
            "http://127.0.0.1",
            http_link_no_puny("http://127.0.0.1", "127.0.0.1"),
        ),
        (
            "https://delta.chat",
            https_link_no_puny("https://delta.chat", "delta.chat"),
        ),
        (
            "ftp://delta.chat",
            ftp_link_no_puny("ftp://delta.chat", "delta.chat"),
        ),
        (
            "https://delta.chat/en/help",
            https_link_no_puny("https://delta.chat/en/help", "delta.chat"),
        ),
        (
            "https://delta.chat?hi=5&e=4",
            https_link_no_puny("https://delta.chat?hi=5&e=4", "delta.chat"),
        ),
        (
            "https://delta.chat/en/help?hi=5&e=4#section2.0",
            https_link_no_puny(
                "https://delta.chat/en/help?hi=5&e=4#section2.0",
                "delta.chat",
            ),
        ),
        (
            "https://delta#section2.0",
            https_link_no_puny("https://delta#section2.0", "delta"),
        ),
        (
            "http://delta.chat:8080?hi=5&e=4#section2.0",
            http_link_no_puny("http://delta.chat:8080?hi=5&e=4#section2.0", "delta.chat"),
        ),
        (
            "http://delta.chat:8080#section2.0",
            http_link_no_puny("http://delta.chat:8080#section2.0", "delta.chat"),
        ),
        (
            "mailto:delta@example.com",
            mailto_link_no_puny("mailto:delta@example.com"),
        ),
        (
            "mailto:delta@example.com?subject=hi&body=hello%20world",
            mailto_link_no_puny("mailto:delta@example.com?subject=hi&body=hello%20world"),
        ),
        (
            "mailto:foö@ü.chat",
            mailto_link_no_puny("mailto:foö@ü.chat"),
        ),
        (
            "https://delta.chat/%C3%BC%C3%A4%C3%B6",
            https_link_no_puny(
                "https://delta.chat/%C3%BC%C3%A4%C3%B6",
                "delta.chat",
            )
        ),
        (
            "https://delta.chat/üäö",
            https_link_no_puny(
                "https://delta.chat/üäö",
                "delta.chat",
            )
        ),
        (
            "https://90eghtesadi.com/Keywords/Index/2031708/%D9%82%D8%B1%D8%A7%D8%B1%D8%AF%D8%A7%D8%AF-%DB%B2%DB%B5-%D8%B3%D8%A7%D9%84%D9%87-%D8%A7%DB%8C%D8%B1%D8%A7%D9%86-%D9%88-%DA%86%DB%8C%D9%86",
            // ^ I guess shame on the Iranian government of the time? --Farooq
            https_link_no_puny(
                "https://90eghtesadi.com/Keywords/Index/2031708/%D9%82%D8%B1%D8%A7%D8%B1%D8%AF%D8%A7%D8%AF-%DB%B2%DB%B5-%D8%B3%D8%A7%D9%84%D9%87-%D8%A7%DB%8C%D8%B1%D8%A7%D9%86-%D9%88-%DA%86%DB%8C%D9%86",
                "90eghtesadi.com",
            )
        ),
        (
            "https://pcworms.ir/صفحه",
            https_link_no_puny(
                "https://pcworms.ir/صفحه",
                "pcworms.ir",
            ),
        ),
        (
            "gopher://republic.circumlunar.space/1/~farooqkz",
            gopher_link_no_puny(
                "gopher://republic.circumlunar.space/1/~farooqkz",
                "republic.circumlunar.space",
            ),
        ),
    ];

    let test_cases_with_puny = [(
        "https://ü.app#help",
        https_link_no_puny("https://ü.app#help", "ü.app"),
    )];

    for (input, destination) in &test_cases_no_puny {
        println!("testing {input}");
        assert_eq!(
            parse_desktop_set(input),
            vec![Link {
                destination: destination.clone()
            }]
        );
    }

    for (input, expected_destination) in &test_cases_with_puny {
        println!("testing {input}");
        match &parse_desktop_set(input)[0] {
            Link { destination } => {
                assert_eq!(expected_destination.target, destination.target);
                assert_eq!(expected_destination.scheme, destination.scheme);
                assert_eq!(expected_destination.hostname, destination.hostname,);
                assert_eq!(destination.punycode.is_some(), true);
            }
            _ => {
                panic!();
            }
        }
    }
}

#[test]
fn test_link_example() {
    assert_eq!(
        parse_desktop_set(
            "This is an my site: https://delta.chat/en/help?hi=5&e=4#section2.0\nVisit me there"
        ),
        vec![
            Text("This is an my site: "),
            Link {
                destination: https_link_no_puny(
                    "https://delta.chat/en/help?hi=5&e=4#section2.0",
                    "delta.chat"
                )
            },
            Linebreak,
            Text("Visit me there")
        ]
    );
}

#[test]
fn delimited_email_example() {
    assert_eq!(
        parse_desktop_set("This is an my site: <hello@delta.chat>\nMessage me there"),
        vec![
            Text("This is an my site: "),
            EmailAddress("hello@delta.chat"),
            Linebreak,
            Text("Message me there")
        ]
    );
}

#[test]
fn labeled_link_should_not_work() {
    assert_eq!(
        parse_desktop_set("[a link](https://delta.chat/en/help?hi=5&e=4#section2.0)"),
        vec![LabeledLink {
            label: vec![Text("a link")],
            destination: https_link_no_puny(
                "https://delta.chat/en/help?hi=5&e=4#section2.0",
                "delta.chat",
            )
        }]
    );
    assert_eq!(
        parse_desktop_set(
            "[rich content **bold**](https://delta.chat/en/help?hi=5&e=4#section2.0)"
        ),
        vec![LabeledLink {
            label: vec![Text("rich content "), Bold(vec![Text("bold")])],
            destination: https_link_no_puny(
                "https://delta.chat/en/help?hi=5&e=4#section2.0",
                "delta.chat",
            )
        }]
    );
}

#[test]
fn labeled_link_example_should_not_work() {
    assert_eq!(
        parse_desktop_set("you can find the details [here](https://delta.chat/en/help)."),
        vec![
            Text("you can find the details "),
            LabeledLink {
                label: vec![Text("here")],
                destination: https_link_no_puny("https://delta.chat/en/help", "delta.chat")
            },
            Text(".")
        ]
    );
}

#[test]
fn inline_link_do_not_eat_last_char_if_it_is_special() {
    assert_eq!(
        parse_desktop_set("https://delta.chat,"),
        vec![
            Link {
                destination: https_link_no_puny("https://delta.chat", "delta.chat")
            },
            Text(",")
        ]
    );
    assert_eq!(
        parse_desktop_set("https://delta.chat."),
        vec![
            Link {
                destination: https_link_no_puny("https://delta.chat", "delta.chat")
            },
            Text(".")
        ]
    );
    assert_eq!(
        parse_desktop_set("https://delta.chat/page.hi"),
        vec![Link {
            destination: https_link_no_puny("https://delta.chat/page.hi", "delta.chat")
        }]
    );
}
