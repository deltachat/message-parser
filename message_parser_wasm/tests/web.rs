//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use message_parser_wasm::*;

use serde_json::json;
use wasm_bindgen::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = JSON)]
    fn stringify(s: &JsValue) -> JsValue;
}

#[wasm_bindgen_test]
fn test_parse() {
    assert_eq!(
        parse("**Block**"),
        JsValue::from_serde(&json!([
            {"t":"Bold","c":[{"t":"Text","c":"Block"}]}
        ]))
        .unwrap()
    );

    assert_eq!(
        stringify(&parse("**`Block`**")),
        JsValue::from_str(r#"{"t":"Bold","c":[{"t":"InlineCode","c":{"content":"Block"}}]}"#)
    ); // this test needs somekind of deep equal because the order of the properties is not fixed
}
