use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mjx_md_voiceover_core::{parse_and_format, PluginRegistry, SpeechFormatter, VoiceAstParser};
use mjx_md_voiceover_plugins::{AdmonitionPlugin, CodeBlockPlugin, LatexMathPlugin};

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

const CODE_HEAVY_DOC: &str = include_str!("../tests/dataset/code_heavy.md");
const MATH_HEAVY_DOC: &str = include_str!("../tests/dataset/math_heavy.md");
const ADMONITION_DOC: &str = include_str!("../tests/dataset/admonitions_mixed.md");

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

fn bench_code_heavy_doc(c: &mut Criterion) {
    let mut registry = PluginRegistry::new();
    registry.register(CodeBlockPlugin::new());
    c.bench_function("dataset_code_heavy_with_plugin", |b| {
        b.iter(|| {
            let ast = VoiceAstParser::parse(black_box(CODE_HEAVY_DOC)).unwrap();
            SpeechFormatter::format_with_registry(&ast, &registry)
        })
    });
}

fn bench_math_heavy_doc(c: &mut Criterion) {
    let mut registry = PluginRegistry::new();
    registry.register(LatexMathPlugin::new());
    c.bench_function("dataset_math_heavy_with_plugin", |b| {
        b.iter(|| {
            let ast = VoiceAstParser::parse(black_box(MATH_HEAVY_DOC)).unwrap();
            SpeechFormatter::format_with_registry(&ast, &registry)
        })
    });
}

fn bench_admonition_doc(c: &mut Criterion) {
    let mut registry = PluginRegistry::new();
    registry.register(AdmonitionPlugin::new());
    c.bench_function("dataset_admonition_with_plugin", |b| {
        b.iter(|| {
            let ast = VoiceAstParser::parse(black_box(ADMONITION_DOC)).unwrap();
            SpeechFormatter::format_with_registry(&ast, &registry)
        })
    });
}

criterion_group!(
    benches,
    bench_small_doc,
    bench_medium_doc,
    bench_code_heavy_doc,
    bench_math_heavy_doc,
    bench_admonition_doc
);
criterion_main!(benches);
