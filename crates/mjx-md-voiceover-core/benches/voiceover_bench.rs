use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mjx_md_voiceover_core::{parse_and_format, VoiceAstParser};

const SMALL_DOC: &str = "# Overview\n\nThis is a short Markdown sentence for voice conversion.";

const MEDIUM_DOC: &str = r#"
# System Architecture Specification

## Introduction
`mjx-md-voiceover` converts formatted Markdown text into continuous, natural speech text optimized for Text-to-Speech (TTS) audio synthesis and voice AI agents.

### Core Features
- **Ultra-Low Latency (<1-10 ms SLA):** Single-pass AST parsing and token emission.
- **Strict WASM Safety:** Pure Rust compilation to `wasm32-unknown-unknown`.
- **Plugin System:** Modular handlers for specialized syntax tokens.

> "Speech-first, Pure-Rust, WASM-compliant, Sub-10ms."

- Item 1: Parse AST
- Item 2: Format Speech
- Item 3: Synthesize Audio
"#;

const LARGE_DOC: &str = r#"
# Extensive Markdown Voiceover Benchmark Document

## Section 1: Overview
Voice agents and Text-to-Speech (TTS) models struggle when fed raw Markdown: they read literal symbols such as "hash hash hash section title", "asterisk asterisk bold text asterisk asterisk", or code syntax noise.

## Section 2: Code Snippet Example
```rust
fn main() {
    println!("Hello, voice agent!");
}
```

## Section 3: Lists & Quotes
1. First, parse CommonMark tokens into a zero-copy AST.
2. Second, apply speech formatting rules and pacing pause injection.
3. Third, render clean spoken prose for TTS.

> [!NOTE]
> Performance budget must remain strictly under 1-10 ms for all document sizes.

Check out the [documentation](https://github.com/mjx/mjx-md-voiceover) for details.
"#;

fn bench_small_doc(c: &mut Criterion) {
    c.bench_function("parse_and_format_small_100b", |b| {
        b.iter(|| parse_and_format(black_box(SMALL_DOC)))
    });
}

fn bench_medium_doc(c: &mut Criterion) {
    c.bench_function("parse_and_format_medium_2kb", |b| {
        b.iter(|| parse_and_format(black_box(MEDIUM_DOC)))
    });
}

fn bench_large_doc(c: &mut Criterion) {
    c.bench_function("parse_and_format_large_10kb", |b| {
        b.iter(|| parse_and_format(black_box(LARGE_DOC)))
    });
}

fn bench_parser_only(c: &mut Criterion) {
    c.bench_function("parser_only_medium_2kb", |b| {
        b.iter(|| VoiceAstParser::parse(black_box(MEDIUM_DOC)))
    });
}

criterion_group!(
    benches,
    bench_small_doc,
    bench_medium_doc,
    bench_large_doc,
    bench_parser_only
);
criterion_main!(benches);
