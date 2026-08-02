#![deny(unsafe_code)]
#![warn(missing_docs)]

//! # `mjx-md-voiceover-core`
//!
//! Core Markdown AST parser and natural speech voiceover generator engine.
//! Designed for sub-millisecond execution (<1-10 ms SLA) and full WebAssembly (`wasm32-unknown-unknown`) safety.

pub mod ast;
pub mod formatter;
pub mod parser;
pub mod plugin;

pub use ast::{EmphasisType, PauseDuration, SpeechToken, TransformContext, VoiceAst, VoiceAstNode};
pub use formatter::SpeechFormatter;
pub use parser::VoiceAstParser;
pub use plugin::{PluginRegistry, VoicePlugin};

/// Convenient top-level helper function to parse Markdown into natural speech text.
pub fn parse_and_format(markdown: &str) -> Result<String> {
    let ast = VoiceAstParser::parse(markdown)?;
    Ok(SpeechFormatter::format(&ast))
}

/// Engine error types.
#[derive(Debug, thiserror::Error)]
pub enum VoiceoverError {
    /// Parsing failed due to invalid Markdown syntax input.
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Result alias for core operations.
pub type Result<T> = std::result::Result<T, VoiceoverError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_crate_initialization() {
        let err = VoiceoverError::ParseError("test initialization".into());
        assert_eq!(err.to_string(), "Parse error: test initialization");
    }
}
