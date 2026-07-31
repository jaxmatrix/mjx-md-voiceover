//! Mermaid diagram speech verbalizer plugin for `mjx-md-voiceover`.
//!
//! Converts Mermaid diagram blocks (` ```mermaid `) into spoken diagram summaries.

use mjx_md_voiceover_core::{SpeechToken, TransformContext, VoiceAstNode, VoicePlugin};

/// Plugin converting Mermaid diagram blocks into conversational spoken overviews.
#[derive(Debug, Default, Clone, Copy)]
pub struct MermaidPlugin;

impl MermaidPlugin {
    /// Creates a new `MermaidPlugin` instance.
    pub fn new() -> Self {
        Self
    }
}

impl VoicePlugin for MermaidPlugin {
    fn name(&self) -> &'static str {
        "MermaidPlugin"
    }

    fn supports_node<'a>(&self, node: &VoiceAstNode<'a>) -> bool {
        if let VoiceAstNode::CodeBlock { language: Some(lang), .. } = node {
            return lang.trim().eq_ignore_ascii_case("mermaid");
        }
        false
    }

    fn transform<'a>(
        &self,
        node: &VoiceAstNode<'a>,
        _context: &mut TransformContext,
    ) -> Option<SpeechToken<'a>> {
        if let VoiceAstNode::CodeBlock { code, .. } = node {
            let trimmed = code.trim();
            let summary = if trimmed.starts_with("sequenceDiagram") {
                "Sequence flow diagram illustration."
            } else if trimmed.starts_with("classDiagram") {
                "Object class diagram architecture."
            } else if trimmed.starts_with("erDiagram") {
                "Entity relationship schema diagram."
            } else if trimmed.starts_with("gantt") {
                "Project Gantt timeline chart."
            } else {
                "Architecture flowchart diagram."
            };

            Some(SpeechToken::VerbalizedCode {
                language: Some("mermaid"),
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
    fn test_mermaid_plugin_transformation() {
        let plugin = MermaidPlugin::new();
        let node = VoiceAstNode::CodeBlock {
            language: Some("mermaid"),
            code: "graph TD\n  A --> B",
        };

        assert!(plugin.supports_node(&node));

        let mut ctx = TransformContext::default();
        let token = plugin.transform(&node, &mut ctx);

        assert_eq!(
            token,
            Some(SpeechToken::VerbalizedCode {
                language: Some("mermaid"),
                summary: "Architecture flowchart diagram."
            })
        );
    }
}
