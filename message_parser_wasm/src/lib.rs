mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

/// parses text to json AST
#[wasm_bindgen]
pub fn parse_text(s: &str, enable_markdown: bool) -> JsValue {
    let ast = match enable_markdown {
        true => deltachat_message_parser::parser::parse_markdown_text(s),
        false => deltachat_message_parser::parser::parse_only_text(s),
    };
    serde_wasm_bindgen::to_value(&ast).expect("Element converts to JsValue")
}

/// parses text to json AST (text elements and labeled links, to replicate current desktop implementation)
#[wasm_bindgen]
pub fn parse_desktop_set(s: &str) -> JsValue {
    serde_wasm_bindgen::to_value(&deltachat_message_parser::parser::parse_desktop_set(s))
        .expect("Element converts to JsValue")
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type PunycodeWarning = {
  original_hostname: string;
  ascii_hostname: string;
  punycode_encoded_url: string;
};
export type LinkDestination = {
  target: string;
  hostname: null | string;
  punycode: null | PunycodeWarning;
};
export type ParsedElement =
  | { t: "Text"; c: string }
  | { t: "Tag"; c: string }
  | { t: "Linebreak" }
  | { t: "Bold"; c: ParsedElement[] }
  | { t: "Italics"; c: ParsedElement[] }
  | { t: "StrikeThrough"; c: ParsedElement[] }
  | { t: "InlineCode"; c: { content: string } }
  | { t: "CodeBlock"; c: { language: null | string; content: string } }
  | { t: "EmailAddress"; c: string }
  | { t: "BotCommandSuggestion"; c: string }
  | { t: "Link"; c: { destination: LinkDestination } }
  | {
      t: "LabeledLink";
      c: { label: ParsedElement[]; destination: LinkDestination };
    };
"#;

/// returns first emoji from text if text begins with an emoji
#[wasm_bindgen]
pub fn get_first_emoji(input: &str) -> Option<String> {
    deltachat_message_parser::parser::is_emoji::get_first_emoji(input).map(|s| s.to_owned())
}

/// If string contains only emojis count the emojis otherwise retuns null
#[wasm_bindgen]
pub fn count_emojis_if_only_contains_emoji(input: &str) -> Option<u32> {
    deltachat_message_parser::parser::is_emoji::count_emojis_if_only_contains_emoji(input)
}

/// encode a host to punycode encoded string
#[wasm_bindgen]
pub fn punycode_encode_host(host: &str) -> String {
    deltachat_message_parser::parser::punycode_encode_host(host)
}

/// Returns host as decoded unicode string
#[wasm_bindgen]
pub fn punycode_decode_host(host: &str) -> String {
    deltachat_message_parser::parser::punycode_decode_host(host)
}

/// Returns true if host string contains non ASCII characters
#[wasm_bindgen]
pub fn is_puny(host: &str) -> bool {
  deltachat_message_parser::parser::is_puny(host)
}