//! Speech Formatter engine for `mjx-md-voiceover-core`.
//!
//! Transforms zero-copy Markdown AST nodes into continuous, natural speech text
//! optimized for Text-to-Speech (TTS) voice synthesis per `VOICEOVER_RULES.md`.

use crate::ast::{VoiceAst, VoiceAstNode};

/// Converts ordinal item index (0-based) to spoken english prefix.
fn ordinal_prefix(index: usize) -> &'static str {
    match index {
        0 => "First",
        1 => "Second",
        2 => "Third",
        3 => "Fourth",
        4 => "Fifth",
        5 => "Sixth",
        6 => "Seventh",
        7 => "Eighth",
        8 => "Ninth",
        9 => "Tenth",
        _ => "Next",
    }
}

/// Formatter engine converting `VoiceAst` hierarchies into natural speech prose.
pub struct SpeechFormatter;

impl SpeechFormatter {
    /// Renders a `VoiceAst` into a continuous speech-friendly string for TTS.
    ///
    /// # Performance
    /// Pre-allocates string buffer based on AST node depth to minimize dynamic re-allocations.
    pub fn format(ast: &VoiceAst) -> String {
        let registry = crate::plugin::PluginRegistry::new();
        Self::format_with_registry(ast, &registry)
    }

    /// Renders a `VoiceAst` with plugin transformations applied via `PluginRegistry`.
    pub fn format_with_registry(
        ast: &VoiceAst,
        registry: &crate::plugin::PluginRegistry,
    ) -> String {
        let mut out = String::with_capacity(256);
        let mut context = crate::ast::TransformContext::default();
        for node in &ast.nodes {
            Self::format_node_with_registry(node, registry, &mut context, &mut out);
        }
        out.trim().to_string()
    }

    fn format_node_with_registry(
        node: &VoiceAstNode,
        registry: &crate::plugin::PluginRegistry,
        context: &mut crate::ast::TransformContext,
        out: &mut String,
    ) {
        if let Some(token) = registry.transform_node(node, context) {
            match token {
                crate::ast::SpeechToken::Text(text)
                | crate::ast::SpeechToken::CustomSpeech(text) => {
                    if !out.is_empty() && !out.ends_with(' ') {
                        out.push(' ');
                    }
                    out.push_str(text);
                    Self::ensure_period(out);
                    return;
                }
                crate::ast::SpeechToken::VerbalizedCode {
                    language: _,
                    summary,
                } => {
                    if !out.is_empty() && !out.ends_with(' ') {
                        out.push(' ');
                    }
                    out.push_str(summary);
                    Self::ensure_period(out);
                    return;
                }
                crate::ast::SpeechToken::Pause(_) | crate::ast::SpeechToken::EmphasisCue(_) => {
                    return;
                }
            }
        }
        Self::format_node(node, registry, context, out);
    }

    /// Applies the built-in CommonMark speech rules to a single node.
    ///
    /// Takes the registry and context purely to hand them back to
    /// `format_node_with_registry` on the way down: a plugin must get its shot at
    /// *every* node, not just the ones sitting at the document root. Without this,
    /// a fenced block nested in a list item, or `$math$` inside a paragraph, would
    /// silently skip the registry and fall through to the default rule.
    fn format_node(
        node: &VoiceAstNode,
        registry: &crate::plugin::PluginRegistry,
        context: &mut crate::ast::TransformContext,
        out: &mut String,
    ) {
        match node {
            VoiceAstNode::Heading { level, children } => {
                if !out.is_empty() && !out.ends_with(' ') {
                    out.push(' ');
                }
                if *level == 1 {
                    out.push_str("Heading: ");
                } else {
                    out.push_str("Section: ");
                }
                Self::format_children(children, registry, context, out);
                Self::ensure_period(out);
            }
            VoiceAstNode::Paragraph { children } => {
                if !out.is_empty() && !out.ends_with(' ') {
                    out.push(' ');
                }
                Self::format_children(children, registry, context, out);
                Self::ensure_period(out);
            }
            VoiceAstNode::List {
                ordered,
                start: _,
                items,
            } => {
                for (idx, item) in items.iter().enumerate() {
                    if !out.is_empty() && !out.ends_with(' ') {
                        out.push(' ');
                    }
                    if *ordered {
                        out.push_str(ordinal_prefix(idx));
                        out.push_str(", ");
                    }
                    if let VoiceAstNode::ListItem { children } = item {
                        Self::format_children(children, registry, context, out);
                    } else {
                        Self::format_node(item, registry, context, out);
                    }
                    Self::ensure_period(out);
                }
            }
            VoiceAstNode::ListItem { children } => {
                Self::format_children(children, registry, context, out);
                Self::ensure_period(out);
            }
            VoiceAstNode::Emphasis { children } | VoiceAstNode::Strong { children } => {
                Self::format_children(children, registry, context, out);
            }
            VoiceAstNode::CodeSpan { text } => {
                out.push_str(text.trim());
            }
            VoiceAstNode::CodeBlock { language, code: _ } => {
                if !out.is_empty() && !out.ends_with(' ') {
                    out.push(' ');
                }
                match language {
                    Some(lang) => {
                        out.push_str("Code snippet in ");
                        out.push_str(lang.trim());
                    }
                    None => {
                        out.push_str("Code snippet");
                    }
                }
                out.push('.');
            }
            VoiceAstNode::BlockQuote { children } => {
                if !out.is_empty() && !out.ends_with(' ') {
                    out.push(' ');
                }
                out.push_str("Quote: ");
                Self::format_children(children, registry, context, out);
                Self::ensure_period(out);
            }
            VoiceAstNode::Link { text, .. } => {
                Self::format_children(text, registry, context, out);
            }
            VoiceAstNode::Text { text } => {
                out.push_str(text);
            }
            VoiceAstNode::SoftBreak | VoiceAstNode::HardBreak => {
                if !out.ends_with(' ') {
                    out.push(' ');
                }
            }
            VoiceAstNode::ThematicBreak => {
                if !out.is_empty() && !out.ends_with(' ') {
                    out.push(' ');
                }
            }
            VoiceAstNode::Table { .. } => {
                // Plugin path handles rich verbalization; bare core falls back.
                if !out.is_empty() && !out.ends_with(' ') {
                    out.push(' ');
                }
                out.push_str("Table.");
            }
            VoiceAstNode::CustomPlugin { tag: _, payload } => {
                out.push_str(payload);
            }
        }
    }

