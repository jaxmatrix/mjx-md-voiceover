#![deny(unsafe_code)]
#![warn(missing_docs)]

//! # `mjx-md-voiceover-plugins`
//!
//! Official plugin ecosystem for `mjx-md-voiceover`.
//! Provides specialized speech transformers for Code fences, LaTeX math formulas, and Admonitions/Callouts.

pub mod admonition;
pub mod code_block;
pub mod latex_math;

pub use admonition::AdmonitionPlugin;
pub use code_block::CodeBlockPlugin;
pub use latex_math::LatexMathPlugin;
pub use mjx_md_voiceover_core::{
    PluginRegistry, SpeechToken, TransformContext, VoiceAstNode, VoicePlugin,
};
