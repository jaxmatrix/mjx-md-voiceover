//! Zero-copy CommonMark AST Parser for `mjx-md-voiceover-core`.
//!
//! Wraps `pulldown_cmark` event stream to construct a `VoiceAst<'a>` hierarchy
//! with sub-millisecond execution latency (<1-10 ms budget).

use crate::ast::{VoiceAst, VoiceAstNode};
use crate::Result;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag};

/// Zero-copy Markdown Voice AST parser.
pub struct VoiceAstParser;

#[derive(Debug)]
enum Frame<'a> {
    Heading {
        level: u8,
        children: Vec<VoiceAstNode<'a>>,
    },
    Paragraph {
        children: Vec<VoiceAstNode<'a>>,
    },
    List {
        ordered: bool,
        start: Option<u64>,
        items: Vec<VoiceAstNode<'a>>,
    },
    ListItem {
        children: Vec<VoiceAstNode<'a>>,
    },
    Emphasis {
        children: Vec<VoiceAstNode<'a>>,
    },
    Strong {
        children: Vec<VoiceAstNode<'a>>,
    },
    BlockQuote {
        children: Vec<VoiceAstNode<'a>>,
    },
    Link {
        text: Vec<VoiceAstNode<'a>>,
        url: &'a str,
        title: Option<&'a str>,
    },
    CodeBlock {
        language: Option<&'a str>,
        code_buf: String,
    },
    /// GFM table container; headers/rows filled by nested head/row frames.
    Table {
        headers: Vec<Vec<VoiceAstNode<'a>>>,
        rows: Vec<Vec<Vec<VoiceAstNode<'a>>>>,
    },
    /// Header section: cells only (pulldown-cmark does not wrap them in `TableRow`).
    TableHead {
        cells: Vec<Vec<VoiceAstNode<'a>>>,
    },
    /// Body row of cells.
    TableRow {
        cells: Vec<Vec<VoiceAstNode<'a>>>,
    },
    /// Single header or body cell (inline children).
    TableCell {
        children: Vec<VoiceAstNode<'a>>,
    },
}

impl<'a> Frame<'a> {
    fn add_child(&mut self, child: VoiceAstNode<'a>) {
        match self {
            Frame::Heading { children, .. }
            | Frame::Paragraph { children }
            | Frame::ListItem { children }
            | Frame::Emphasis { children }
            | Frame::Strong { children }
            | Frame::BlockQuote { children }
            | Frame::Link { text: children, .. }
            | Frame::TableCell { children } => {
                children.push(child);
            }
            Frame::List { items, .. } => {
                items.push(child);
            }
            Frame::CodeBlock { code_buf, .. } => {
                if let VoiceAstNode::Text { text } = child {
                    code_buf.push_str(text);
                }
            }
            Frame::Table { .. } | Frame::TableHead { .. } | Frame::TableRow { .. } => {
                // Structural frames receive cells via `add_cell`, not free children.
            }
        }
    }

    fn add_cell(&mut self, cell: Vec<VoiceAstNode<'a>>) {
        match self {
            Frame::TableHead { cells } | Frame::TableRow { cells } => {
                cells.push(cell);
            }
            _ => {}
        }
    }
}

#[inline]
fn cow_to_str<'a>(cow: CowStr<'a>) -> &'a str {
    match cow {
        CowStr::Borrowed(s) => s,
        CowStr::Inlined(s) => Box::leak(s.to_string().into_boxed_str()),
        CowStr::Boxed(s) => Box::leak(s),
    }
}

