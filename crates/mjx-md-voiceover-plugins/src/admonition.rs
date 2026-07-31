//! Callout / Admonition alert plugin for `mjx-md-voiceover`.
//!
//! Intercepts blockquote callouts (`> [!NOTE]`, `> [!WARNING]`, etc.)
//! and prefixes spoken text with auditory alert cues for voice agents.

use mjx_md_voiceover_core::{SpeechToken, TransformContext, VoiceAstNode, VoicePlugin};

/// Plugin converting GitHub-style markdown admonitions into auditory alerts.
#[derive(Debug, Default, Clone, Copy)]
pub struct AdmonitionPlugin;

impl AdmonitionPlugin {
    /// Creates a new `AdmonitionPlugin` instance.
    pub fn new() -> Self {
        Self
    }

    fn extract_first_text<'a>(nodes: &'a [VoiceAstNode<'a>]) -> Option<&'a str> {
        for node in nodes {
            match node {
                VoiceAstNode::Text { text } => return Some(text),
                VoiceAstNode::Paragraph { children } => {
                    if let Some(t) = Self::extract_first_text(children) {
                        return Some(t);
                    }
                }
                _ => {}
            }
        }
        None
    }
}

impl VoicePlugin for AdmonitionPlugin {
    fn name(&self) -> &'static str {
        "AdmonitionPlugin"
    }

    fn supports_node<'a>(&self, node: &VoiceAstNode<'a>) -> bool {
        if let VoiceAstNode::BlockQuote { children } = node {
            if let Some(text) = Self::extract_first_text(children) {
                let trimmed = text.trim_start();
                return trimmed.starts_with("[!NOTE]")
                    || trimmed.starts_with("[!WARNING]")
                    || trimmed.starts_with("[!IMPORTANT]")
                    || trimmed.starts_with("[!TIP]")
                    || trimmed.starts_with("[!CAUTION]");
            }
        }
        false
    }

    fn transform<'a>(
        &self,
        node: &VoiceAstNode<'a>,
        _context: &mut TransformContext,
    ) -> Option<SpeechToken<'a>> {
        if let VoiceAstNode::BlockQuote { children } = node {
            if let Some(text) = Self::extract_first_text(children) {
                let trimmed = text.trim_start();
                let prefix: &'static str = if trimmed.starts_with("[!NOTE]") {
                    "Note: "
                } else if trimmed.starts_with("[!WARNING]") {
                    "Warning: "
                } else if trimmed.starts_with("[!IMPORTANT]") {
                    "Important: "
                } else if trimmed.starts_with("[!TIP]") {
                    "Tip: "
                } else if trimmed.starts_with("[!CAUTION]") {
                    "Caution: "
                } else {
                    "Alert: "
                };

                let spoken: &'a str = Box::leak(format!("{}{}", prefix, text).into_boxed_str());
                return Some(SpeechToken::CustomSpeech(spoken));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admonition_plugin() {
        let plugin = AdmonitionPlugin::new();
        let node = VoiceAstNode::BlockQuote {
            children: vec![VoiceAstNode::Paragraph {
                children: vec![VoiceAstNode::Text {
                    text: "[!NOTE]\nThis is a critical update.",
                }],
            }],
        };

        assert!(plugin.supports_node(&node));

        let mut ctx = TransformContext::default();
        let token = plugin.transform(&node, &mut ctx);

        if let Some(SpeechToken::CustomSpeech(speech)) = token {
            assert!(speech.starts_with("Note: "));
            assert!(speech.contains("This is a critical update."));
        } else {
            panic!("Expected CustomSpeech token");
        }
    }
}
