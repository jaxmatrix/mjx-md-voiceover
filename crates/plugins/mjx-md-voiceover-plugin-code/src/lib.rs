//! Code block speech verbalizer plugin for `mjx-md-voiceover`.
//!
//! Converts fenced code blocks (` ```rust `, ` ```python `, etc.) into spoken code announcements.

use mjx_md_voiceover_core::{SpeechToken, TransformContext, VoiceAstNode, VoicePlugin};

/// Plugin transforming fenced code blocks into conversational code descriptions for TTS.
#[derive(Debug, Default, Clone, Copy)]
pub struct CodeBlockPlugin;

impl CodeBlockPlugin {
    /// Creates a new `CodeBlockPlugin` instance.
    pub fn new() -> Self {
        Self
    }
}

impl VoicePlugin for CodeBlockPlugin {
    fn name(&self) -> &'static str {
        "CodeBlockPlugin"
    }

    fn supports_node<'a>(&self, node: &VoiceAstNode<'a>) -> bool {
        matches!(node, VoiceAstNode::CodeBlock { .. })
    }

    fn transform<'a>(
        &self,
        node: &VoiceAstNode<'a>,
        _context: &mut TransformContext,
    ) -> Option<SpeechToken<'a>> {
        if let VoiceAstNode::CodeBlock { language, .. } = node {
            let summary = match language {
                Some(lang) if !lang.trim().is_empty() => {
                    let l = lang.trim().to_lowercase();
                    match l.as_str() {
                        "rust" | "rs" => "Code snippet in Rust.",
                        "python" | "py" => "Code snippet in Python.",
                        "javascript" | "js" => "Code snippet in JavaScript.",
                        "typescript" | "ts" => "Code snippet in TypeScript.",
                        "sql" => "SQL Schema Definition.",
                        "bash" | "sh" | "shell" => "Shell command script snippet.",
                        "cpp" | "c++" | "c" => "Code snippet in C++.",
                        "go" | "golang" => "Code snippet in Go.",
                        "java" => "Code snippet in Java.",
                        _ => "Fenced code block snippet.",
                    }
                }
                _ => "Fenced code block snippet.",
            };

            Some(SpeechToken::VerbalizedCode {
                language: *language,
                summary,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_block_plugin_transformation() {
        let plugin = CodeBlockPlugin::new();
        let node = VoiceAstNode::CodeBlock {
            language: Some("rust"),
            code: "fn main() {}",
        };

        assert!(plugin.supports_node(&node));

        let mut ctx = TransformContext::default();
        let token = plugin.transform(&node, &mut ctx);

        assert_eq!(
            token,
            Some(SpeechToken::VerbalizedCode {
                language: Some("rust"),
                summary: "Code snippet in Rust."
            })
        );
    }
}