    fn format_children(
        children: &[VoiceAstNode],
        registry: &crate::plugin::PluginRegistry,
        context: &mut crate::ast::TransformContext,
        out: &mut String,
    ) {
        for child in children {
            Self::format_node_with_registry(child, registry, context, out);
        }
    }

    fn ensure_period(out: &mut String) {
        let trimmed = out.trim_end();
        if !trimmed.ends_with('.') && !trimmed.ends_with('!') && !trimmed.ends_with('?') {
            out.push('.');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::VoiceAstParser;

    #[test]
    fn test_format_heading_and_paragraph() {
        let md = "# Title\n\nThis is a test paragraph.";
        let ast = VoiceAstParser::parse(md).unwrap();
        let speech = SpeechFormatter::format(&ast);
        assert_eq!(speech, "Heading: Title. This is a test paragraph.");
    }

    #[test]
    fn test_format_section_heading() {
        let md = "## Subsection";
        let ast = VoiceAstParser::parse(md).unwrap();
        let speech = SpeechFormatter::format(&ast);
        assert_eq!(speech, "Section: Subsection.");
    }

    #[test]
    fn test_format_emphasis_and_bold() {
        let md = "Some *italic* and **bold** text.";
        let ast = VoiceAstParser::parse(md).unwrap();
        let speech = SpeechFormatter::format(&ast);
        assert_eq!(speech, "Some italic and bold text.");
    }

    #[test]
    fn test_format_unordered_list() {
        let md = "- First item\n- Second item";
        let ast = VoiceAstParser::parse(md).unwrap();
        let speech = SpeechFormatter::format(&ast);
        assert_eq!(speech, "First item. Second item.");
    }

    #[test]
    fn test_format_ordered_list() {
        let md = "1. First item\n2. Second item";
        let ast = VoiceAstParser::parse(md).unwrap();
        let speech = SpeechFormatter::format(&ast);
        assert_eq!(speech, "First, First item. Second, Second item.");
    }

    #[test]
    fn test_format_blockquote() {
        let md = "> Wise words.";
        let ast = VoiceAstParser::parse(md).unwrap();
        let speech = SpeechFormatter::format(&ast);
        assert_eq!(speech, "Quote: Wise words.");
    }

    #[test]
    fn test_format_link_and_code_span() {
        let md = "Check [Google](https://google.com) and `run()` code.";
        let ast = VoiceAstParser::parse(md).unwrap();
        let speech = SpeechFormatter::format(&ast);
        assert_eq!(speech, "Check Google and run() code.");
    }

    #[test]
    fn test_format_code_block() {
        let md = "```rust\nfn main() {}\n```";
        let ast = VoiceAstParser::parse(md).unwrap();
        let speech = SpeechFormatter::format(&ast);
        assert_eq!(speech, "Code snippet in rust.");
    }

    #[test]
    fn test_format_table_fallback_without_plugin() {
        let md = "| A | B |\n| --- | --- |\n| 1 | 2 |\n";
        let ast = VoiceAstParser::parse(md).unwrap();
        let speech = SpeechFormatter::format(&ast);
        assert_eq!(speech, "Table.");
    }
}
