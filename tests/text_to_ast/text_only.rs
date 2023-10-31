use super::*;
use deltachat_message_parser::parser::{parse_only_text, LinkDestination};

#[test]
fn do_not_parse_markdown_elements() {
    assert_eq!(
        parse_only_text("**bold** world"),
        vec![Text("**bold** world")]
    );
    assert_eq!(
        parse_only_text("**_strange_ hello** world"),
        vec![Text("**_strange_ hello** world")]
    );
    assert_eq!(
        parse_only_text("**_strange_ hello** world"),
        vec![Text("**_strange_ hello** world")]
    );
    assert_eq!(
        parse_only_text("~~strikethrough~~ text ~~ notstrikethrough~~ text"),
        vec![Text("~~strikethrough~~ text ~~ notstrikethrough~~ text")]
    );
    assert_eq!(
        parse_only_text("~~strikethrough and **bold**, jo!~~"),
        vec![Text("~~strikethrough and **bold**, jo!~~")]
    );
    assert_eq!(
        parse_only_text(
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
        parse_only_text(input),
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
        parse_only_text(input),
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
            parse_only_text(input),
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
    assert_eq!(parse_only_text(input), vec![Text("read/write")]);
}

#[test]
fn hashtag() {
    let input =
        "#hashtag\nWhen your new here look for #noob\nIf your already an expert look for #expert";
    assert_eq!(
        parse_only_text(input),
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
        parse_only_text(input),
        vec![Tag("#bücher"), Text(" "), Tag("#Ängste")]
    );
}

#[test]
fn two_adjacent_hashtags() {
    let input = "#1#topic2";
    assert_eq!(parse_only_text(input), vec![Tag("#1"), Tag("#topic2")]);
}

#[test]
fn two_hashtags_seperated_by_linebreak() {
    let input = "#1\n#topic2";
    assert_eq!(
        parse_only_text(input),
        vec![Tag("#1"), Linebreak, Tag("#topic2")]
    );
}

#[test]
fn two_hashtags_seperated_by_tab() {
    let input = "#1\t#topic2";
    assert_eq!(
        parse_only_text(input),
        vec![Tag("#1"), Text("\t"), Tag("#topic2")]
    );
}

#[test]
fn persian_hashtags() {
    let input = "راجع به نرم‌افزار #آزاد و #متنباز چی شنیدی؟";
    assert_eq!(
        parse_only_text(input),
        vec![
            Text("راجع به نرم‌افزار "),
            Tag("#آزاد"),
            Text(" و "),
            Tag("#متنباز"),
            Text(" چی شنیدی؟")
        ]
    );
}

#[test]
fn persian_hashtag_with_underline() {
    let input = "میازار موری که دانه‌کش است. #ابوالقاسم_فردوسی";
    assert_eq!(
        parse_only_text(input),
        vec![
            Text("میازار موری که دانه‌کش است. "),
            Tag("#ابوالقاسم_فردوسی")
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
        assert_eq!(parse_only_text(input), vec![EmailAddress(input)]);
    }

    assert_eq!(
        parse_only_text("(mrcow@moo.com)"),
        vec![Text("("), EmailAddress("mrcow@moo.com"), Text(")")]
    );
    assert_eq!(
        parse_only_text("(mr.cow@moo.com"),
        vec![Text("("), EmailAddress("mr.cow@moo.com")]
    );
    assert_eq!(
        parse_only_text("[mr.cow@moo.com]"),
        vec![Text("["), EmailAddress("mr.cow@moo.com"), Text("]")]
    );
    assert_eq!(
        parse_only_text("mr.cow@moo.com}"),
        vec![EmailAddress("mr.cow@moo.com"), Text("}")]
    );
}

#[test]
fn email_address_example() {
    assert_eq!(
        parse_only_text("This is an email address: message.parser@example.com\nMessage me there"),
        vec![
            Text("This is an email address: "),
            EmailAddress("message.parser@example.com"),
            Linebreak,
            Text("Message me there")
        ]
    );
}

#[test]
fn email_address_do_not_parse_last_dot() {
    assert_eq!(
        parse_only_text("you can reach me on me@provider.tld."),
        vec![
            Text("you can reach me on "),
            EmailAddress("me@provider.tld"),
            Text(".")
        ]
    );
}

#[test]
fn email_address_do_not_parse_last_char_if_special() {
    assert_eq!(
        parse_only_text("you can reach me on me@provider.tld!"),
        vec![
            Text("you can reach me on "),
            EmailAddress("me@provider.tld"),
            Text("!")
        ]
    );
    assert_eq!(
        parse_only_text("you can reach me on me@provider.tld?"),
        vec![
            Text("you can reach me on "),
            EmailAddress("me@provider.tld"),
            Text("?")
        ]
    );
    assert_eq!(
        parse_only_text("you can reach me on me@provider.tld,"),
        vec![
            Text("you can reach me on "),
            EmailAddress("me@provider.tld"),
            Text(",")
        ]
    );
    assert_eq!(
        parse_only_text("you can reach me on me@provider.tld:"),
        vec![
            Text("you can reach me on "),
            EmailAddress("me@provider.tld"),
            Text(":")
        ]
    );
    assert_eq!(
        parse_only_text("you can reach me on me@provider.tld;"),
        vec![
            Text("you can reach me on "),
            EmailAddress("me@provider.tld"),
            Text(";")
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
        "https://ü.app#help", // TODO add more urls for testing
    ];

    for input in &test_cases {
        println!("testing {}", input);
        assert_eq!(
            parse_only_text(input),
            vec![Link {
                destination: link_destination_for_testing(input)
            }]
        );
    }

    let input = "http://[2001:0db8:85a3:08d3::0370:7344]:8080/";
    let hostname = "[2001:0db8:85a3:08d3::0370:7344]";
    assert_eq!(
        parse_only_text(input),
        vec![Link {
            destination: LinkDestination {
                target: input,
                hostname: Some(hostname),
                punycode: None,
                scheme: "http"
            }
        }]
    );
}

#[test]
fn link_special_chars_in_location_part() {
    // see "links that have non spec chars in the location part are not clickable #16"
    // https://github.com/deltachat/message-parser/issues/16
    let test_cases = vec![
        // from the issue
        "https://delta.chat/üäö",
        "https://90eghtesadi.com/Content/Detail/2139889/فیلم-مناظره-واکاوی-قرارداد-۲۵-ساله-ایران-و-چین",
        "https://move-it-sportcamps.de/krafttraining-kinder-jugendliche/#:~:text=Sch%C3%A4dliche%20Nebenwirkungen%20beim%20Krafttraining%20f%C3%BCr,Leichtathletik%20als%20bessere%20Alternative%20gelten.",
        "https://int.bahn.de/en/buchung/fahrplan/suche#sts=true&so=Freiburg(Breisgau)%20Hbf&zo=Gl%C3%BCckstadt&kl=2&r=13:16:KLASSENLOS:1&soid=A%3D1%40O%3DFreiburg(Breisgau)%20Hbf%40X%3D7841174%40Y%3D47997696%40U%3D80%40L%3D8000107%40B%3D1%40p%3D1695240090%40&zoid=A%3D1%40O%3DGl%C3%BCckstadt%40X%3D9428318%40Y%3D53788610%40U%3D80%40L%3D8002293%40B%3D1%40p%3D1695240090%40&hd=2023-09-23T08:00:59&hza=D&ar=false&s=true&d=false&hz=%5B%5D&fm=false&bp=false",
        "https://www.erbrechtsinfo.com/allgemeines-erbrecht/pflichtteilsergaenzungsanspruch-niessbrauch/#:~:text=Wird%20ein%20Nießbrauch%20bei%20einer,Schenkenden%20herausfallen%2C%20einen%20Pflichtteilsergänzungsanspruch%20auslösen",
        // from https://developer.mozilla.org/en-US/docs/Web/Text_fragments
        "https://example.com#:~:text=for",
        "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#:~:text=human",
        "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#:~:text=linked%20URL",
        "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#:~:text=human,url",
        "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#:~:text=linked%20URL,defining%20a%20value",
        "https://example.com/#:~:text=asking-,for",
        "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#:~:text=sent-,referrer",
        "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#:~:text=linked%20URL,-'s%20format",
        "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#:~:text=downgrade:-,The%20Referer,be%20sent,-to%20origins",
        "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#:~:text=causes&text=linked",
        "https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#:~:text=linked%20URL,-'s%20format&text=Deprecated-,attributes,attribute",
    ];

    for input in &test_cases {
        println!("testing {}", input);
        assert_eq!(
            parse_only_text(input),
            vec![Link {
                destination: link_destination_for_testing(input)
            }]
        );
    }
}

#[test]
fn link_stop_at_ending_char() {
    let endings = vec![
        "\"",
        "'",
        "''",
        "\n\n",
    ];

    for end_case in &endings {
        let test_case = format!("https://delta.chat/delta{end_case}");
        println!("testing {}", test_case);
        let mut parsed_ending = parse_only_text(end_case);
        let mut result = vec![Link {
            destination: link_destination_for_testing("https://delta.chat/delta")
        }];
        result.append(&mut parsed_ending);
        assert_eq!(
            parse_only_text(&test_case),
            result
        );
    }
}

#[test]
fn link_stop_at_ending_char_with_space() {
    let endings = vec![
        "\" ",
        "' ",
        "'' ",
        "\n\n ",
        " ",
        ": "
    ];

    for end_case in &endings {
        let test_case = format!("https://delta.chat/delta{end_case}");
        println!("testing {}", test_case);
        let mut parsed_ending = parse_only_text(end_case);
        let mut result = vec![Link {
            destination: link_destination_for_testing("https://delta.chat/delta")
        }];
        result.append(&mut parsed_ending);
        assert_eq!(
            parse_only_text(&test_case),
            result
        );
    }
}


#[test]
fn test_link_example() {
    assert_eq!(
        parse_only_text(
            "This is an my site: https://delta.chat/en/help?hi=5&e=4#section2.0\nVisit me there"
        ),
        vec![
            Text("This is an my site: "),
            Link {
                destination: link_destination_for_testing(
                    "https://delta.chat/en/help?hi=5&e=4#section2.0"
                )
            },
            Linebreak,
            Text("Visit me there")
        ]
    );
}

#[test]
fn delimited_email_should_not_work() {
    assert_ne!(
        parse_only_text("This is an my site: <hello@delta.chat>\nMessage me there"),
        vec![
            Text("This is an my site: "),
            EmailAddress("hello@delta.chat"),
            Linebreak,
            Text("Message me there")
        ]
    );
}

#[test]
fn delimited_link_should_not_work() {
    assert_ne!(
        parse_only_text(
            "This is an my site: <https://delta.chat/en/help?hi=5&e=4#section2.0>\nVisit me there"
        ),
        vec![
            Text("This is an my site: "),
            Link {
                destination: link_destination_for_testing(
                    "https://delta.chat/en/help?hi=5&e=4#section2.0"
                )
            },
            Linebreak,
            Text("Visit me there")
        ]
    );
}

#[test]
fn labeled_link_should_not_work() {
    assert_ne!(
        parse_only_text("[a link](https://delta.chat/en/help?hi=5&e=4#section2.0)"),
        vec![LabeledLink {
            label: vec![Text("a link")],
            destination: link_destination_for_testing(
                "https://delta.chat/en/help?hi=5&e=4#section2.0"
            )
        }]
    );
    assert_ne!(
        parse_only_text("[rich content **bold**](https://delta.chat/en/help?hi=5&e=4#section2.0)"),
        vec![LabeledLink {
            label: vec![Text("rich content "), Bold(vec![Text("bold")])],
            destination: link_destination_for_testing(
                "https://delta.chat/en/help?hi=5&e=4#section2.0"
            )
        }]
    );
}

#[test]
fn labeled_link_example_should_not_work() {
    assert_ne!(
        parse_only_text("you can find the details [here](https://delta.chat/en/help)."),
        vec![
            Text("you can find the details "),
            LabeledLink {
                label: vec![Text("here")],
                destination: link_destination_for_testing("https://delta.chat/en/help")
            },
            Text(".")
        ]
    );
}

#[test]
fn link_do_not_consume_last_comma() {
    assert_eq!(
        parse_only_text("you can find the details on https://delta.chat/en/help,"),
        vec![
            Text("you can find the details on "),
            Link {
                destination: link_destination_for_testing("https://delta.chat/en/help")
            },
            Text(",")
        ]
    );
}

#[test]
fn link_do_not_consume_last_semicolon_or_colon() {
    assert_eq!(
        parse_only_text("you can find the details on https://delta.chat/en/help;"),
        vec![
            Text("you can find the details on "),
            Link {
                destination: link_destination_for_testing("https://delta.chat/en/help")
            },
            Text(";")
        ]
    );
    assert_eq!(
        parse_only_text("you can find the details on https://delta.chat/en/help:"),
        vec![
            Text("you can find the details on "),
            Link {
                destination: link_destination_for_testing("https://delta.chat/en/help")
            },
            Text(":")
        ]
    );
}

#[test]
fn link_do_not_consume_last_dot() {
    assert_eq!(
        parse_only_text("you can find the details on https://delta.chat/en/help."),
        vec![
            Text("you can find the details on "),
            Link {
                destination: link_destination_for_testing("https://delta.chat/en/help")
            },
            Text(".")
        ]
    );
    assert_eq!(
        parse_only_text("you can find the details on https://delta.chat/en/help.txt."),
        vec![
            Text("you can find the details on "),
            Link {
                destination: link_destination_for_testing("https://delta.chat/en/help.txt")
            },
            Text(".")
        ]
    );
}

#[test]
fn link_with_file_extention() {
    assert_eq!(
        parse_only_text("you can find the details on https://delta.chat/en/help.html"),
        vec![
            Text("you can find the details on "),
            Link {
                destination: link_destination_for_testing("https://delta.chat/en/help.html")
            }
        ]
    );
}

#[test]
fn parenthesis_in_links() {
    assert_eq!(
        parse_only_text("links can contain parenthesis, https://en.wikipedia.org/wiki/Bracket_(disambiguation) is an example of this."),
        vec![
            Text("links can contain parenthesis, "),
            Link {
                destination: link_destination_for_testing("https://en.wikipedia.org/wiki/Bracket_(disambiguation)")
            },
            Text(" is an example of this.")
        ]
    );
}

#[test]
fn link_in_parenthesis() {
    assert_eq!(
        parse_only_text(
            "for more information see (https://github.com/deltachat/message-parser/issues/12)"
        ),
        vec![
            Text("for more information see ("),
            Link {
                destination: link_destination_for_testing(
                    "https://github.com/deltachat/message-parser/issues/12"
                )
            },
            Text(")")
        ]
    );
}

#[test]
fn link_with_parenthesis_in_parenthesis() {
    assert_eq!(
        parse_only_text("there are links that contain parenthesis (for example https://en.wikipedia.org/wiki/Bracket_(disambiguation))"),
        vec![
            Text("there are links that contain parenthesis (for example "),
            Link {
                destination: link_destination_for_testing("https://en.wikipedia.org/wiki/Bracket_(disambiguation)")
            },
            Text(")")
        ]
    );
}

#[test]
fn link_with_different_parenthesis_in_parenthesis() {
    assert_eq!(
        parse_only_text(
            "()(for [example{ https://en.wikipedia.org/wiki/Bracket_(disambiguation){[}hi]])}"
        ),
        vec![
            Text("()(for [example{ "),
            Link {
                destination: link_destination_for_testing(
                    "https://en.wikipedia.org/wiki/Bracket_(disambiguation){[}hi]"
                )
            },
            Text("])}")
        ]
    );
}

#[test]
fn link_with_backets_in_backets() {
    assert_eq!(
        parse_only_text("there are links that contain backets [for example https://en.wikipedia.org/wiki/Bracket_[disambiguation]]"),
        vec![
            Text("there are links that contain backets [for example "),
            Link {
                destination: link_destination_for_testing("https://en.wikipedia.org/wiki/Bracket_[disambiguation]")
            },
            Text("]")
        ]
    );
}

#[test]
fn link_with_parenthesis_in_parenthesis_curly() {
    assert_eq!(
        parse_only_text("there are links that contain parenthesis {for example https://en.wikipedia.org/wiki/Bracket_{disambiguation}}"),
        vec![
            Text("there are links that contain parenthesis {for example "),
            Link {
                destination: link_destination_for_testing("https://en.wikipedia.org/wiki/Bracket_{disambiguation}")
            },
            Text("}")
        ]
    );
}

#[test]
fn link_with_descriptive_parenthesis() {
    assert_eq!(
        parse_only_text("https://delta.chat/page(this is the link to our site)"),
        vec![
            Link {
                destination: link_destination_for_testing("https://delta.chat/page")
            },
            Text("(this is the link to our site)")
        ]
    );
}

#[test]
fn link_in_parenthesis2() {
    assert_eq!(
        parse_only_text("A great chat app (see https://delta.chat/en/)"),
        vec![
            Text("A great chat app (see "),
            Link {
                destination: link_destination_for_testing("https://delta.chat/en/")
            },
            Text(")")
        ]
    );
}
