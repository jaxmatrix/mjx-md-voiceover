//! # `mjx-md-voiceover-wasm`
//!
//! WebAssembly interface layer for `mjx-md-voiceover`.
//! Targets `wasm32-unknown-unknown` for Web browsers, Node.js, Cloudflare Workers, and Deno.

use mjx_md_voiceover_core::{PluginRegistry, SpeechFormatter, VoiceAstParser};
use mjx_md_voiceover_plugins::{AdmonitionPlugin, CodeBlockPlugin, LatexMathPlugin, MermaidPlugin};
use wasm_bindgen::prelude::*;

/// Builds the plugin set the browser bindings run with.
///
/// Order matters: `PluginRegistry` dispatch is first-match-wins, and
/// `CodeBlockPlugin` claims *every* fenced block — including `mermaid` ones — so
/// registering it first would silently mask `MermaidPlugin` entirely.
///
/// `TablePlugin` is deliberately absent. It matches any text run containing both
/// a pipe and a hyphen and always emits a fixed "Structured data table.", which
/// would swallow ordinary prose; GFM tables also do not yet form frames in the
/// parser, so it cannot produce a correct readout regardless.
fn registry() -> PluginRegistry {
    let mut registry = PluginRegistry::new();
    registry.register(MermaidPlugin::new());
    registry.register(CodeBlockPlugin::new());
    registry.register(LatexMathPlugin::new());
    registry.register(AdmonitionPlugin::new());
    registry
}

/// Converts Markdown syntax string into continuous, natural speech text for TTS synthesis.
///
/// # Errors
/// Returns a `JsValue` error if Markdown parsing fails.
#[wasm_bindgen]
pub fn convert_markdown_to_voiceover(markdown: &str) -> Result<String, JsValue> {
    let ast = VoiceAstParser::parse(markdown).map_err(|err| JsValue::from_str(&err.to_string()))?;
    Ok(SpeechFormatter::format_with_registry(&ast, &registry()))
}

/// Converts Markdown using the bare CommonMark rules, with no plugins registered.
///
/// Exposed so a caller can show what the plugin layer is actually contributing.
///
/// # Errors
/// Returns a `JsValue` error if Markdown parsing fails.
#[wasm_bindgen]
pub fn convert_markdown_core_only(markdown: &str) -> Result<String, JsValue> {
    mjx_md_voiceover_core::parse_and_format(markdown)
        .map_err(|err| JsValue::from_str(&err.to_string()))
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

    /// The registry has to reach fenced blocks and callouts, not just name them.
    #[test]
    fn test_wasm_runs_plugins() {
        let md = "```rust\nfn main() {}\n```";
        assert_eq!(
            convert_markdown_to_voiceover(md).unwrap(),
            "Code snippet in Rust."
        );

        let md = "> [!NOTE]\n> Backups run every six hours.";
        assert_eq!(
            convert_markdown_to_voiceover(md).unwrap(),
            "Note callout. Backups run every six hours."
        );
    }

    /// Mermaid must win over the catch-all code-block plugin.
    #[test]
    fn test_wasm_mermaid_not_masked() {
        let md = "```mermaid\ngraph TD;\nA-->B;\n```";
        assert_eq!(
            convert_markdown_to_voiceover(md).unwrap(),
            "Architecture flowchart diagram."
        );
    }

    #[test]
    fn test_wasm_core_only_skips_plugins() {
        let md = "```rust\nfn main() {}\n```";
        assert_eq!(
            convert_markdown_core_only(md).unwrap(),
            "Code snippet in rust."
        );
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
