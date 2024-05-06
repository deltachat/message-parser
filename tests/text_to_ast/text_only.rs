use super::*;
use deltachat_message_parser::parser::parse_only_text;

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
fn unix_abs_path_is_not_command() {
    let input = "/etc/nginx";
    assert_eq!(parse_only_text(input), vec![Text("/etc/nginx")]);
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
fn test_link_example() {
    assert_eq!(
        parse_only_text(
            "This is an my site: https://delta.chat/en/help?hi=5&e=4#section2.0\nVisit me there"
        ),
        vec![
            Text("This is an my site: "),
            Link {
                destination: https_link_no_puny(
                    "https://delta.chat/en/help?hi=5&e=4#section2.0",
                    "delta.chat",
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
            Text("This is an my email: "),
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
                destination: https_link_no_puny(
                    "https://delta.chat/en/help?hi=5&e=4#section2.0",
                    "delta.chat",
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
            destination: https_link_no_puny(
                "https://delta.chat/en/help?hi=5&e=4#section2.0",
                "delta.chat",
            )
        }]
    );
    assert_ne!(
        parse_only_text("[rich content **bold**](https://delta.chat/en/help?hi=5&e=4#section2.0)"),
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
    assert_ne!(
        parse_only_text("you can find the details [here](https://delta.chat/en/help)."),
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
fn link_do_not_consume_last_comma() {
    assert_eq!(
        parse_only_text("you can find the details on https://delta.chat/en/help,"),
        vec![
            Text("you can find the details on "),
            Link {
                destination: https_link_no_puny("https://delta.chat/en/help", "delta.chat")
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
                destination: https_link_no_puny("https://delta.chat/en/help", "delta.chat")
            },
            Text(";")
        ]
    );
    assert_eq!(
        parse_only_text("you can find the details on https://delta.chat/en/help:"),
        vec![
            Text("you can find the details on "),
            Link {
                destination: https_link_no_puny("https://delta.chat/en/help", "delta.chat")
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
                destination: https_link_no_puny("https://delta.chat/en/help", "delta.chat")
            },
            Text(".")
        ]
    );
    assert_eq!(
        parse_only_text("you can find the details on https://delta.chat/en/help.txt."),
        vec![
            Text("you can find the details on "),
            Link {
                destination: https_link_no_puny("https://delta.chat/en/help.txt", "delta.chat")
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
                destination: https_link_no_puny("https://delta.chat/en/help.html", "delta.chat")
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
                destination: https_link_no_puny("https://en.wikipedia.org/wiki/Bracket_(disambiguation)", "en.wikipedia.org")
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
                destination: https_link_no_puny(
                    "https://github.com/deltachat/message-parser/issues/12",
                    "github.com"
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
                destination: https_link_no_puny("https://en.wikipedia.org/wiki/Bracket_(disambiguation)", "en.wikipedia.org")
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
                destination: https_link_no_puny(
                    "https://en.wikipedia.org/wiki/Bracket_(disambiguation)",
                    "en.wikipedia.org"
                )
            },
            Text("{[}hi]])}")
        ]
    );
}

#[test]
fn link_with_descriptive_parenthesis() {
    assert_eq!(
        parse_only_text("https://delta.chat/page(this is the link to our site)"),
        vec![
            Link {
                destination: https_link_no_puny("https://delta.chat/page", "delta.chat")
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
                destination: https_link_no_puny("https://delta.chat/en/", "delta.chat")
            },
            Text(")")
        ]
    );
}

#[test]
fn bot_suggestion_is_no_email() {
    assert_eq!(
        parse_only_text("/command@bot@addr.com"),
        vec![BotCommandSuggestion("/command@bot@addr.com"),]
    );
    assert_eq!(
        parse_only_text("\n/command@bot@addr.com"),
        vec![Linebreak, BotCommandSuggestion("/command@bot@addr.com"),]
    );

    assert_eq!(
        parse_only_text("Bots that can be selected \n/command@bot@addr.com BOT"),
        vec![
            Text("Bots that can be selected "),
            Linebreak,
            BotCommandSuggestion("/command@bot@addr.com"),
            Text(" BOT"),
        ]
    );
}
