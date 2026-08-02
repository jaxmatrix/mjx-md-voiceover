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

    /// Collects the inline text of a callout body into one string.
    ///
    /// Matching on the first `Text` child alone does not work on real markdown:
    /// `pulldown-cmark`'s link-bracket lookahead splits `[!NOTE]` into three
    /// separate text events — `"["`, `"!NOTE"`, `"]"` — so the tag is only ever
    /// visible once the run is stitched back together.
    ///
    /// `limit` caps the work done during `supports_node`, which runs against every
    /// blockquote in the document; a tag is always within the first few bytes, so
    /// there is no reason to walk a long quote just to answer "is this a callout?".
    fn flatten(children: &[VoiceAstNode], limit: Option<usize>, out: &mut String) {
        for child in children {
            if limit.is_some_and(|max| out.len() >= max) {
                return;
            }
            match child {
                VoiceAstNode::Text { text } | VoiceAstNode::CodeSpan { text } => out.push_str(text),
                VoiceAstNode::SoftBreak | VoiceAstNode::HardBreak => out.push(' '),
                VoiceAstNode::Paragraph { children }
                | VoiceAstNode::Emphasis { children }
                | VoiceAstNode::Strong { children } => {
                    if !out.is_empty() && !out.ends_with(' ') {
                        out.push(' ');
                    }
                    Self::flatten(children, limit, out);
                }
                VoiceAstNode::Link { text, .. } => Self::flatten(text, limit, out),
                _ => {}
            }
        }
    }

    /// The full spoken form of a callout blockquote, or `None` if it isn't one.
    fn callout_speech(children: &[VoiceAstNode]) -> Option<String> {
        let mut flat = String::new();
        Self::flatten(children, None, &mut flat);
        let (tag, remainder) = Self::extract_callout_tag(&flat)?;

        let announcement = match tag.to_uppercase().as_str() {
            "NOTE" => "Note callout.",
            "WARNING" => "Warning alert callout.",
            "IMPORTANT" => "Important announcement.",
            "TIP" => "Helpful tip.",
            "CAUTION" => "Cautionary alert.",
            _ => "Auditory alert callout.",
        };

        Some(if remainder.is_empty() {
            announcement.to_string()
        } else {
            format!("{announcement} {remainder}")
        })
    }
}

impl VoicePlugin for AdmonitionPlugin {
    fn name(&self) -> &'static str {
        "AdmonitionPlugin"
    }

    fn supports_node<'a>(&self, node: &VoiceAstNode<'a>) -> bool {
        if let VoiceAstNode::BlockQuote { children } = node {
            /* 64 bytes is far more than the longest `[!IMPORTANT]` needs, and keeps
            this cheap on quotes that turn out not to be callouts at all. */
            let mut head = String::new();
            Self::flatten(children, Some(64), &mut head);
            return Self::extract_callout_tag(&head).is_some();
        }
        false
    }

    fn transform<'a>(
        &self,
        node: &VoiceAstNode<'a>,
        _context: &mut TransformContext,
    ) -> Option<SpeechToken<'a>> {
        if let VoiceAstNode::BlockQuote { children } = node {
            let speech = Self::callout_speech(children)?;
            let spoken: &'a str = Box::leak(speech.into_boxed_str());
            return Some(SpeechToken::CustomSpeech(spoken));
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
            Some(SpeechToken::CustomSpeech(
                "Note callout. Database backup created."
            ))
        );
    }

    /// The test above hand-builds an idealised single-`Text` node. This one goes
    /// through the real parser, where `[!NOTE]` arrives as three separate text
    /// events — the case that actually occurs in a document.
    #[test]
    fn test_admonition_through_real_parser() {
        use mjx_md_voiceover_core::{PluginRegistry, SpeechFormatter, VoiceAstParser};

        let md = "> [!WARNING]\n> Dropping columns will break active clients.";
        let ast = VoiceAstParser::parse(md).unwrap();

        let mut registry = PluginRegistry::new();
        registry.register(AdmonitionPlugin::new());

        assert_eq!(
            SpeechFormatter::format_with_registry(&ast, &registry),
            "Warning alert callout. Dropping columns will break active clients."
        );
    }
}
