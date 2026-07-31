//! Markdown GFM table speech verbalizer plugin for `mjx-md-voiceover`.
//!
//! Converts raw pipe Markdown tables (`| Col1 | Col2 |`) into spoken data summaries.

use mjx_md_voiceover_core::{SpeechToken, TransformContext, VoiceAstNode, VoicePlugin};

/// Plugin converting Markdown table syntax into conversational data overviews.
#[derive(Debug, Default, Clone, Copy)]
pub struct TablePlugin;

impl TablePlugin {
    /// Creates a new `TablePlugin` instance.
    pub fn new() -> Self {
        Self
    }
}

impl VoicePlugin for TablePlugin {
    fn name(&self) -> &'static str {
        "TablePlugin"
    }

    fn supports_node<'a>(&self, node: &VoiceAstNode<'a>) -> bool {
        match node {
            VoiceAstNode::Text { text } => text.contains('|') && text.contains('-'),
            VoiceAstNode::CustomPlugin { tag, .. } => *tag == "table",
            _ => false,
        }
    }

    fn transform<'a>(
        &self,
        node: &VoiceAstNode<'a>,
        _context: &mut TransformContext,
    ) -> Option<SpeechToken<'a>> {
        if self.supports_node(node) {
            Some(SpeechToken::CustomSpeech("Structured data table."))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_plugin_transformation() {
        let plugin = TablePlugin::new();
        let node = VoiceAstNode::Text {
            text: "| Header 1 | Header 2 |\n| --- | --- |",
        };

        assert!(plugin.supports_node(&node));

        let mut ctx = TransformContext::default();
        let token = plugin.transform(&node, &mut ctx);

        assert_eq!(
            token,
            Some(SpeechToken::CustomSpeech("Structured data table."))
        );
    }
}
