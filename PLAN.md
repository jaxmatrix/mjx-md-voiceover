# PLAN.md — Development Roadmap for mjx-md-voiceover

## Overview

`mjx-md-voiceover` is an ultra-fast, WASM-compliant, cross-platform Rust engine designed to parse Markdown into an AST and transform syntax tokens into natural, human-friendly speech text for voice agents and Text-to-Speech (TTS) applications.

## Phased Development Roadmap

### Phase 1: Project Setup, Architecture & Documentation (Current Phase)
- [x] Initialize Git repository & workspace manifest (`Cargo.toml`).
- [x] Adapt `AGENTS.md`, `CLAUDE.md`, and `CONTRIBUTING.md` from `mjx-ooxml-rs`.
- [x] Document system architecture (`docs/ARCHITECTURE.md`), plugin spec (`docs/PLUGIN_SPEC.md`), and voiceover translation guidelines (`docs/VOICEOVER_RULES.md`).
- [x] Establish strict sub-10ms performance budget and WASM compatibility guarantees.
- [x] Align with project stakeholders on development plan before code implementation.

### Phase 2: Core AST Parser & Voice Text Generator (`mjx-md-voiceover-core`)
- [ ] Define Markdown Voice AST node hierarchy (`VoiceAst`, `VoiceNode`, `SpeechToken`).
- [ ] Implement zero-copy tokenization / AST parsing over CommonMark stream.
- [ ] Build default `SpeechFormatter` converting standard AST nodes to natural speech strings:
  - Headings (`#`, `##`, etc.) -> Conversational section pauses / announcements.
  - Lists (`-`, `1.`) -> Sequential item speech phrasing.
  - Emphasis (`**`, `*`) -> Rhythm & cadence formatting without reading asterisks.
  - Links (`[text](url)`) -> Spoken link phrasing (e.g. "text, link target: ...").
  - Blockquotes (`>`) -> Quote introduction ("Quote: ...").
- [ ] Implement benchmark suite (`criterion` / microsecond timing assertions <10 ms).

### Phase 3: Plugin Ecosystem & Registry (`mjx-md-voiceover-plugins`)
- [ ] Design `VoicePlugin` trait with lifecycle hooks: `on_node_parse`, `transform_token`, `render_speech`.
- [ ] Implement `CodeBlockPlugin`: converts code fences into spoken code descriptions or friendly language summaries.
- [ ] Implement `LatexMathPlugin`: converts `$ ... $` and `$$ ... $$` math blocks into plain English speech expressions.
- [ ] Implement `AdmonitionPlugin`: converts callout boxes (`> [!NOTE]`, `> [!WARNING]`) into clear auditory alerts.

### Phase 4: WebAssembly & TypeScript Bindings (`mjx-md-voiceover-wasm`)
- [ ] Implement `wasm-bindgen` interface for JS/TS environments.
- [ ] Ensure full target support for `wasm32-unknown-unknown` (Browsers, Node.js, Cloudflare Workers, Edge runtimes).
- [ ] Build npm package structure & TypeScript definition files (`.d.ts`).

### Phase 5: Python Bindings & Maturin Build (`mjx-md-voiceover-py`)
- [ ] Implement PyO3 wrapper exposing `parse_and_convert(markdown, plugins)` to Python.
- [ ] Configure `maturin` build system for PyPI wheel generation across Linux, macOS, Windows.

### Phase 6: Performance Optimization & SLA Auditing
- [ ] Run benchmark suites against large, complex Markdown documents.
- [ ] Validate < 1-10 ms SLA across all targets (native, WASM, Python).
- [ ] Audit heap allocations and enforce zero/single-pass string buffer generation.
