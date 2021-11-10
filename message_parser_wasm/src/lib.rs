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
        true => dc_message_parser::parser::parse_markdown_text(s),
        false => dc_message_parser::parser::parse_only_text(s),
    };
    JsValue::from_serde(&ast).expect("Element converts to JsValue")
}

/// parses text to json AST (text elements and labled links, to replicate current desktop implementation)
#[wasm_bindgen]
pub fn parse_desktop_set(s: &str) -> JsValue {
    JsValue::from_serde(&dc_message_parser::parser::parse_desktop_set(s))
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
