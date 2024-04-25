use super::*;
use deltachat_message_parser::parser::parse_markdown_text;
use deltachat_message_parser::parser::LinkDestination;

#[test]
fn bold_capitalized_command_suggestion() {
    let input = "**/TELL** world";
    assert_eq!(
        parse_markdown_text(input),
        vec![Bold(vec![BotCommandSuggestion("/TELL")]), Text(" world")]
    );
}

#[test]
fn bold_command_suggestion() {
    let input = "**/yes** - write yes to the bot";
    assert_eq!(
        parse_markdown_text(input),
        vec![
            Bold(vec![BotCommandSuggestion("/yes")]),
            Text(" - write yes to the bot")
        ]
    );
}

#[test]
fn command_suggestions() {
    let input = "/yes\n/move_a5_a6 \n/answer2_gameid or /answer__no";
    assert_eq!(
        parse_markdown_text(input),
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
        parse_markdown_text(input),
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
            parse_markdown_text(input),
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
    assert_eq!(parse_markdown_text(input), vec![Text("read/write")]);
}

#[test]
fn bold() {
    let input = "**hello** world";
    assert_eq!(
        parse_markdown_text(input),
        vec![Bold(vec![Text("hello")]), Text(" world")]
    );
}
#[test]
fn not_bold() {
    let input = "**\nshould not be bold\n**";
    assert_eq!(
        parse_markdown_text(input),
        vec![
            Text("**"),
            Linebreak,
            Text("should not be bold"),
            Linebreak,
            Text("**")
        ]
    );
}

#[test]
fn italics() {
    let input = "_hi_ world";
    assert_eq!(
        parse_markdown_text(input),
        vec![Italics(vec![Text("hi")]), Text(" world")]
    );
}
#[test]
fn nested_bold_italics() {
    let input = "**_strange_ hello** world";
    assert_eq!(
        parse_markdown_text(input),
        vec![
            Bold(vec![Italics(vec![Text("strange")]), Text(" hello"),]),
            Text(" world")
        ]
    );
}
#[test]
fn nested_bold_italics2() {
    let input = "**hello _strange_** world";
    assert_eq!(
        parse_markdown_text(input),
        vec![
            Bold(vec![Text("hello "), Italics(vec![Text("strange")])]),
            Text(" world")
        ]
    );
}
#[test]
fn bold_italics_and_linebreak() {
    let input = "**bold**\ngreen\n\t**_lorem_ ipsum**";
    assert_eq!(
        parse_markdown_text(input),
        vec![
            Bold(vec![Text("bold")]),
            Linebreak,
            Text("green"),
            Linebreak,
            Text("\t"),
            Bold(vec![Italics(vec![Text("lorem")]), Text(" ipsum")])
        ]
    );
}

#[test]
fn strikethrough() {
    let input = "~~strikethrough~~ text ~~ notstrikethrough~~ text";
    assert_eq!(
        parse_markdown_text(input),
        vec![
            StrikeThrough(vec![Text("strikethrough")]),
            Text(" text ~~ notstrikethrough~~ text")
        ]
    );
}
#[test]
fn strikethrough_with_bold_inside() {
    let input = "~~strikethrough and **bold**, jo!~~";
    assert_eq!(
        parse_markdown_text(input),
        vec![StrikeThrough(vec![
            Text("strikethrough and "),
            Bold(vec![Text("bold")]),
            Text(", jo!")
        ])]
    );
}
#[test]
fn inline_code() {
    let input = "hi there, you need to `cargo run` it.\nhi there, you need to ` cargo run ` it.";
    assert_eq!(
        parse_markdown_text(input),
        vec![
            Text("hi there, you need to "),
            InlineCode {
                content: "cargo run"
            },
            Text(" it."),
            Linebreak,
            Text("hi there, you need to "),
            InlineCode {
                content: " cargo run "
            },
            Text(" it.")
        ]
    );
}

#[test]
fn hashtag() {
    let input =
        "#hashtag\nWhen your new here look for #noob\nIf your already an expert look for #expert";
    assert_eq!(
        parse_markdown_text(input),
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
        parse_markdown_text(input),
        vec![Tag("#bücher"), Text(" "), Tag("#Ängste")]
    );
}

#[test]
fn two_adjacent_hashtags() {
    let input = "#1#topic2";
    assert_eq!(parse_markdown_text(input), vec![Tag("#1"), Tag("#topic2")]);
}

#[test]
fn two_hashtags_seperated_by_linebreak() {
    let input = "#1\n#topic2";
    assert_eq!(
        parse_markdown_text(input),
        vec![Tag("#1"), Linebreak, Tag("#topic2")]
    );
}

#[test]
fn two_hashtags_seperated_by_tab() {
    let input = "#1\t#topic2";
    assert_eq!(
        parse_markdown_text(input),
        vec![Tag("#1"), Text("\t"), Tag("#topic2")]
    );
}

#[test]
fn bold_hashtag() {
    let input = "**#hashTagInsideOfBold**";
    assert_eq!(
        parse_markdown_text(input),
        vec![Bold(vec![Tag("#hashTagInsideOfBold")])]
    );
}

#[test]
fn code_fence_block_single_line_with_lang() {
    assert_eq!(
        parse_markdown_text("```js alert('Hello World');```"),
        vec![CodeBlock {
            language: Some("js"),
            content: "alert('Hello World');"
        }]
    );
    assert_eq!(
        parse_markdown_text("```rust let c = a + b;```"),
        vec![CodeBlock {
            language: Some("rust"),
            content: "let c = a + b;"
        }]
    );
}

#[test]
fn code_fence_block_single_line_without_lang() {
    assert_eq!(
        parse_markdown_text("``` alert('Hello World');```"),
        vec![CodeBlock {
            language: None,
            content: "alert('Hello World');"
        }]
    );
    assert_eq!(
        parse_markdown_text("``` let c = a + b;```"),
        vec![CodeBlock {
            language: None,
            content: "let c = a + b;"
        }]
    );

    // no space should fail
    let input = "```alert('Hello World');```";
    assert_ne!(
        parse_markdown_text(input),
        vec![CodeBlock {
            language: Some("alert"),
            content: "('Hello World');"
        }]
    );
    assert_eq!(
        parse_markdown_text(input),
        vec![
            Text("``"),
            InlineCode {
                content: "alert('Hello World');"
            },
            Text("``")
        ]
    );
}

#[test]
fn code_fence_block_multi_line_with_lang() {
    assert_eq!(
        parse_markdown_text("```js\nalert('Hello World');\n```"),
        vec![CodeBlock {
            language: Some("js"),
            content: "alert('Hello World');"
        }]
    );
    assert_eq!(
        parse_markdown_text("```rust\nlet c = a + b;\n```"),
        vec![CodeBlock {
            language: Some("rust"),
            content: "let c = a + b;"
        }]
    );
}

#[test]
fn code_fence_block_one_line_without_lang() {
    assert_eq!(
        parse_markdown_text("```\nalert('Hello World');\n```"),
        vec![CodeBlock {
            language: None,
            content: "alert('Hello World');"
        }]
    );
    assert_eq!(
        parse_markdown_text("```\nlet c = a + b;\n```"),
        vec![CodeBlock {
            language: None,
            content: "let c = a + b;"
        }]
    );
}

#[test]
fn code_fence_block_remove_whitespaces_at_the_end() {
    assert_eq!(
        parse_markdown_text("```\nhello world  \t   \n    \n```"),
        vec![CodeBlock {
            language: None,
            content: "hello world"
        }]
    );
    assert_eq!(
        parse_markdown_text("```js\nalert(\"hi\")  \t   \n    \n```"),
        vec![CodeBlock {
            language: Some("js"),
            content: "alert(\"hi\")"
        }]
    );
}

#[test]
fn code_fence_block_with_emoji() {
    assert_eq!(
        parse_markdown_text("``` 🤔123```"),
        vec![CodeBlock {
            language: None,
            content: "🤔123"
        }]
    );
    assert_eq!(
        parse_markdown_text("```\n🤔100123456\n\n```"),
        vec![CodeBlock {
            language: None,
            content: "🤔100123456"
        }]
    );
    assert_eq!(
        parse_markdown_text("```\n🤔Result🤔\n100123456```"),
        vec![CodeBlock {
            language: None,
            content: "🤔Result🤔\n100123456"
        }]
    );
    assert_eq!(
        parse_markdown_text("```\n🤔Result🤔\n000123456\n```"),
        vec![CodeBlock {
            language: None,
            content: "🤔Result🤔\n000123456"
        }]
    );
    assert_eq!(
        parse_markdown_text(
            "```\n🤔Result🤔\nCODE:       33 | CLASSES:       35 | SCSS:      119\n\n\n```"
        ),
        vec![CodeBlock {
            language: None,
            content: "🤔Result🤔\nCODE:       33 | CLASSES:       35 | SCSS:      119"
        }]
    );
}

#[test]
fn code_fence_block_multi_line_with_extra_spaces() {
    let input = "```js\t  \nalert('Hello World');\n```";
    assert_eq!(
        parse_markdown_text(input),
        vec![CodeBlock {
            language: Some("js"),
            content: "alert('Hello World');"
        }]
    );
    assert_eq!(
        parse_markdown_text("```    \nalert('Hello World');\n   ```"),
        vec![CodeBlock {
            language: None,
            content: "alert('Hello World');"
        }]
    );
    assert_eq!(
        parse_markdown_text("```\t \nlet c = a + b;\n\t  \t```"),
        vec![CodeBlock {
            language: None,
            content: "let c = a + b;"
        }]
    );
}

#[test]
fn code_fence_block_embedded_into_text_example() {
    let input =
            "In javascript there is the `document.getElementById(id)` function for this usecase.\
            \nHere is an **example** that shows how you can use it: ```html <div id=\"my-element\">``` \
            ```js\nlet myElement = document.getElementById(\"my-element\");\
            \nmyElement.onclick = (ev) => console.log(ev);```\nI hope this can help you.";
    assert_eq!(
            parse_markdown_text(input),
            vec![
                Text("In javascript there is the "),
                InlineCode { content: "document.getElementById(id)" },
                Text(" function for this usecase."),
                Linebreak,
                Text("Here is an "),
                Bold(vec![Text("example")]),
                Text(" that shows how you can use it: "),
                CodeBlock {
                    language: Some("html"),
                    content: "<div id=\"my-element\">"
                },
                Text(" "),
                CodeBlock {
                    language: Some("js"),
                    content: "let myElement = document.getElementById(\"my-element\");\nmyElement.onclick = (ev) => console.log(ev);"
                },
                Linebreak,
                Text("I hope this can help you.")
            ]
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
        assert_eq!(parse_markdown_text(input), vec![EmailAddress(input)]);
    }
}

#[test]
fn email_address_example() {
    assert_eq!(
        parse_markdown_text(
            "This is an email address: message.parser@example.com\nMessage me there"
        ),
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
            https_link_no_puny("http://delta.chat", "delta.chat"),
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
            https_link_no_puny("https://delta.chat/en/help?hi=5&e=4#section2.0", "delta.chat"),
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
            mailto_link_no_puny("mailto:delta@example.com", "example.com"),
        ),
        (
            "mailto:delta@example.com?subject=hi&body=hello%20world",
            mailto_link_no_puny("mailto:delta@example.com?subject=hi&body=hello%20world", "example.com"),
        ),
    ];

    let test_cases_with_puny = [
        (
            "mailto:foö@ü.chat",
            mailto_link_no_puny("mailto:foö@ü.chat", "ü.chat"),
        ),
        (
            "https://ü.app#help",
            https_link_no_puny("https://ü.app#help", "ü.app")
        )
    ];


    for (input, destination) in &test_cases_no_puny {
        println!("testing {}", input);
        assert_eq!(
            parse_markdown_text(input),
            vec![Link {
                destination: *destination
            }]
        );
    }

    for (input, destination) in &test_cases_with_puny {
        println!("testing <{}>", input);
        let result = parse_markdown_text(input)[0].destination;
        assert_eq!(
            result.target,
            destination.target
        );
        assert_eq!(
            result.scheme,
            destination.scheme
        );
        assert_eq!(
            result.hostname,
            destination.hostname,
        );
        assert_eq!(
            result.punycode.is_some(),
            true
        );
    }
}

