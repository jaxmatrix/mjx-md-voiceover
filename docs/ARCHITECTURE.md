# Architecture Specification: mjx-md-voiceover

## System Vision

`mjx-md-voiceover` transforms formatted Markdown text into continuous, natural speech text optimized for Text-to-Speech (TTS) audio synthesis and voice AI agents.

```
+------------------+     +--------------------------+     +----------------------------+
| Raw Markdown In  | --> | mjx-md-voiceover-core    | --> | Plugin Registry            |
| (Untrusted &     |     | - CommonMark Tokenizer   |     | - Code Spoken Summarizer   |
|  Unstructured)   |     | - Voice AST Generator    |     | - LaTeX Math Speechifier   |
+------------------+     +--------------------------+     | - Callouts / Admonitions   |
                                                          +----------------------------+
                                                                        |
                                                                        v
+------------------+     +--------------------------+     +----------------------------+
| Natural Speech   | <-- | Speech Text Renderer     | <-- | Voice AST Stream           |
| Output String    |     | - Pacing / Pause Injection|     | (Enhanced & Transformed)   |
+------------------+     +--------------------------+     +----------------------------+
```

## Key Architectural Principles

1. **Ultra-Low Latency (<1-10 ms SLA):**
   - Single-pass AST parsing and token emission using zero-copy slice references (`&str`).
   - String buffer pre-allocation based on input length estimation to minimize dynamic reallocation.
   - Microsecond execution target for standard documents (<1 ms target).

2. **Strict WASM Safety (`wasm32-unknown-unknown`):**
   - Absolutely no OS sys-calls, non-WASM multi-threading, file I/O, or native C library dependencies in `core` or `plugins`.
   - Guaranteed compilation for Web, Node.js, Deno, Bun, and Cloudflare Workers.

3. **Core vs. Plugin Separation:**
   - **Core**: Focuses strictly on base CommonMark syntax (Headings, Paragraphs, Lists, Emphasis, Blockquotes, Links, Inline Code).
   - **Plugins**: Modular handlers for specialized syntax tokens (Fenced code blocks, LaTeX math `$`, Tables, Callout alerts `> [!NOTE]`).

4. **Multi-Language Bindings (FFI):**
   - **TypeScript / JavaScript**: `wasm-bindgen` interface returning typed speech output and options.
   - **Python**: PyO3 / Maturin extension module exposing native Python bindings.

## Crate Layout

- `crates/mjx-md-voiceover-core`: Parser engine, AST types, Speech formatter, Core plugin traits.
- `crates/mjx-md-voiceover-plugins`: Official plugins (Code, Math, Admonition).
- `crates/mjx-md-voiceover-wasm`: WebAssembly interface layer.
- `crates/mjx-md-voiceover-py`: Python FFI layer.
