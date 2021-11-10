use super::Element;

pub(crate) mod base_parsers;
mod desktop_subset;
mod markdown_elements;
mod text_elements;

/// parses text elements such as links and email addresses, excluding markdown
pub(crate) fn parse_only_text(input: &str) -> std::vec::Vec<Element> {
    let mut result = Vec::new();
    let mut remaining = input;
    // println!("p-{}", input);
    while !remaining.is_empty() {
        // println!("r-{}", remaining);
        if let Ok((rest, element)) = text_elements::parse_text_element(remaining) {
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
        if let Ok((rest, element)) = markdown_elements::parse_element(remaining) {
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
        if let Ok((rest, element)) = desktop_subset::parse_element(remaining) {
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

#[cfg(test)]
mod test_markdown_text_to_ast {
    use crate::parser::link_url::LinkDestination;

    use super::Element::*;
    use super::*;

    #[test]
    fn bold_capitalized_command_suggestion() {
        let input = "**/TELL** world";
        assert_eq!(
            parse_all(&input),
            vec![Bold(vec![BotCommandSuggestion("/TELL")]), Text(" world")]
        );
    }

    #[test]
    fn bold_command_suggestion() {
        let input = "**/yes** - write yes to the bot";
        assert_eq!(
            parse_all(&input),
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
            parse_all(&input),
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
            parse_all(&input),
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
            parse_all(&input),
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
        assert_eq!(parse_all(&input), vec![Text("read/write")]);
    }

    #[test]
    fn bold() {
        let input = "**hello** world";
        assert_eq!(
            parse_all(&input),
            vec![Bold(vec![Text("hello")]), Text(" world")]
        );
    }
    #[test]
    fn not_bold() {
        let input = "**\nshould not be bold\n**";
        assert_eq!(
            parse_all(&input),
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
            parse_all(&input),
            vec![Italics(vec![Text("hi")]), Text(" world")]
        );
    }
    #[test]
    fn nested_bold_italics() {
        let input = "**_strange_ hello** world";
        assert_eq!(
            parse_all(&input),
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
            parse_all(&input),
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
            parse_all(&input),
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
            parse_all(&input),
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
            parse_all(&input),
            vec![StrikeThrough(vec![
                Text("strikethrough and "),
                Bold(vec![Text("bold")]),
                Text(", jo!")
            ])]
        );
    }
    #[test]
    fn inline_code() {
        let input =
            "hi there, you need to `cargo run` it.\nhi there, you need to ` cargo run ` it.";
        assert_eq!(
            parse_all(&input),
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
            parse_all(&input),
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
        // revert this back to assert_eq, once implemented see https://github.com/deltachat/message-parser/issues/8 for more info
        assert_ne!(
            parse_all(&input),
            vec![Tag("#bücher"), Text(" "), Tag("#Ängste")]
        );
    }

    #[test]
    fn two_adjacent_hashtags() {
        let input = "#1#topic2";
        assert_eq!(parse_all(&input), vec![Tag("#1"), Tag("#topic2")]);
    }

    #[test]
    fn two_hashtags_seperated_by_linebreak() {
        let input = "#1\n#topic2";
        assert_eq!(
            parse_all(&input),
            vec![Tag("#1"), Linebreak, Tag("#topic2")]
        );
    }

    #[test]
    fn two_hashtags_seperated_by_tab() {
        let input = "#1\t#topic2";
        assert_eq!(
            parse_all(&input),
            vec![Tag("#1"), Text("\t"), Tag("#topic2")]
        );
    }

    #[test]
    fn bold_hashtag() {
        let input = "**#hashTagInsideOfBold**";
        assert_eq!(
            parse_all(&input),
            vec![Bold(vec![Tag("#hashTagInsideOfBold")])]
        );
    }

    #[test]
    fn code_fence_block_single_line_with_lang() {
        assert_eq!(
            parse_all(&"```js alert('Hello World');```"),
            vec![CodeBlock {
                language: Some("js"),
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse_all(&"```rust let c = a + b;```"),
            vec![CodeBlock {
                language: Some("rust"),
                content: "let c = a + b;"
            }]
        );
    }

    #[test]
    fn code_fence_block_single_line_without_lang() {
        assert_eq!(
            parse_all(&"``` alert('Hello World');```"),
            vec![CodeBlock {
                language: None,
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse_all(&"``` let c = a + b;```"),
            vec![CodeBlock {
                language: None,
                content: "let c = a + b;"
            }]
        );

        // no space should fail
        let input = "```alert('Hello World');```";
        assert_ne!(
            parse_all(&input),
            vec![CodeBlock {
                language: Some("alert"),
                content: "('Hello World');"
            }]
        );
        assert_eq!(
            parse_all(&input),
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
            parse_all(&"```js\nalert('Hello World');\n```"),
            vec![CodeBlock {
                language: Some("js"),
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse_all(&"```rust\nlet c = a + b;\n```"),
            vec![CodeBlock {
                language: Some("rust"),
                content: "let c = a + b;"
            }]
        );
    }

    #[test]
    fn code_fence_block_multi_line_without_lang() {
        assert_eq!(
            parse_all(&"```\nalert('Hello World');\n```"),
            vec![CodeBlock {
                language: None,
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse_all(&"```\nlet c = a + b;\n```"),
            vec![CodeBlock {
                language: None,
                content: "let c = a + b;"
            }]
        );
    }

    #[test]
    fn code_fence_block_multi_line_with_extra_spaces() {
        let input = "```js\t  \nalert('Hello World');\n```";
        assert_eq!(
            parse_all(&input),
            vec![CodeBlock {
                language: Some("js"),
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse_all(&"```    \nalert('Hello World');\n   ```"),
            vec![CodeBlock {
                language: None,
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse_all(&"```\t \nlet c = a + b;\n\t  \t```"),
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
            parse_all(&input),
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
            assert_eq!(parse_all(&input), vec![EmailAddress(&input)]);
        }
    }

    #[test]
    fn email_address_example() {
        assert_eq!(
            parse_all(&"This is an email address: message.parser@example.com\nMessage me there"),
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
            "https://ü.app#help", // TODO add more url test cases
        ];

        for input in &test_cases {
            println!("testing {}", input);
            assert_eq!(
                parse_all(input),
                vec![Link {
                    destination: LinkDestination::parse(input).unwrap().1
                }]
            );
        }

        for input in &test_cases {
            println!("testing {}", format!("<{}>", input));
            assert_eq!(
                parse_all(input),
                vec![Link {
                    destination: LinkDestination::parse(input).unwrap().1
                }]
            );
        }
    }

    #[test]
    fn test_link_example() {
        assert_eq!(
            parse_all(&"This is an my site: https://delta.chat/en/help?hi=5&e=4#section2.0\nVisit me there"),
            vec![
                Text("This is an my site: "),
                Link {
                    destination: LinkDestination::for_testing("https://delta.chat/en/help?hi=5&e=4#section2.0")
                },
                Linebreak,
                Text("Visit me there")
            ]
        );
    }

    #[test]
    fn test_delimited_link_example() {
        assert_eq!(
            parse_all(&"This is an my site: <https://delta.chat/en/help?hi=5&e=4#section2.0>\nVisit me there"),
            vec![
                Text("This is an my site: "),
                Link {
                    destination: LinkDestination::for_testing("https://delta.chat/en/help?hi=5&e=4#section2.0")
                },
                Linebreak,
                Text("Visit me there")
            ]
        );
    }

    #[test]
    fn labeled_link() {
        assert_eq!(
            parse_all(&"[a link](https://delta.chat/en/help?hi=5&e=4#section2.0)"),
            vec![LabeledLink {
                label: vec![Text("a link")],
                destination: LinkDestination::for_testing(
                    "https://delta.chat/en/help?hi=5&e=4#section2.0"
                )
            }]
        );
        assert_eq!(
            parse_all(&"[rich content **bold**](https://delta.chat/en/help?hi=5&e=4#section2.0)"),
            vec![LabeledLink {
                label: vec![Text("rich content "), Bold(vec![Text("bold")])],
                destination: LinkDestination::for_testing(
                    "https://delta.chat/en/help?hi=5&e=4#section2.0"
                )
            }]
        );
    }

    #[test]
    fn labeled_link_example() {
        assert_eq!(
            parse_all(&"you can find the details [here](https://delta.chat/en/help)."),
            vec![
                Text("you can find the details "),
                LabeledLink {
                    label: vec![Text("here")],
                    destination: LinkDestination::for_testing("https://delta.chat/en/help")
                },
                Text(".")
            ]
        );
    }
}