#[test]
fn test_link_example() {
    assert_eq!(
        parse_markdown_text(
            "This is an my site: https://delta.chat/en/help?hi=5&e=4#section2.0\nVisit me there"
        ),
        vec![
            Text("This is an my site: "),
            http_link_no_puny(
                "https://delta.chat/en/help?hi=5&e=4#section2.0",
                "delta.chat"
            ),
            Linebreak,
            Text("Visit me there")
        ]
    );
}

#[test]
fn delimited_email_example() {
    assert_eq!(
        parse_markdown_text("This is an my site: <hello@delta.chat>\nMessage me there"),
        vec![
            Text("This is an my site: "),
            EmailAddress("hello@delta.chat"),
            Linebreak,
            Text("Message me there")
        ]
    );
}

#[test]
fn test_delimited_link_example() {
    assert_eq!(
        parse_markdown_text(
            "This is an my site: <https://delta.chat/en/help?hi=5&e=4#section2.0>\nVisit me there"
        ),
        vec![
            Text("This is an my site: "),
            https_link_no_puny(
                "https://delta.chat/en/help?hi=5&e=4#section2.0",
                "delta.chat"
            ),
            Linebreak,
            Text("Visit me there")
        ]
    );
}

#[test]
fn labeled_link() {
    assert_eq!(
        parse_markdown_text("[a link](https://delta.chat/en/help?hi=5&e=4#section2.0)"),
        vec![LabeledLink {
            label: vec![Text("a link")],
            destination: https_link_no_puny(
                "https://delta.chat/en/help?hi=5&e=4#section2.0",
                "delta.chat"
            ),
        }]
    );
    assert_eq!(
        parse_markdown_text(
            "[rich content **bold**](https://delta.chat/en/help?hi=5&e=4#section2.0)"
        ),
        vec![LabeledLink {
            label: vec![Text("rich content "), Bold(vec![Text("bold")])],
            destination: https_link_no_puny(
                "https://delta.chat/en/help?hi=5&e=4#section2.0",
                "delta.chat"
            ),
        }]
    );
}

