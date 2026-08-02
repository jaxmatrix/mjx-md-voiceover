//! LaTeX math speechifier plugin for `mjx-md-voiceover`.
//!
//! Converts inline `$ ... $` and block `$$ ... $$` math formulas into plain English spoken text.

use mjx_md_voiceover_core::{SpeechToken, TransformContext, VoiceAstNode, VoicePlugin};

/// Plugin converting LaTeX math formulas into natural spoken English expressions.
#[derive(Debug, Default, Clone, Copy)]
pub struct LatexMathPlugin;

impl LatexMathPlugin {
    /// Creates a new `LatexMathPlugin` instance.
    pub fn new() -> Self {
        Self
    }

    /// Rewrites only the `$…$` / `$$…$$` spans inside a prose run.
    ///
    /// A `Text` node is mostly ordinary words, so it cannot be handed to
    /// `speechify` wholesale: that would leave the `$` delimiters in the output
    /// and turn every stray hyphen in the sentence into " minus ". Only the maths
    /// between delimiters is spoken; the prose around it passes through untouched.
    pub fn speechify_inline(raw: &str) -> String {
        let mut out = String::with_capacity(raw.len());
        let mut rest = raw;

        while let Some(start) = rest.find('$') {
            out.push_str(&rest[..start]);
            let after = &rest[start..];

            /* `$$` block maths closes on `$$`; inline `$` closes on the next `$`. */
            let delim = if after.starts_with("$$") { "$$" } else { "$" };

            match after[delim.len()..].find(delim) {
                Some(offset) => {
                    let body = &after[delim.len()..delim.len() + offset];
                    out.push_str(Self::speechify(body).trim());
                    rest = &after[delim.len() + offset + delim.len()..];
                }
                /* Unterminated — a lone `$` is a currency sign, not maths.
                Emit the remainder verbatim rather than swallowing it. */
                None => {
                    out.push_str(after);
                    return out;
                }
            }
        }

        out.push_str(rest);
        out
    }

    /// Converts LaTeX formula syntax string into spoken English words.
    pub fn speechify(raw_math: &str) -> String {
        let trimmed = raw_math.trim().trim_matches('$').trim();
        let mut out = String::with_capacity(trimmed.len() * 2);

        let mut chars = trimmed.chars().peekable();
        while let Some(ch) = chars.next() {
            match ch {
                '+' => out.push_str(" plus "),
                '-' => out.push_str(" minus "),
                '=' => out.push_str(" equals "),
                '*' => out.push_str(" times "),
                '/' => out.push_str(" divided by "),
                '^' => {
                    if let Some(&next) = chars.peek() {
                        if next == '2' {
                            chars.next();
                            out.push_str(" squared");
                            continue;
                        } else if next == '3' {
                            chars.next();
                            out.push_str(" cubed");
                            continue;
                        }
                    }
                    out.push_str(" to the power of ");
                }
                '\\' => {
                    let mut cmd = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphabetic() {
                            cmd.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    match cmd.as_str() {
                        "alpha" => out.push_str("alpha"),
                        "beta" => out.push_str("beta"),
                        "gamma" => out.push_str("gamma"),
                        "delta" => out.push_str("delta"),
                        "pi" => out.push_str("pi"),
                        "theta" => out.push_str("theta"),
                        "sqrt" => out.push_str("square root of "),
                        "frac" => out.push_str("fraction "),
                        "sum" => out.push_str("sum of "),
                        "int" => out.push_str("integral of "),
                        "times" => out.push_str(" times "),
                        "cdot" => out.push_str(" dot "),
                        "infty" => out.push_str("infinity"),
                        _ => {
                            out.push_str(&cmd);
                        }
                    }
                }
                '{' | '}' => {
                    out.push(' ');
                }
                _ => {
                    out.push(ch);
                }
            }
        }

        // Clean up redundant whitespace
        out.split_whitespace().collect::<Vec<_>>().join(" ")
    }
}

impl VoicePlugin for LatexMathPlugin {
    fn name(&self) -> &'static str {
        "LatexMathPlugin"
    }

    fn supports_node<'a>(&self, node: &VoiceAstNode<'a>) -> bool {
        match node {
            VoiceAstNode::CustomPlugin { tag, .. } => *tag == "math" || *tag == "latex",
            VoiceAstNode::CodeSpan { text } => text.starts_with('$') && text.ends_with('$'),
            /* Two delimiters, or it is a price rather than a formula. */
            VoiceAstNode::Text { text } => text.matches('$').count() >= 2,
            _ => false,
        }
    }

    fn transform<'a>(
        &self,
        node: &VoiceAstNode<'a>,
        _context: &mut TransformContext,
    ) -> Option<SpeechToken<'a>> {
        let math_text = match node {
            VoiceAstNode::CustomPlugin { payload, .. } => *payload,
            VoiceAstNode::CodeSpan { text } => *text,
            /* Prose with maths embedded in it — rewrite the spans, keep the words. */
            VoiceAstNode::Text { text } if text.matches('$').count() >= 2 => {
                let spoken: &'a str = Box::leak(Self::speechify_inline(text).into_boxed_str());
                return Some(SpeechToken::CustomSpeech(spoken));
            }
            _ => return None,
        };

        let spoken: &'a str = Box::leak(Self::speechify(math_text).into_boxed_str());
        Some(SpeechToken::CustomSpeech(spoken))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speechify_formula() {
        assert_eq!(
            LatexMathPlugin::speechify("$a^2 + b^2 = c^2$"),
            "a squared plus b squared equals c squared"
        );
        assert_eq!(LatexMathPlugin::speechify(r"\frac{a}{b}"), "fraction a b");
        assert_eq!(LatexMathPlugin::speechify(r"\sqrt{x}"), "square root of x");
    }

    #[test]
    fn test_speechify_inline_keeps_surrounding_prose() {
        assert_eq!(
            LatexMathPlugin::speechify_inline("The formula $a^2 + b^2 = c^2$ is famous."),
            "The formula a squared plus b squared equals c squared is famous."
        );
        /* A lone `$` is currency; the sentence must survive intact. */
        assert_eq!(
            LatexMathPlugin::speechify_inline("It cost $5 to run."),
            "It cost $5 to run."
        );
    }

    #[test]
    fn test_plugin_node_transformation() {
        let plugin = LatexMathPlugin::new();
        let node = VoiceAstNode::CodeSpan {
            text: "$a^2 + b^2 = c^2$",
        };
        assert!(plugin.supports_node(&node));

        let mut ctx = TransformContext::default();
        let token = plugin.transform(&node, &mut ctx);

        assert_eq!(
            token,
            Some(SpeechToken::CustomSpeech(
                "a squared plus b squared equals c squared"
            ))
        );
    }
}
