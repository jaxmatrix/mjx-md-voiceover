//! Markdown GFM table speech verbalizer plugin for `mjx-md-voiceover`.
//!
//! Converts parsed `VoiceAstNode::Table` nodes into spoken data summaries.
//! Does **not** claim ordinary prose that merely contains `|` or `-`.

use mjx_md_voiceover_core::{SpeechToken, TransformContext, VoiceAstNode, VoicePlugin};

/// Maximum body rows spoken before summarizing the remainder.
const MAX_SPOKEN_ROWS: usize = 3;

/// Plugin converting Markdown table AST nodes into conversational data overviews.
#[derive(Debug, Default, Clone, Copy)]
pub struct TablePlugin;

impl TablePlugin {
    /// Creates a new `TablePlugin` instance.
    pub fn new() -> Self {
        Self
    }

    /// Flattens inline cell children to plain text for speech.
    fn flatten_inlines(nodes: &[VoiceAstNode<'_>], out: &mut String) {
        for node in nodes {
            match node {
                VoiceAstNode::Text { text } | VoiceAstNode::CodeSpan { text } => {
                    out.push_str(text);
                }
                VoiceAstNode::SoftBreak | VoiceAstNode::HardBreak => {
                    if !out.ends_with(' ') {
                        out.push(' ');
                    }
                }
                VoiceAstNode::Emphasis { children }
                | VoiceAstNode::Strong { children }
                | VoiceAstNode::Paragraph { children } => {
                    Self::flatten_inlines(children, out);
                }
                VoiceAstNode::Link { text, .. } => {
                    Self::flatten_inlines(text, out);
                }
                _ => {}
            }
        }
    }

    fn cell_text(cell: &[VoiceAstNode<'_>]) -> String {
        let mut out = String::new();
        Self::flatten_inlines(cell, &mut out);
        out.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Joins column/cell labels with natural spoken conjunctions.
    fn join_with_and(parts: &[String]) -> String {
        match parts.len() {
            0 => String::new(),
            1 => parts[0].clone(),
            2 => {
                let mut s = String::with_capacity(parts[0].len() + parts[1].len() + 5);
                s.push_str(&parts[0]);
                s.push_str(" and ");
                s.push_str(&parts[1]);
                s
            }
            _ => {
                let mut s = String::new();
                for (i, part) in parts.iter().enumerate() {
                    if i + 1 == parts.len() {
                        s.push_str(", and ");
                        s.push_str(part);
                    } else if i == 0 {
                        s.push_str(part);
                    } else {
                        s.push_str(", ");
                        s.push_str(part);
                    }
                }
                s
            }
        }
    }

    /// Builds deterministic spoken prose for a table AST node.
    pub fn verbalize(
        headers: &[Vec<VoiceAstNode<'_>>],
        rows: &[Vec<Vec<VoiceAstNode<'_>>>],
    ) -> String {
        if headers.is_empty() && rows.is_empty() {
            return "Empty table.".to_string();
        }

        let header_labels: Vec<String> = headers
            .iter()
            .map(|cell| Self::cell_text(cell))
            .filter(|s| !s.is_empty())
            .collect();

        let mut out = String::with_capacity(128);

        if header_labels.is_empty() {
            out.push_str("Table with ");
            out.push_str(&rows.len().to_string());
            out.push_str(if rows.len() == 1 {
                " data row."
            } else {
                " data rows."
            });
        } else {
            out.push_str("Table with columns ");
            out.push_str(&Self::join_with_and(&header_labels));
            out.push_str(". ");
            out.push_str(&rows.len().to_string());
            out.push_str(if rows.len() == 1 {
                " data row."
            } else {
                " data rows."
            });
        }

        let spoken = rows.len().min(MAX_SPOKEN_ROWS);
        for (idx, row) in rows.iter().take(spoken).enumerate() {
            let cells: Vec<String> = row
                .iter()
                .map(|cell| Self::cell_text(cell))
                .filter(|s| !s.is_empty())
                .collect();
            out.push(' ');
            out.push_str("Row ");
            out.push_str(&(idx + 1).to_string());
            out.push_str(": ");
            if cells.is_empty() {
                out.push('.');
            } else {
                out.push_str(&cells.join(", "));
                out.push('.');
            }
        }

        if rows.len() > MAX_SPOKEN_ROWS {
            let remaining = rows.len() - MAX_SPOKEN_ROWS;
            out.push_str(" And ");
            out.push_str(&remaining.to_string());
            out.push_str(if remaining == 1 {
                " more row."
            } else {
                " more rows."
            });
        }

        out
    }
}

impl VoicePlugin for TablePlugin {
    fn name(&self) -> &'static str {
        "TablePlugin"
    }

    fn supports_node<'a>(&self, node: &VoiceAstNode<'a>) -> bool {
        match node {
            VoiceAstNode::Table { .. } => true,
            VoiceAstNode::CustomPlugin { tag, .. } => *tag == "table",
            _ => false,
        }
    }

    fn transform<'a>(
        &self,
        node: &VoiceAstNode<'a>,
        _context: &mut TransformContext,
    ) -> Option<SpeechToken<'a>> {
        let spoken = match node {
            VoiceAstNode::Table { headers, rows } => Self::verbalize(headers, rows),
            VoiceAstNode::CustomPlugin { tag, payload } if *tag == "table" => {
                if payload.trim().is_empty() {
                    "Empty table.".to_string()
                } else {
                    // Opaque custom payload — no GFM structure available.
                    "Table.".to_string()
                }
            }
            _ => return None,
        };

        let leaked: &'a str = Box::leak(spoken.into_boxed_str());
        Some(SpeechToken::CustomSpeech(leaked))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mjx_md_voiceover_core::{PluginRegistry, SpeechFormatter, VoiceAstParser};

    #[test]
    fn test_supports_table_ast_only() {
        let plugin = TablePlugin::new();
        let table = VoiceAstNode::Table {
            headers: vec![vec![VoiceAstNode::Text { text: "A" }]],
            rows: vec![],
        };
        let prose = VoiceAstNode::Text {
            text: "Use a | b - c in prose.",
        };
        assert!(plugin.supports_node(&table));
        assert!(!plugin.supports_node(&prose));
    }

    #[test]
    fn test_gfm_table_through_parser_and_formatter() {
        let md = "| Name | Age |\n| --- | --- |\n| Ada | 36 |\n| Bob | 28 |\n";
        let ast = VoiceAstParser::parse(md).expect("parse");
        let mut registry = PluginRegistry::new();
        registry.register(TablePlugin::new());
        let speech = SpeechFormatter::format_with_registry(&ast, &registry);
        assert_eq!(
            speech,
            "Table with columns Name and Age. 2 data rows. Row 1: Ada, 36. Row 2: Bob, 28."
        );
    }

    #[test]
    fn test_prose_with_pipe_and_hyphen_not_claimed() {
        let md = "Choose a | b - c carefully.";
        let ast = VoiceAstParser::parse(md).expect("parse");
        let mut registry = PluginRegistry::new();
        registry.register(TablePlugin::new());
        let speech = SpeechFormatter::format_with_registry(&ast, &registry);
        assert!(
            !speech.to_lowercase().contains("table"),
            "prose must not be verbalized as a table: {speech}"
        );
        assert!(speech.contains('|') || speech.contains("Choose"));
    }

    #[test]
    fn test_caps_at_three_rows_with_remainder() {
        let md = "| Col |\n| --- |\n| a |\n| b |\n| c |\n| d |\n| e |\n";
        let ast = VoiceAstParser::parse(md).expect("parse");
        let mut registry = PluginRegistry::new();
        registry.register(TablePlugin::new());
        let speech = SpeechFormatter::format_with_registry(&ast, &registry);
        assert_eq!(
            speech,
            "Table with columns Col. 5 data rows. Row 1: a. Row 2: b. Row 3: c. And 2 more rows."
        );
    }

    #[test]
    fn test_empty_custom_table_plugin_node() {
        let plugin = TablePlugin::new();
        let node = VoiceAstNode::CustomPlugin {
            tag: "table",
            payload: "",
        };
        let mut ctx = TransformContext::default();
        assert_eq!(
            plugin.transform(&node, &mut ctx),
            Some(SpeechToken::CustomSpeech("Empty table."))
        );
    }

    #[test]
    fn test_three_column_oxford_and() {
        let headers = vec![
            vec![VoiceAstNode::Text { text: "A" }],
            vec![VoiceAstNode::Text { text: "B" }],
            vec![VoiceAstNode::Text { text: "C" }],
        ];
        let rows: Vec<Vec<Vec<VoiceAstNode>>> = vec![];
        assert_eq!(
            TablePlugin::verbalize(&headers, &rows),
            "Table with columns A, B, and C. 0 data rows."
        );
    }
}
