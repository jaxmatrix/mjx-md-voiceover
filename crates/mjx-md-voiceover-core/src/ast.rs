//! Markdown Voice AST and Speech Token Definitions.
//!
//! Provides zero-copy slice-based structures representing Markdown syntax nodes
//! and speech synthesis tokens.

use serde::{Deserialize, Serialize};

/// Duration level for speech pauses injected into output text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PauseDuration {
    /// Brief pause (e.g. comma, list item boundary).
    Short,
    /// Medium pause (e.g. period, paragraph boundary, section break).
    Medium,
    /// Long pause (e.g. major section transition, thematic break).
    Long,
}

/// Type of emphasis cue for speech modulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmphasisType {
    /// Italic emphasis.
    Italic,
    /// Strong / bold emphasis.
    Strong,
}

/// Individual speech token emitted during AST transformation for TTS audio synthesis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SpeechToken<'a> {
    /// Plain spoken text content.
    Text(&'a str),
    /// Injected pacing pause for TTS natural rhythm.
    Pause(PauseDuration),
    /// Speech modulation / emphasis cue marker.
    EmphasisCue(EmphasisType),
    /// Verbalized code snippet or language description.
    VerbalizedCode {
        /// Code language identifier if present.
        language: Option<&'a str>,
        /// Friendly spoken description of the code fence.
        summary: &'a str,
    },
    /// Custom plugin speech output string.
    CustomSpeech(&'a str),
}

/// Node representation in the Markdown Voice AST.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum VoiceAstNode<'a> {
    /// Heading element (`#`, `##`, etc.).
    Heading {
        /// Heading depth level (1 to 6).
        level: u8,
        /// Inner inline AST nodes.
        children: Vec<VoiceAstNode<'a>>,
    },
    /// Standard paragraph block.
    Paragraph {
        /// Inner inline AST nodes.
        children: Vec<VoiceAstNode<'a>>,
    },
    /// Ordered or unordered list container.
    List {
        /// True if ordered (1., 2.), false if bullet (-).
        ordered: bool,
        /// Starting index for ordered lists.
        start: Option<u64>,
        /// List item child nodes.
        items: Vec<VoiceAstNode<'a>>,
    },
    /// Single item inside a list.
    ListItem {
        /// Item content AST nodes.
        children: Vec<VoiceAstNode<'a>>,
    },
    /// Italic text (`*text*` or `_text_`).
    Emphasis {
        /// Inner AST nodes.
        children: Vec<VoiceAstNode<'a>>,
    },
    /// Bold text (`**text**` or `__text__`).
    Strong {
        /// Inner AST nodes.
        children: Vec<VoiceAstNode<'a>>,
    },
    /// Inline code span (`` `code` ``).
    CodeSpan {
        /// Literal inline code string.
        text: &'a str,
    },
    /// Fenced or indented code block.
    CodeBlock {
        /// Programming language identifier if specified.
        language: Option<&'a str>,
        /// Raw code fence content.
        code: &'a str,
    },
    /// Blockquote container (`> Quote`).
    BlockQuote {
        /// Inner block AST nodes.
        children: Vec<VoiceAstNode<'a>>,
    },
    /// Hyperlink element (`[text](url)`).
    Link {
        /// Link display text AST nodes.
        text: Vec<VoiceAstNode<'a>>,
        /// Target URL reference slice.
        url: &'a str,
        /// Optional link title attribute slice.
        title: Option<&'a str>,
    },
    /// Plain text leaf node.
    Text {
        /// Text slice reference.
        text: &'a str,
    },
    /// Soft line break in Markdown.
    SoftBreak,
    /// Hard line break (`  \n` or `\n`).
    HardBreak,
    /// Horizontal rule / thematic break (`---`).
    ThematicBreak,
    /// GFM pipe table (`| Col | … |` with separator row).
    ///
    /// `headers` is one cell per column (inline children). `rows` is body rows,
    /// each a list of cells, each cell a list of inline children.
    Table {
        /// Header row cells (left-to-right).
        headers: Vec<Vec<VoiceAstNode<'a>>>,
        /// Body rows; each row is a list of cells.
        rows: Vec<Vec<Vec<VoiceAstNode<'a>>>>,
    },
    /// Specialized plugin extension node.
    CustomPlugin {
        /// Plugin unique identifier tag.
        tag: &'a str,
        /// Raw payload data for plugin transformation.
        payload: &'a str,
    },
}

/// Root AST container holding document nodes and metadata.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct VoiceAst<'a> {
    /// Top-level AST nodes of the Markdown document.
    pub nodes: Vec<VoiceAstNode<'a>>,
}

impl<'a> VoiceAst<'a> {
    /// Creates a new empty `VoiceAst`.
    #[inline]
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Appends a top-level node to the AST.
    #[inline]
    pub fn push(&mut self, node: VoiceAstNode<'a>) {
        self.nodes.push(node);
    }
}

/// Transformation context tracking state during AST traversal and speech generation.
#[derive(Debug, Clone, Default)]
pub struct TransformContext {
    /// Current depth level in nested structures (e.g. lists, blockquotes).
    pub depth: usize,
    /// Pre-allocated output capacity estimate for minimizing re-allocations.
    pub capacity_hint: usize,
}

impl TransformContext {
    /// Creates a new `TransformContext` with default capacity hints.
    #[inline]
    pub fn new(capacity_hint: usize) -> Self {
        Self {
            depth: 0,
            capacity_hint,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_node_creation_and_nesting() {
        let mut ast = VoiceAst::new();
        let heading = VoiceAstNode::Heading {
            level: 1,
            children: vec![VoiceAstNode::Text { text: "Title" }],
        };
        ast.push(heading);

        assert_eq!(ast.nodes.len(), 1);
        if let VoiceAstNode::Heading { level, children } = &ast.nodes[0] {
            assert_eq!(*level, 1);
            assert_eq!(children.len(), 1);
            assert_eq!(children[0], VoiceAstNode::Text { text: "Title" });
        } else {
            panic!("Expected Heading node");
        }
    }

    #[test]
    fn test_speech_token_variants() {
        let text_token = SpeechToken::Text("Hello world");
        let pause_token = SpeechToken::Pause(PauseDuration::Medium);
        let code_token = SpeechToken::VerbalizedCode {
            language: Some("rust"),
            summary: "Defines main function",
        };

        assert_eq!(text_token, SpeechToken::Text("Hello world"));
        assert_eq!(pause_token, SpeechToken::Pause(PauseDuration::Medium));
        if let SpeechToken::VerbalizedCode { language, summary } = code_token {
            assert_eq!(language, Some("rust"));
            assert_eq!(summary, "Defines main function");
        } else {
            panic!("Expected VerbalizedCode token");
        }
    }

    #[test]
    fn test_transform_context() {
        let ctx = TransformContext::new(1024);
        assert_eq!(ctx.capacity_hint, 1024);
        assert_eq!(ctx.depth, 0);
    }
}