#[test]
fn labeled_link_example() {
    assert_eq!(
        parse_markdown_text("you can find the details [here](https://delta.chat/en/help)."),
        vec![
            Text("you can find the details "),
            LabeledLink {
                label: vec![Text("here")],
                destination: https_link_no_puny(
                    "https://delta.chat/en/help?hi=5&e=4#section2.0",
                    "delta.chat"
                ),
            },
            Text(".")
        ]
    );
}

#[test]
fn labeled_link_can_have_comma_or_dot_at_end() {
    assert_eq!(
        parse_markdown_text("you can find the details [here](https://delta.chat/en/help.)."),
        vec![
            Text("you can find the details "),
            LabeledLink {
                label: vec![Text("here")],
                destination: https_link_no_puny(
                    "https://delta.chat/en/help.",
                    "delta.chat"
                ),
            },
            Text(".")
        ]
    );
    assert_eq!(
        parse_markdown_text("you can find the details [here](https://delta.chat/en/help,)."),
        vec![
            Text("you can find the details "),
            LabeledLink {
                label: vec![Text("here")],
                destination: https_link_no_puny(
                    "https://delta.chat/en/help,",
                    "delta.chat"
                ),
            },
            Text(".")
        ]
    );
    assert_eq!(
        parse_markdown_text("you can find the details [here](https://delta.chat/en/help:)."),
        vec![
            Text("you can find the details "),
            LabeledLink {
                label: vec![Text("here")],
                destination: https_link_no_puny(
                    "https://delta.chat/en/help:",
                    "delta.chat"
                ),
            },
            Text(".")
        ]
    );
    assert_eq!(
        parse_markdown_text("you can find the details [here](https://delta.chat/en/help;)."),
        vec![
            Text("you can find the details "),
            LabeledLink {
                label: vec![Text("here")],
                destination: https_link_no_puny(
                    "https://delta.chat/en/help;",
                    "delta.chat"
                ),
            },
            Text(".")
        ]
    );
}
