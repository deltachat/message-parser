// mod email;
pub mod is_emoji;
pub mod link_url;
pub mod parse_from_text;
pub mod utils;
pub mod unicode_ranges;

pub use crate::parser::link_url::LinkDestination;

/// The representation of Elements for the Abstract Syntax Tree
#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Element<'a> {
    /*
    All elements that are not markdown, but still parsed.
    These elements are parsed from every text, but are not converted to or from html.
    */
    Text(&'a str),
    /// #hashtag
    Tag(&'a str),
    /// Represents a linebreak - \n
    Linebreak,
    Link {
        destination: LinkDestination<'a>,
    },
    EmailAddress(&'a str),
    // Later:
    // Mention {
    //     internal_id: &str
    // },
    /// On click, the command gets prefilled as the draft, so it can be easily send.
    BotCommandSuggestion(&'a str),

    /*
    All markdown elements.
    These elements are converted to html when sent out and converted back to the AST format when displaying the message.
    */
    Bold(Vec<Element<'a>>),
    Italics(Vec<Element<'a>>),
    StrikeThrough(Vec<Element<'a>>),

    LabeledLink {
        label: Vec<Element<'a>>,
        destination: LinkDestination<'a>,
    },
    InlineCode {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
    // Later:
    // CollonEmoji(&'a str),
    // InlineTex(&str),
    // BlockTex(&str),
}

/// parses all kinds of elements, including markdown
pub fn parse_markdown_text(input: &str) -> std::vec::Vec<Element> {
    parse_from_text::parse_all(input)
}

/// parses text elements such as links and email addresses, excluding markdown
pub fn parse_only_text(input: &str) -> std::vec::Vec<Element> {
    parse_from_text::parse_only_text(input)
}

/// parses text and delimited/labled link elements to replicate current desktop elements
pub fn parse_desktop_set(input: &str) -> std::vec::Vec<Element> {
    parse_from_text::parse_desktop_set(input)
}
