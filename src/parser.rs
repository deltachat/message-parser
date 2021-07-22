use email_address_parser::EmailAddress;
use iref::Iri;
use nom::{
    bytes::{
        complete::{is_not, tag, take, take_while, take_while1},
        streaming::take_till1,
    },
    character::{self, complete::alphanumeric1},
    combinator::{opt, peek, recognize},
    error::{ErrorKind, ParseError},
    sequence::delimited,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum CustomError<I> {
    NoContent,
    InvalidWhiteSpaceFound,
    NoElement,
    Nom(I, ErrorKind),
    InvalidEmail,
    InvalidLink,
    UnexpectedContent,
}

impl<I> ParseError<I> for CustomError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        CustomError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Element<'a> {
    Text(&'a str),
    /// #hashtag
    Tag(&'a str),
    /// Represents a linebreak - \n
    Linebreak,
    Bold(Vec<Element<'a>>),
    Italics(Vec<Element<'a>>),
    StrikeThrough(Vec<Element<'a>>),
    Link {
        destination: &'a str,
        // contains_puny_code: bool,
    },
    LabeledLink {
        label: Vec<Element<'a>>,
        destination: &'a str,
        // contains_puny_code: bool,
    },
    InlineCode {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
    // BotCommandSuggestion(&'a str),
    EmailAddress(&'a str),
    // Later:
    // CollonEmoji(&'a str),
    // Mention {
    //     internal_id: &str
    // },
    // InlineTex(&str),
    // BlockTex(&str),
}

/// consumes all text until parse_element works again, internal use text instead
///
/// its output is useable on its own, always combinate this with [nom::combinator::recognize]
fn eat_text<'a>(input: &'a str) -> IResult<&'a str, (), CustomError<&'a str>> {
    let mut remaining = input;
    while remaining.len() > 0 {
        // take 1, because other parsers didn't work (text is always the last used parser)
        remaining = take(1usize)(remaining)?.0;
        // peek if there is an element
        if peek(parse_element)(remaining).is_ok() {
            break;
        }
        // take until whitespace
        //remaining = take_while(|c| not_blank_space(c))(remaining)?.0;
    }
    Ok((remaining, ()))
}

/// Consumes text until another parser of parse_element works again
///
/// used as last parser, if the others do not consume the input it consumes the input until another parser works again
/// (uses whitespace seperation to make the parsing faster)
fn text<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    let (rest, content) = recognize(eat_text)(input)?;
    Ok((rest, Element::Text(content)))
}

fn is_white_space(c: char) -> bool {
    match c {
        '\n' | '\r' | '\t' | ' ' => true,
        _ => false,
    }
}

fn is_not_white_space(c: char) -> bool {
    !is_white_space(c)
}

fn is_white_space_but_not_linebreak(c: char) -> bool {
    match c {
        '\t' | ' ' => true,
        _ => false,
    }
}

/// delimited no whitespace start or end
fn direct_delimited<'a>(
    input: &'a str,
    tag_str: &str,
) -> IResult<&'a str, &'a str, CustomError<&'a str>> {
    let (input, content): (&str, &str) = delimited(
        tag(tag_str),
        nom::bytes::complete::is_not(tag_str),
        tag(tag_str),
    )(input)?;
    if content.len() == 0 {
        return Err(nom::Err::Error(CustomError::NoContent));
    }
    if is_white_space(content.chars().nth(0).unwrap())
        || is_white_space(content.chars().last().unwrap())
    {
        return Err(nom::Err::Error(CustomError::InvalidWhiteSpaceFound));
    }
    Ok((input, content))
}

named!(inline_code<&str, &str>, delimited!(tag!("`"), is_not!("`"), tag!("`")));
named!(linebreak<&str, char>, char!('\n'));

fn hashtag_content_char(c: char) -> bool {
    !(is_white_space(c) || c == '#')
}

fn hashtag<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    let (input, _) = character::complete::char('#')(input)?;
    let (input, content) = take_while1(hashtag_content_char)(input)?;

    Ok((input, Element::Tag(content)))
}

fn code_block<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    let (input, content): (&str, &str) =
        delimited(tag("```"), nom::bytes::complete::is_not("```"), tag("```"))(input)?;

    let (content, lang) = if is_white_space(
        content
            .chars()
            .nth(0)
            .ok_or(nom::Err::Error(CustomError::NoContent))?,
    ) {
        // no language defined
        (content, None)
    } else {
        // language defined
        let (content, lang): (&str, &str) = alphanumeric1(content)?;
        (content, Some(lang))
    };

    // expect whitespace or new line after language or beginning (if no language is defined)
    let char_in_question = content
        .chars()
        .nth(0)
        .ok_or(nom::Err::Error(CustomError::NoContent))?;

    let content = if is_white_space_but_not_linebreak(char_in_question) {
        // remove whitespaces until newline or non whitespaces
        let (content, _) = take_while(is_white_space_but_not_linebreak)(content)?;
        // remove new line if there is one
        let (content, _) = opt(tag("\n"))(content)?;
        content
    } else {
        // remove new line if there is one
        let (content, _) = tag("\n")(content)?;
        content
    };

    // remove spaces and last newline at end
    let mut offset = 0;
    let mut c_iter = content.chars().rev();
    while is_white_space_but_not_linebreak(
        c_iter
            .next()
            .ok_or(nom::Err::Error(CustomError::NoContent))?,
    ) {
        offset = offset + 1
    }

    if content
        .chars()
        .rev()
        .nth(offset)
        .ok_or(nom::Err::Error(CustomError::NoContent))?
        == '\n'
    {
        offset = offset + 1
    }

    Ok((
        input,
        Element::CodeBlock {
            language: lang,
            content: &content[0..content.chars().count() - offset],
        },
    ))
}

fn not_email_address_part_char(c: char) -> bool {
    match c {
        '@' | '\n' | '\r' | '\t' | ' ' => true,
        _ => false,
    }
}

fn email_address_part_char(c: char) -> bool {
    !not_email_address_part_char(c)
}

/// rough recognition of an email, results gets checked by a real email address parser
fn email_intern<'a>(input: &'a str) -> IResult<&'a str, (), CustomError<&'a str>> {
    let (input, _) = take_till1(not_email_address_part_char)(input)?;
    let (input, _) = tag("@")(input)?;
    let (input, _) = take_while1(email_address_part_char)(input)?;
    Ok((input, ()))
}

