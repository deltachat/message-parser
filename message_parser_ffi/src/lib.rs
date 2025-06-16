use std::ffi::CString;

use ::safer_ffi::prelude::*;
use deltachat_message_parser::parser::{Element, LinkDestination};
use quick_xml::escape::escape;
use safer_ffi::char_p::{char_p_boxed, char_p_ref};

// IDEAS
// - returning json representation
// - IDEA return in C types?
//   -> though complex enums (sum types) are not supported in safer_ffi yet:
//      https://getditto.github.io/safer_ffi/derive-reprc/enum.html#more-complex-enums
// - expose other functions
// - emoji functions

/// Modes of the parser, which element set to parse
///
/// see https://github.com/deltachat/message-parser/blob/main/spec.md for details
#[derive_ReprC] // <- `::safer_ffi`'s attribute
#[repr(u8)] // <- explicit integer `repr` is mandatory!
pub enum ParsingMode {
    /// Email addresses, Links, Bot command suggestions and hashtags
    ///
    /// Basically the text displayed is not changed, just clickable
    Text,
    /// The desktop set includes everythin of the text set and additionally:
    /// - Delimited Email addresses: `<hello@delta.chat>`
    /// - Delimited Links: `<http://example.org>`
    /// - Labeled Links: `[Name](url)`
    Desktop,
    /// Desktop set additionally to a markdown subset like code blocks and bold and italics
    Markdown,
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextResultForQt {
    /// whether to use `Text.RichText` instead of the faster `Text.StyledText`
    /// https://doc.qt.io/archives/qt-5.15/qml-qtquick-text.html#textFormat-prop
    pub advanced: bool,
    /// text field for `QML Type: Text` https://doc.qt.io/archives/qt-5.15/qml-qtquick-text.html#textFormat-prop
    ///
    /// the clickable text is prefixed with a keyword telling you how to handle it:
    /// - `hashtag`: open search
    /// - `link` and `labled-link`: open the link - value contains the link destination object as json (so ui knows wether it contains unicode).
    /// - `email`: offer to start chat with email address
    /// - `bcs`: bot command suggestion, prefill draft with value
    pub html: char_p_boxed,
}

/// Pretty-prints a TextResultForQt using Rust's formatting logic.
#[ffi_export]
pub fn mp_print_text_result_for_qt(result: &TextResultForQt) {
    println!("{:?}", result);
}

/// frees the TextResultForQt
#[ffi_export]
pub fn mp_free_text_result_for_qt(result: TextResultForQt) {
    drop(result);
}

fn encode_link_destination(destination: &LinkDestination) -> String {
    let destination_data = serde_json::to_string(&destination).unwrap_or("serdejson-serialization-error".to_string());
    // into username part as uri encoded
    format!("{}@link", urlencoding::encode(&destination_data))
}

fn element_to_qt_html(input: &Element) -> String {
    match input {
        Element::Text(text) => escape(*text).to_string(),
        Element::Tag(tag) => {
            format!(r#""<a href="hashtag:{}">{}</a>""#, escape(*tag), escape(*tag)).to_string()
        }
        Element::Linebreak => "<br>".to_string(),
        Element::Link { destination } => format!(
            r#""<a href="link:{}">{}</a>""#,
            encode_link_destination(destination),
            escape(destination.target)
        )
        .to_string(),
        Element::EmailAddress(email) => {
            let escaped_email = escape(*email);
            format!(r#""<a href="email:{escaped_email}">{escaped_email}</a>""#).to_string()
        }
        Element::BotCommandSuggestion(bcs) => {
            let escaped_bcs = escape(*bcs);
            format!(r#""<a href="bcs:{escaped_bcs}">{escaped_bcs}</a>""#).to_string()
        }

        Element::Bold(elements) => {
            let element_xml: String = elements.iter().map(|e| element_to_qt_html(e)).collect();
            format!(r#""<b>{element_xml}</b>""#).to_string()
        }
        Element::Italics(elements) => {
            let element_xml: String = elements.iter().map(|e| element_to_qt_html(e)).collect();
            format!(r#""<i>{element_xml}</i>""#).to_string()
        }
        Element::StrikeThrough(elements) => {
            let element_xml: String = elements.iter().map(|e| element_to_qt_html(e)).collect();
            format!(r#""<s>{element_xml}</s>""#).to_string()
        }
        Element::LabeledLink { label, destination } => {
            let label_xml: String = label.iter().map(|e| element_to_qt_html(e)).collect();
            format!(
                r#""<a href="link:{}">{label_xml}</a>""#,
                encode_link_destination(destination)
            )
            .to_string()
        }

        Element::InlineCode { content } => {
            format!(r#""<code>{}</code>""#, escape(*content)).to_string()
        }
        Element::CodeBlock { language, content } => format!(
            r#""<div class="code-block" x-code-block-language="{}"><code>{}</code></div>""#,
            language.map(escape).unwrap_or_default(),
            escape(*content)
        )
        .to_string(),
    }
}

/// Pretty-prints a TextResultForQt using Rust's formatting logic.
#[ffi_export]
pub fn mp_parse_to_text_result_for_qt(text: char_p_ref<'_>, mode: ParsingMode) -> TextResultForQt {
    let input = text.to_str();
    let elements = match mode {
        ParsingMode::Text => deltachat_message_parser::parser::parse_only_text(input),
        ParsingMode::Desktop => deltachat_message_parser::parser::parse_desktop_set(input),
        ParsingMode::Markdown => deltachat_message_parser::parser::parse_markdown_text(input),
    };

    // build xml
    let html: String = elements.iter().map(|e| element_to_qt_html(e)).collect();

    // determine what mode is suffient
    let advanced = html.contains("<code>");

    TextResultForQt {
        html: char_p_boxed::from(
            CString::new(html).unwrap_or(CString::new("message-parser-ffi error").unwrap()),
        ),
        advanced,
    }
}

/// get_first_emoji of text, result needs to be freed with `mp_free_rust_string`
#[ffi_export]
pub fn mp_get_first_emoji(text: char_p_ref<'_>) -> char_p_boxed {
    let input = text.to_str();

    match deltachat_message_parser::parser::is_emoji::get_first_emoji(input) {
        Some(emoji) => char_p_boxed::from(CString::new(emoji).unwrap()),
        None => char_p_boxed::from(CString::new("").unwrap()),
    }
}

/// Count emojis in a message, if there are only emojis.
/// 
/// This is used to display messages with only emojis in a larger font size. 
#[ffi_export]
pub fn mp_count_emojis_if_only_contains_emoji(text: char_p_ref<'_>) -> u32 {
    deltachat_message_parser::parser::is_emoji::count_emojis_if_only_contains_emoji(text.to_str()).unwrap_or(0)
}

/// frees a string managed by rust
#[ffi_export]
pub fn mp_free_rust_string(string: char_p_boxed) {
    drop(string);
}

// The following function is only necessary for the header generation.
#[cfg(feature = "headers")] // c.f. the `Cargo.toml` section
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("message_parser.h")?
        .generate()
}
