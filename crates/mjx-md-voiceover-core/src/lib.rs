#![deny(unsafe_code)]
#![warn(missing_docs)]

//! # `mjx-md-voiceover-core`
//!
//! Core Markdown AST parser and natural speech voiceover generator engine.
//! Designed for sub-millisecond execution (<1-10 ms SLA) and full WebAssembly (`wasm32-unknown-unknown`) safety.

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