fn email_address<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    let (input, content) = recognize(email_intern)(input)?;
    // check if result is valid email
    if EmailAddress::is_valid(content, None) {
        Ok((input, Element::EmailAddress(content)))
    } else {
        Err(nom::Err::Error(CustomError::InvalidEmail))
    }
}

fn not_link_part_char(c: char) -> bool {
    match c {
        ':' | '\n' | '\r' | '\t' | ' ' => false,
        _ => true,
    }
}

/// rough recognition of an link, results gets checked by a real link parser
fn link_intern<'a>(input: &'a str) -> IResult<&'a str, (), CustomError<&'a str>> {
    let (input, _) = take_while1(not_link_part_char)(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = take_while1(is_not_white_space)(input)?;
    Ok((input, ()))
}

fn link<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    let (input, content) = recognize(link_intern)(input)?;
    // check if result is valid link
    if Iri::new(content).is_ok() {
        Ok((
            input,
            Element::Link {
                destination: content,
            },
        ))
    } else {
        Err(nom::Err::Error(CustomError::InvalidLink))
    }
}

// <https://link>
fn delimited_link<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    let (input, content): (&str, &str) = delimited(tag("<"), is_not(">"), tag(">"))(input)?;
    if content.len() == 0 {
        return Err(nom::Err::Error(CustomError::NoContent));
    }
    let (rest, link) = link(content)?;
    if rest.len() != 0 {
        return Err(nom::Err::Error(CustomError::UnexpectedContent));
    }
    Ok((input, link))
}

// [labeled](https://link)
fn labeled_link<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    let (input, raw_label): (&str, &str) = delimited(tag("["), is_not("]"), tag("]"))(input)?;
    if raw_label.len() == 0 {
        return Err(nom::Err::Error(CustomError::NoContent));
    }
    let label = parse(raw_label);

    let (input, raw_link): (&str, &str) = delimited(tag("("), is_not(")"), tag(")"))(input)?;
    if raw_link.len() == 0 {
        return Err(nom::Err::Error(CustomError::NoContent));
    }
    // check if result is valid link
    if Iri::new(raw_link).is_ok() {
        Ok((
            input,
            Element::LabeledLink {
                label,
                destination: raw_link,
            },
        ))
    } else {
        Err(nom::Err::Error(CustomError::InvalidLink))
    }
}

