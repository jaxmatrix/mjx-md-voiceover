//! Code block verbalizer plugin for `mjx-md-voiceover`.
//!
//! Intercepts fenced code blocks (e.g. ````rust fn main() {} ````)
//! and transforms them into natural spoken summaries.

use mjx_md_voiceover_core::{SpeechToken, TransformContext, VoiceAstNode, VoicePlugin};

/// Plugin that verbalizes fenced code blocks into conversational descriptions.
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
        if let VoiceAstNode::CodeBlock { language, code: _ } = node {
            let summary: &'a str = match language {
                Some("rust") => "Code snippet in Rust.",
                Some("python") | Some("py") => "Code snippet in Python.",
                Some("js") | Some("javascript") => "Code snippet in JavaScript.",
                Some("ts") | Some("typescript") => "Code snippet in TypeScript.",
                Some("html") => "HTML code snippet.",
                Some("css") => "CSS stylesheet snippet.",
                Some("json") => "JSON data snippet.",
                Some("sh") | Some("bash") | Some("zsh") => "Shell command script snippet.",
                Some(_) => "Fenced code block snippet.",
                None => "Unspecified code block snippet.",
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
        assert_eq!(plugin.name(), "CodeBlockPlugin");

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
                summary: "Code snippet in Rust.",
            })
        );
    }
}
