# Plugin System Specification: mjx-md-voiceover

## Overview

The `mjx-md-voiceover` plugin system allows users to extend markdown voiceover conversion beyond base CommonMark elements without compromising parsing speed or WASM compatibility.

## Plugin Interface Design

A plugin implements the `VoicePlugin` trait:

```rust
pub trait VoicePlugin: Send + Sync {
    /// Unique identifier for the plugin
    fn name(&self) -> &'static str;

    /// Called when the parser encounters a custom syntax token or block
    fn supports_node(&self, node: &VoiceAstNode) -> bool;

    /// Transform an AST node into speech tokens or natural spoken text
    fn transform(&self, node: &VoiceAstNode, context: &mut TransformContext) -> Option<SpeechToken>;
}
```

## Supported Plugin Extensions

1. **Code Verbalizer Plugin (`CodeBlockPlugin`)**
   - Intercepts fenced code blocks (e.g. ````rust fn main() {} ````).
   - Generates natural speech descriptions such as: `"Code snippet in Rust. Defines function main."`

2. **LaTeX Math Speechifier (`LatexMathPlugin`)**
   - Intercepts inline `$a^2 + b^2 = c^2$` and block `$$ ... $$` math tokens.
   - Converts formulas to plain spoken English: `"a squared plus b squared equals c squared."`

3. **Callout / Admonition Plugin (`AdmonitionPlugin`)**
   - Intercepts blockquotes with callout markers (`> [!NOTE]`, `> [!WARNING]`).
   - Generates auditory cue prefixes: `"Important note: ..."` or `"Warning: ..."`.

## Execution Model

- Plugins execute inline during the AST traversal phase.
- Plugins must operate within microsecond execution boundaries to satisfy the overall <10 ms latency budget.