pub fn parse_element<'a>(input: &'a str) -> IResult<&'a str, Element<'a>, CustomError<&'a str>> {
    // the order is important
    // generaly more specific parsers that fail/return fast should be in the front
    // But keep in mind that the order can also change how and if the parser works as intended
    if let Ok((i, b)) = direct_delimited(input, "**") {
        Ok((i, Element::Bold(parse(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "__") {
        Ok((i, Element::Bold(parse(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "_") {
        Ok((i, Element::Italics(parse(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "*") {
        Ok((i, Element::Italics(parse(b))))
    } else if let Ok((i, b)) = direct_delimited(input, "~~") {
        Ok((i, Element::StrikeThrough(parse(b))))
    } else if let Ok((i, elm)) = code_block(input) {
        Ok((i, elm))
    } else if let Ok((i, b)) = inline_code(input) {
        Ok((i, Element::InlineCode { content: b }))
    } else if let Ok((i, elm)) = hashtag(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = delimited_link(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = labeled_link(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = email_address(input) {
        Ok((i, elm))
    } else if let Ok((i, elm)) = link(input) {
        Ok((i, elm))
    } else if let Ok((i, _)) = linebreak(input) {
        Ok((i, Element::Linebreak))
    } else {
        Err(nom::Err::Error(CustomError::NoElement))
    }
}

pub fn parse<'a>(input: &'a str) -> std::vec::Vec<Element<'a>> {
    let mut result = Vec::new();
    let mut remaining = input;
    // println!("p-{}", input);
    while remaining.len() > 0 {
        // println!("r-{}", remaining);
        if let Ok((rest, element)) = parse_element(remaining) {
            // println!("e-{:?} - {}", element, remaining);
            remaining = rest;
            result.push(element);
        } else if let Ok((rest, element)) = text(remaining) {
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
mod tests {
    use crate::parser::*;
    use Element::*;

    #[test]
    fn bold() {
        let input = "**hello** world";
        assert_eq!(
            parse(&input),
            vec![Bold(vec![Text("hello")]), Text(" world")]
        );
    }
    #[test]
    fn not_bold() {
        let input = "**\nshould not be bold\n**";
        assert_eq!(
            parse(&input),
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
            parse(&input),
            vec![Italics(vec![Text("hi")]), Text(" world")]
        );
    }
    #[test]
    fn nested_bold_italics() {
        let input = "**_strange_ hello** world";
        assert_eq!(
            parse(&input),
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
            parse(&input),
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
            parse(&input),
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
            parse(&input),
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
            parse(&input),
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
            parse(&input),
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
            parse(&input),
            vec![
                Tag("hashtag"),
                Linebreak,
                Text("When your new here look for "),
                Tag("noob"),
                Linebreak,
                Text("If your already an expert look for "),
                Tag("expert")
            ]
        );
    }

    #[test]
    fn german_umlaut_hashtag() {
        let input = "#bücher #Ängste";
        assert_eq!(parse(&input), vec![Tag("bücher"), Text(" "), Tag("Ängste")]);
    }

    #[test]
    fn two_adjacent_hashtags() {
        let input = "#1#topic2";
        assert_eq!(parse(&input), vec![Tag("1"), Tag("topic2")]);
    }

    #[test]
    fn two_hashtags_seperated_by_linebreak() {
        let input = "#1\n#topic2";
        assert_eq!(parse(&input), vec![Tag("1"), Linebreak, Tag("topic2")]);
    }

    #[test]
    fn two_hashtags_seperated_by_tab() {
        let input = "#1\t#topic2";
        assert_eq!(parse(&input), vec![Tag("1"), Text("\t"), Tag("topic2")]);
    }

    #[test]
    fn bold_hashtag() {
        let input = "**#hashTagInsideOfBold**";
        assert_eq!(parse(&input), vec![Bold(vec![Tag("hashTagInsideOfBold")])]);
    }

    #[test]
    fn code_fence_block_single_line_with_lang() {
        assert_eq!(
            parse(&"```js alert('Hello World');```"),
            vec![CodeBlock {
                language: Some("js"),
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse(&"```rust let c = a + b;```"),
            vec![CodeBlock {
                language: Some("rust"),
                content: "let c = a + b;"
            }]
        );
    }

    #[test]
    fn code_fence_block_single_line_without_lang() {
        assert_eq!(
            parse(&"``` alert('Hello World');```"),
            vec![CodeBlock {
                language: None,
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse(&"``` let c = a + b;```"),
            vec![CodeBlock {
                language: None,
                content: "let c = a + b;"
            }]
        );

        // no space should fail
        let input = "```alert('Hello World');```";
        assert_ne!(
            parse(&input),
            vec![CodeBlock {
                language: Some("alert"),
                content: "('Hello World');"
            }]
        );
        assert_eq!(
            parse(&input),
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
            parse(&"```js\nalert('Hello World');\n```"),
            vec![CodeBlock {
                language: Some("js"),
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse(&"```rust\nlet c = a + b;\n```"),
            vec![CodeBlock {
                language: Some("rust"),
                content: "let c = a + b;"
            }]
        );
    }

    #[test]
    fn code_fence_block_multi_line_without_lang() {
        assert_eq!(
            parse(&"```\nalert('Hello World');\n```"),
            vec![CodeBlock {
                language: None,
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse(&"```\nlet c = a + b;\n```"),
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
            parse(&input),
            vec![CodeBlock {
                language: Some("js"),
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse(&"```    \nalert('Hello World');\n   ```"),
            vec![CodeBlock {
                language: None,
                content: "alert('Hello World');"
            }]
        );
        assert_eq!(
            parse(&"```\t \nlet c = a + b;\n\t  \t```"),
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
            parse(&input),
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
            "617b5772c6d10feda41fc6e0e43b976c4cc9383d3729310d3dc9e1332f0d9acd@yggmail"
            // TODO add email test
        ];

        for input in test_cases {
            println!("testing {}", &input);
            assert_eq!(parse(&input), vec![EmailAddress(&input)]);
        }
    }

    #[test]
    fn email_address_example() {
        assert_eq!(
            parse(&"This is an email address: message.parser@example.com\nMessage me there"),
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
            assert_eq!(parse(input), vec![Link { destination: input }]);
        }

        for input in &test_cases {
            println!("testing {}", format!("<{}>", input));
            assert_eq!(parse(input), vec![Link { destination: input }]);
        }
    }

    #[test]
    fn test_link_example() {
        assert_eq!(
            parse(&"This is an my site: https://delta.chat/en/help?hi=5&e=4#section2.0\nVisit me there"),
            vec![
                Text("This is an my site: "),
                Link {
                    destination: "https://delta.chat/en/help?hi=5&e=4#section2.0"
                },
                Linebreak,
                Text("Visit me there")
            ]
        );
    }

    #[test]
    fn test_delimited_link_example() {
        assert_eq!(
            parse(&"This is an my site: <https://delta.chat/en/help?hi=5&e=4#section2.0>\nVisit me there"),
            vec![
                Text("This is an my site: "),
                Link {
                    destination: "https://delta.chat/en/help?hi=5&e=4#section2.0"
                },
                Linebreak,
                Text("Visit me there")
            ]
        );
    }

    #[test]
    fn labeled_link() {
        assert_eq!(
            parse(&"[a link](https://delta.chat/en/help?hi=5&e=4#section2.0)"),
            vec![LabeledLink {
                label: vec![Text("a link")],
                destination: "https://delta.chat/en/help?hi=5&e=4#section2.0"
            }]
        );
        assert_eq!(
            parse(&"[rich content **bold**](https://delta.chat/en/help?hi=5&e=4#section2.0)"),
            vec![LabeledLink {
                label: vec![Text("rich content "), Bold(vec![Text("bold")])],
                destination: "https://delta.chat/en/help?hi=5&e=4#section2.0"
            }]
        );
    }

    #[test]
    fn labeled_link_example() {
        assert_eq!(
            parse(&"you can find the details [here](https://delta.chat/en/help)."),
            vec![
                Text("you can find the details "),
                LabeledLink {
                    label: vec![Text("here")],
                    destination: "https://delta.chat/en/help"
                },
                Text(".")
            ]
        );
    }
}
