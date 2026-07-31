//! Admonition callout box auditory alert cue plugin for `mjx-md-voiceover`.
//!
//! Transforms GitHub blockquote callout boxes (`> [!NOTE]`, `> [!WARNING]`, etc.) into auditory alerts.

use mjx_md_voiceover_core::{SpeechToken, TransformContext, VoiceAstNode, VoicePlugin};

/// Plugin transforming Markdown blockquote callout boxes into auditory alert announcements.
#[derive(Debug, Default, Clone, Copy)]
pub struct AdmonitionPlugin;

impl AdmonitionPlugin {
    /// Creates a new `AdmonitionPlugin` instance.
    pub fn new() -> Self {
        Self
    }

    fn extract_callout_tag(text: &str) -> Option<(&str, &str)> {
        let trimmed = text.trim();
        if trimmed.starts_with("[!") {
            if let Some(end_idx) = trimmed.find(']') {
                let tag = &trimmed[2..end_idx];
                let remainder = trimmed[end_idx + 1..].trim();
                return Some((tag, remainder));
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
            if let Some(VoiceAstNode::Paragraph { children: p_children }) = children.first() {
                if let Some(VoiceAstNode::Text { text }) = p_children.first() {
                    return Self::extract_callout_tag(text).is_some();
                }
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
            if let Some(VoiceAstNode::Paragraph { children: p_children }) = children.first() {
                if let Some(VoiceAstNode::Text { text }) = p_children.first() {
                    if let Some((tag, remainder)) = Self::extract_callout_tag(text) {
                        let announcement = match tag.to_uppercase().as_str() {
                            "NOTE" => "Note callout.",
                            "WARNING" => "Warning alert callout.",
                            "IMPORTANT" => "Important announcement.",
                            "TIP" => "Helpful tip.",
                            "CAUTION" => "Cautionary alert.",
                            _ => "Auditory alert callout.",
                        };

                        let full_speech = if remainder.is_empty() {
                            announcement.to_string()
                        } else {
                            format!("{} {}", announcement, remainder)
                        };

                        let spoken: &'a str = Box::leak(full_speech.into_boxed_str());
                        return Some(SpeechToken::CustomSpeech(spoken));
                    }
                }
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
                    text: "[!NOTE] Database backup created.",
                }],
            }],
        };

        assert!(plugin.supports_node(&node));

        let mut ctx = TransformContext::default();
        let token = plugin.transform(&node, &mut ctx);

        assert_eq!(
            token,
            Some(SpeechToken::CustomSpeech("Note callout. Database backup created."))
        );
    }
}