impl VoiceAstParser {
    /// Parses a raw Markdown string into a zero-copy `VoiceAst<'a>`.
    ///
    /// # Performance
    /// Executes in sub-millisecond time (<1 ms target) for standard inputs.
    pub fn parse<'a>(markdown: &'a str) -> Result<VoiceAst<'a>> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(markdown, options);
        let mut ast = VoiceAst::new();
        let mut stack: Vec<Frame<'a>> = Vec::with_capacity(16);

        for event in parser {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Heading { level, .. } => {
                        let lvl = match level {
                            HeadingLevel::H1 => 1,
                            HeadingLevel::H2 => 2,
                            HeadingLevel::H3 => 3,
                            HeadingLevel::H4 => 4,
                            HeadingLevel::H5 => 5,
                            HeadingLevel::H6 => 6,
                        };
                        stack.push(Frame::Heading {
                            level: lvl,
                            children: Vec::new(),
                        });
                    }
                    Tag::Paragraph => {
                        stack.push(Frame::Paragraph {
                            children: Vec::new(),
                        });
                    }
                    Tag::List(first_num) => {
                        stack.push(Frame::List {
                            ordered: first_num.is_some(),
                            start: first_num,
                            items: Vec::new(),
                        });
                    }
                    Tag::Item => {
                        stack.push(Frame::ListItem {
                            children: Vec::new(),
                        });
                    }
                    Tag::Emphasis => {
                        stack.push(Frame::Emphasis {
                            children: Vec::new(),
                        });
                    }
                    Tag::Strong => {
                        stack.push(Frame::Strong {
                            children: Vec::new(),
                        });
                    }
                    Tag::BlockQuote(_kind) => {
                        stack.push(Frame::BlockQuote {
                            children: Vec::new(),
                        });
                    }
                    Tag::Link {
                        dest_url, title, ..
                    } => {
                        let url_str = cow_to_str(dest_url);
                        let title_str = if title.is_empty() {
                            None
                        } else {
                            Some(cow_to_str(title))
                        };
                        stack.push(Frame::Link {
                            text: Vec::new(),
                            url: url_str,
                            title: title_str,
                        });
                    }
                    Tag::CodeBlock(kind) => {
                        let lang = match kind {
                            CodeBlockKind::Fenced(lang) => {
                                let l = cow_to_str(lang);
                                if l.is_empty() {
                                    None
                                } else {
                                    Some(l)
                                }
                            }
                            CodeBlockKind::Indented => None,
                        };
                        stack.push(Frame::CodeBlock {
                            language: lang,
                            code_buf: String::new(),
                        });
                    }
                    Tag::Table(_alignments) => {
                        stack.push(Frame::Table {
                            headers: Vec::new(),
                            rows: Vec::new(),
                        });
                    }
                    Tag::TableHead => {
                        stack.push(Frame::TableHead { cells: Vec::new() });
                    }
                    Tag::TableRow => {
                        stack.push(Frame::TableRow { cells: Vec::new() });
                    }
                    Tag::TableCell => {
                        stack.push(Frame::TableCell {
                            children: Vec::new(),
                        });
                    }
                    _ => {}
                },
                Event::End(_tag_end) => {
                    let Some(frame) = stack.pop() else {
                        continue;
                    };

                    // Table structural frames fold into the parent without emitting a node.
                    match frame {
                        Frame::TableCell { children } => {
                            if let Some(parent) = stack.last_mut() {
                                parent.add_cell(children);
                            }
                            continue;
                        }
                        Frame::TableHead { cells } => {
                            if let Some(Frame::Table { headers, .. }) = stack.last_mut() {
                                *headers = cells;
                            }
                            continue;
                        }
                        Frame::TableRow { cells } => {
                            if let Some(Frame::Table { rows, .. }) = stack.last_mut() {
                                rows.push(cells);
                            }
                            continue;
                        }
                        frame => {
                            let completed_node = match frame {
                                Frame::Heading { level, children } => {
                                    VoiceAstNode::Heading { level, children }
                                }
                                Frame::Paragraph { children } => {
                                    VoiceAstNode::Paragraph { children }
                                }
                                Frame::List {
                                    ordered,
                                    start,
                                    items,
                                } => VoiceAstNode::List {
                                    ordered,
                                    start,
                                    items,
                                },
                                Frame::ListItem { children } => VoiceAstNode::ListItem { children },
                                Frame::Emphasis { children } => VoiceAstNode::Emphasis { children },
                                Frame::Strong { children } => VoiceAstNode::Strong { children },
                                Frame::BlockQuote { children } => {
                                    VoiceAstNode::BlockQuote { children }
                                }
                                Frame::Link { text, url, title } => {
                                    VoiceAstNode::Link { text, url, title }
                                }
                                Frame::CodeBlock { language, code_buf } => {
                                    let code_slice: &'a str = Box::leak(code_buf.into_boxed_str());
                                    VoiceAstNode::CodeBlock {
                                        language,
                                        code: code_slice,
                                    }
                                }
                                Frame::Table { headers, rows } => {
                                    VoiceAstNode::Table { headers, rows }
                                }
                                Frame::TableCell { .. }
                                | Frame::TableHead { .. }
                                | Frame::TableRow { .. } => {
                                    // Handled above; unreachable for exhaustiveness.
                                    continue;
                                }
                            };

                            if let Some(parent) = stack.last_mut() {
                                parent.add_child(completed_node);
                            } else {
                                ast.push(completed_node);
                            }
                        }
                    }
                }
                Event::Text(text) => {
                    let slice: &'a str = cow_to_str(text);
                    let node = VoiceAstNode::Text { text: slice };
                    if let Some(parent) = stack.last_mut() {
                        parent.add_child(node);
                    } else {
                        ast.push(node);
                    }
                }
                Event::Code(code) => {
                    let slice: &'a str = cow_to_str(code);
                    let node = VoiceAstNode::CodeSpan { text: slice };
                    if let Some(parent) = stack.last_mut() {
                        parent.add_child(node);
                    } else {
                        ast.push(node);
                    }
                }
                Event::Rule => {
                    let node = VoiceAstNode::ThematicBreak;
                    if let Some(parent) = stack.last_mut() {
                        parent.add_child(node);
                    } else {
                        ast.push(node);
                    }
                }
                Event::SoftBreak => {
                    let node = VoiceAstNode::SoftBreak;
                    if let Some(parent) = stack.last_mut() {
                        parent.add_child(node);
                    } else {
                        ast.push(node);
                    }
                }
                Event::HardBreak => {
                    let node = VoiceAstNode::HardBreak;
                    if let Some(parent) = stack.last_mut() {
                        parent.add_child(node);
                    } else {
                        ast.push(node);
                    }
                }
                _ => {}
            }
        }

        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading_and_paragraph() {
        let md = "# Title\n\nThis is a test paragraph.";
        let ast = VoiceAstParser::parse(md).expect("Parsing failed");

        assert_eq!(ast.nodes.len(), 2);

        match &ast.nodes[0] {
            VoiceAstNode::Heading { level, children } => {
                assert_eq!(*level, 1);
                assert_eq!(children.len(), 1);
                assert_eq!(children[0], VoiceAstNode::Text { text: "Title" });
            }
            _ => panic!("Expected Heading node"),
        }

        match &ast.nodes[1] {
            VoiceAstNode::Paragraph { children } => {
                assert_eq!(children.len(), 1);
                assert_eq!(
                    children[0],
                    VoiceAstNode::Text {
                        text: "This is a test paragraph."
                    }
                );
            }
            _ => panic!("Expected Paragraph node"),
        }
    }

    #[test]
    fn test_parse_emphasis_and_strong() {
        let md = "Some *italic* and **bold** text.";
        let ast = VoiceAstParser::parse(md).expect("Parsing failed");

        assert_eq!(ast.nodes.len(), 1);
        if let VoiceAstNode::Paragraph { children } = &ast.nodes[0] {
            assert!(children.contains(&VoiceAstNode::Emphasis {
                children: vec![VoiceAstNode::Text { text: "italic" }]
            }));
            assert!(children.contains(&VoiceAstNode::Strong {
                children: vec![VoiceAstNode::Text { text: "bold" }]
            }));
        } else {
            panic!("Expected Paragraph node");
        }
    }

    #[test]
    fn test_parse_lists() {
        let md = "- Item 1\n- Item 2";
        let ast = VoiceAstParser::parse(md).expect("Parsing failed");

        assert_eq!(ast.nodes.len(), 1);
        if let VoiceAstNode::List { ordered, items, .. } = &ast.nodes[0] {
            assert!(!ordered);
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected Unordered List node");
        }
    }

    #[test]
    fn test_parse_blockquote_and_code_span() {
        let md = "> Quote with `code` span.";
        let ast = VoiceAstParser::parse(md).expect("Parsing failed");

        assert_eq!(ast.nodes.len(), 1);
        if let VoiceAstNode::BlockQuote { children } = &ast.nodes[0] {
            assert!(!children.is_empty());
        } else {
            panic!("Expected BlockQuote node");
        }
    }

    #[test]
    fn test_parse_link_and_code_block() {
        let md = "[Google](https://google.com)\n\n```rust\nfn main() {}\n```";
        let ast = VoiceAstParser::parse(md).expect("Parsing failed");

        assert_eq!(ast.nodes.len(), 2);
        if let VoiceAstNode::Paragraph { children } = &ast.nodes[0] {
            if let VoiceAstNode::Link { url, text, .. } = &children[0] {
                assert_eq!(*url, "https://google.com");
                assert_eq!(text[0], VoiceAstNode::Text { text: "Google" });
            } else {
                panic!("Expected Link node");
            }
        } else {
            panic!("Expected Paragraph node for link");
        }

        if let VoiceAstNode::CodeBlock { language, code } = &ast.nodes[1] {
            assert_eq!(*language, Some("rust"));
            assert!(code.contains("fn main()"));
        } else {
            panic!("Expected CodeBlock node");
        }
    }

    #[test]
    fn test_parse_gfm_table() {
        let md = "| Name | Age |\n| --- | --- |\n| Ada | 36 |\n| Bob | 28 |\n";
        let ast = VoiceAstParser::parse(md).expect("Parsing failed");

        assert_eq!(ast.nodes.len(), 1);
        match &ast.nodes[0] {
            VoiceAstNode::Table { headers, rows } => {
                assert_eq!(headers.len(), 2);
                assert_eq!(headers[0], vec![VoiceAstNode::Text { text: "Name" }]);
                assert_eq!(headers[1], vec![VoiceAstNode::Text { text: "Age" }]);
                assert_eq!(rows.len(), 2);
                assert_eq!(rows[0][0], vec![VoiceAstNode::Text { text: "Ada" }]);
                assert_eq!(rows[0][1], vec![VoiceAstNode::Text { text: "36" }]);
                assert_eq!(rows[1][0], vec![VoiceAstNode::Text { text: "Bob" }]);
                assert_eq!(rows[1][1], vec![VoiceAstNode::Text { text: "28" }]);
            }
            _ => panic!("Expected Table node"),
        }
    }

    #[test]
    fn test_prose_with_pipes_is_not_a_table() {
        let md = "Use a | b - c in prose.";
        let ast = VoiceAstParser::parse(md).expect("Parsing failed");
        assert!(
            !ast.nodes
                .iter()
                .any(|n| matches!(n, VoiceAstNode::Table { .. })),
            "pipe/hyphen prose must not become a Table AST node"
        );
    }
}
