//! Plugin trait and execution registry for `mjx-md-voiceover`.
//!
//! Provides lifecycle hooks for intercepting AST nodes and emitting custom speech tokens
//! without compromising WASM compatibility or latency SLAs (<1-10 ms).

use crate::ast::{SpeechToken, TransformContext, VoiceAstNode};

/// Core plugin interface for extending Markdown voiceover conversion.
pub trait VoicePlugin: Send + Sync {
    /// Unique identifier for the plugin (e.g. `"CodeBlockPlugin"`).
    fn name(&self) -> &'static str;

    /// Checks if this plugin handles the given AST node.
    fn supports_node<'a>(&self, node: &VoiceAstNode<'a>) -> bool;

    /// Transforms an AST node into speech tokens or custom speech output.
    fn transform<'a>(
        &self,
        node: &VoiceAstNode<'a>,
        context: &mut TransformContext,
    ) -> Option<SpeechToken<'a>>;
}

/// Registry managing active voiceover plugins.
#[derive(Default)]
pub struct PluginRegistry {
    plugins: Vec<Box<dyn VoicePlugin>>,
}

impl PluginRegistry {
    /// Creates a new empty `PluginRegistry`.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Registers a new plugin instance.
    pub fn register<P: VoicePlugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }

    /// Iterates registered plugins to transform a node if supported.
    pub fn transform_node<'a>(
        &self,
        node: &VoiceAstNode<'a>,
        context: &mut TransformContext,
    ) -> Option<SpeechToken<'a>> {
        for plugin in &self.plugins {
            if plugin.supports_node(node) {
                if let Some(token) = plugin.transform(node, context) {
                    return Some(token);
                }
            }
        }
        None
    }

    /// Returns the number of registered plugins.
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Returns true if no plugins are registered.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::PauseDuration;

    struct DummyPlugin;

    impl VoicePlugin for DummyPlugin {
        fn name(&self) -> &'static str {
            "DummyPlugin"
        }

        fn supports_node<'a>(&self, node: &VoiceAstNode<'a>) -> bool {
            matches!(node, VoiceAstNode::ThematicBreak)
        }

        fn transform<'a>(
            &self,
            _node: &VoiceAstNode<'a>,
            _context: &mut TransformContext,
        ) -> Option<SpeechToken<'a>> {
            Some(SpeechToken::Pause(PauseDuration::Long))
        }
    }

    #[test]
    fn test_plugin_registration_and_transformation() {
        let mut registry = PluginRegistry::new();
        assert!(registry.is_empty());

        registry.register(DummyPlugin);
        assert_eq!(registry.len(), 1);

        let mut ctx = TransformContext::default();
        let rule_node = VoiceAstNode::ThematicBreak;
        let text_node = VoiceAstNode::Text { text: "hello" };

        let token = registry.transform_node(&rule_node, &mut ctx);
        assert_eq!(token, Some(SpeechToken::Pause(PauseDuration::Long)));

        let unhandled_token = registry.transform_node(&text_node, &mut ctx);
        assert_eq!(unhandled_token, None);
    }
}
