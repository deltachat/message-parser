#![allow(clippy::unwrap_used)]
use deltachat_message_parser::parser::{link_url::PunycodeWarning, LinkDestination};

#[test]
fn basic_parsing() {
    let test_cases_no_puny = vec![
        "http://delta.chat",
        "http://delta.chat:8080",
        "http://localhost",
        "http://127.0.0.0",
        "https://[::1]/",
        "https://[::1]:9000?hi#o",
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
        "ftp://test-test",
        "https://www.openmandriva.org/en/news/article/openmandriva-rome-24-07-released",
        "https://www.openmandriva.org///en/news/article/openmandriva-rome-24-07-released",
    ];

    let test_cases_with_puny = vec!["https://ü.app#help", "http://münchen.de"];

    for input in &test_cases_no_puny {
        let (rest, link_destination) = LinkDestination::parse(input)
            .unwrap_or_else(|_| panic!("Cannot parse link: {}", input));

        assert_eq!(input, &link_destination.target);
        assert_eq!(rest.len(), 0);
        assert!(link_destination.punycode.is_none());
    }

    for input in &test_cases_with_puny {
        let Ok((rest, link_destination)) = LinkDestination::parse(input) else {
            panic!("Parsing {} as link failed", input);
        };

        assert!(link_destination.punycode.is_some());
        assert_eq!(rest.len(), 0);
        assert_eq!(input, &link_destination.target);
    }
}

#[test]
fn bare_scheme_no_parse() {
    // bare scheme shouldn't be linkified
    let bare = vec!["tel", "tel:", "bitcoin:", "mailto", "https://", "http://"];

    for input in bare {
        let result = LinkDestination::parse(input);
        assert!(result.is_err());
    }
}

#[test]
fn invalid_domains() {
    let test_cases = vec![";?:/hi", "##://thing"];

    for input in &test_cases {
        println!("testing {input}");
        assert!(LinkDestination::parse(input).is_err());
    }
}

#[test]
fn punycode_detection() {
    assert_eq!(
        LinkDestination::parse("http://münchen.de").unwrap().1,
        LinkDestination {
            hostname: Some("münchen.de"),
            target: "http://münchen.de",
            scheme: Some("http"),
            punycode: Some(PunycodeWarning {
                original_hostname: "münchen.de".to_owned(),
                ascii_hostname: "xn--mnchen-3ya.de".to_owned(),
                punycode_encoded_url: "http://xn--mnchen-3ya.de".to_owned(),
            }),
        }
    );

    assert_eq!(
        LinkDestination::parse("http://muenchen.de").unwrap().1,
        LinkDestination {
            hostname: Some("muenchen.de"),
            target: "http://muenchen.de",
            scheme:  Some("http"),
            punycode: None,
        }
    );
}

#[test]
fn common_schemes() {
    assert_eq!(
        LinkDestination::parse("http://delta.chat").unwrap(),
        (
            "",
            LinkDestination {
                hostname: Some("delta.chat"),
                target: "http://delta.chat",
                scheme:  Some("http"),
                punycode: None,
            }
        )
    );
    assert_eq!(
        LinkDestination::parse("https://far.chickenkiller.com").unwrap(),
        (
            "",
            LinkDestination {
                hostname: Some("far.chickenkiller.com"),
                target: "https://far.chickenkiller.com",
                scheme:  Some("https"),
                punycode: None,
            }
        )
    );
}
#[test]
fn generic_schemes() {
    assert_eq!(
        LinkDestination::parse("mailto:someone@example.com").unwrap(),
        (
            "",
            LinkDestination {
                hostname: None,
                scheme:  Some("mailto"),
                punycode: None,
                target: "mailto:someone@example.com"
            }
        )
    );
    assert_eq!(
        LinkDestination::parse("bitcoin:bc1qt3xhfvwmdqvxkk089tllvvtzqs8ts06u3u6qka")
            .unwrap()
            .1,
        LinkDestination {
            hostname: None,
            scheme:  Some("bitcoin"),
            target: "bitcoin:bc1qt3xhfvwmdqvxkk089tllvvtzqs8ts06u3u6qka",
            punycode: None,
        }
    );
    assert_eq!(
        LinkDestination::parse("geo:37.786971,-122.399677")
            .unwrap()
            .1,
        LinkDestination {
            scheme:  Some("geo"),
            punycode: None,
            target: "geo:37.786971,-122.399677",
            hostname: None
        }
    );
}

#[test]
fn no_scheme_simple() {
    assert_eq!(
        LinkDestination::parse("example.com").unwrap(),
        (
            "",
            LinkDestination {
                hostname: Some("example.com"),
                scheme: None,
                punycode: None,
                target: "example.com"
            }
        )
    );
}

#[test]
fn no_scheme_with_chat() {
    // exceptional
    assert_eq!(
        LinkDestination::parse("delta.chat").unwrap(),
        (
            "",
            LinkDestination {
                hostname: Some("delta.chat"),
                scheme: None,
                punycode: None,
                target: "delta.chat"
            }
        )
    );
}

#[test]
fn no_scheme_full_iri_segments() {
    // long one with all the path segments
    assert_eq!(
        LinkDestination::parse("delta.chat/path/with/segments?query=params#fragment").unwrap(),
        (
            "",
            LinkDestination {
                hostname: Some("delta.chat"),
                scheme: None,
                punycode: None,
                target: "delta.chat/path/with/segments?query=params#fragment"
            }
        )
    );
}

#[test]
fn no_scheme_punycode() {
    // punycode
    assert_eq!(
        LinkDestination::parse("münchen.com").unwrap(),
        (
            "",
            LinkDestination {
                hostname: Some("münchen.com"),
                scheme: None,
                punycode: Some(PunycodeWarning {
                    original_hostname: "münchen.com".to_owned(),
                    ascii_hostname: "xn--mnchen-3ya.com".to_owned(),
                    punycode_encoded_url: "xn--mnchen-3ya.com".to_owned()
                }),
                target: "münchen.com"
            }
        )
    );
}

#[test]
fn no_scheme_disallow_zip() {
    // Failing case for unsupported TLD
    let result = LinkDestination::parse("free_money.zip");
    assert!(result.is_err());
}

#[test]
fn no_scheme_disallow_authority() {
    // Failing case with user prefix, we dont want this for simple links without scheme
    let result = LinkDestination::parse("user@delta.chat");
    assert!(result.is_err());
}

#[test]
fn no_scheme_disallow_port() {
    // Failing case with port, also not good for simple links without scheme
    let result = LinkDestination::parse("delta.chat:8080/api");
    assert!(result.is_ok());
}
