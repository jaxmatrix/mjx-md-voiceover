//! # `mjx-md-voiceover-wasm`
//!
//! WebAssembly interface layer for `mjx-md-voiceover`.
//! Targets `wasm32-unknown-unknown` for Web browsers, Node.js, Cloudflare Workers, and Deno.

use mjx_md_voiceover_core::{parse_and_format, VoiceAstParser};
use wasm_bindgen::prelude::*;

/// Converts Markdown syntax string into continuous, natural speech text for TTS synthesis.
///
/// # Errors
/// Returns a `JsValue` error if Markdown parsing fails.
#[wasm_bindgen]
pub fn convert_markdown_to_voiceover(markdown: &str) -> Result<String, JsValue> {
    parse_and_format(markdown).map_err(|err| JsValue::from_str(&err.to_string()))
}

/// Returns the JSON representation of the parsed Markdown Voice AST.
///
/// # Errors
/// Returns a `JsValue` error if parsing or JSON serialization fails.
#[wasm_bindgen]
pub fn parse_markdown_ast_json(markdown: &str) -> Result<String, JsValue> {
    let ast = VoiceAstParser::parse(markdown).map_err(|err| JsValue::from_str(&err.to_string()))?;
    serde_json::to_string(&ast).map_err(|err| JsValue::from_str(&err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_convert_markdown_to_voiceover() {
        let md = "# Title\n\nHello WASM!";
        let res = convert_markdown_to_voiceover(md);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Heading: Title. Hello WASM!");
    }

    #[test]
    fn test_wasm_parse_markdown_ast_json() {
        let md = "# Test";
        let res = parse_markdown_ast_json(md);
        assert!(res.is_ok());
        let json = res.unwrap();
        assert!(json.contains("Heading"));
    }
}
