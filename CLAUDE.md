# CLAUDE.md — guidance for AI agents working in mjx-md-voiceover

This file orients Claude Code (and any coding agent) working in this repo. Humans: see `README.md`, `PLAN.md`, and `CONTRIBUTING.md`.

## What this project is

A pure-Rust, cross-platform library and WASM engine to parse Markdown into an AST and transform syntax tokens into **speech-friendly, natural conversational voiceover text**.

Voice agents and Text-to-Speech (TTS) models struggle when fed raw Markdown: they read literal symbols (e.g. "hash hash hash section title", "asterisk asterisk bold text asterisk asterisk", or code syntax noise). `mjx-md-voiceover` converts Markdown structures into clear, natural spoken prose with conversational cues and pacing pauses.

## Core Non-Negotiable Requirements

1. **Strict Performance Budget (< 1-10 ms):** The entire pipeline (Markdown string → AST parse → plugin token transformations → speech text generation) **MUST complete within 1-10 ms** (aiming for sub-millisecond <1 ms for standard inputs). Any pipeline benchmark taking >10 ms is classified as **weak** and unacceptable.
2. **WASM Compatibility:** The core parser and speech transformer MUST target `wasm32-unknown-unknown` without relying on browser DOM, node APIs, C-bindings, thread primitives, or OS sys-calls.
3. **Plugin System:** Core library handles pure CommonMark / standard Markdown. Plugin hooks allow dynamic injection/transformation for specialized syntax like Code blocks (language-aware verbalizer), LaTeX math (plain English equation readouts), Tables, and Callouts.
4. **Cross-Platform Bindings:** Expose native FFI bindings for **Python** (`pyo3`) and **TypeScript / Web / Node.js** (`wasm-bindgen`).

## How we work here (non-negotiable process)

Every unit of work follows: **Plan → Plan-Optimization → thorough atomic implementation.**

1. **Plan** the atomic piece of work.
2. **Plan-Optimization** — *before writing code*, refine the design for **low memory footprint, sub-millisecond execution speed, and WASM compatibility**. Weigh heap allocations, string cloning, slice references, and cache locality. **No shortcuts or monkey-patching**.
3. **Thorough atomic implementation** — finish the piece *completely, correctly, with unit/bench tests* before moving on.
4. **Discussion-first** — begin each working session by discussing the plan for what we're about to implement.

## Architecture & Crate Hierarchy

Dependencies point **downward only**:

- `mjx-md-voiceover-core`: CommonMark AST parser, voice token transformer, core plugin trait, speech formatter, benchmarking harness.
- `mjx-md-voiceover-plugins`: Collection of standard plugins (Code verbalizer, LaTeX math speechifier, Admonitions/Callouts handler).
- `mjx-md-voiceover-wasm`: WebAssembly binding layer for Web browsers, Edge runtimes, and Node.js.
- `mjx-md-voiceover-py`: Python C-extension binding layer built with PyO3 / Maturin.

## Design Rules & Safety Constraints

- **`unsafe_code = "deny"`** workspace-wide; any local exception must have an explicit written safety justification.
- **Pure Rust only** — zero native C or C++ external library dependencies.
- **Zero-allocation or minimal single-pass allocation:** Use slice references (`&str`), small-vec / stack buffers, and streaming string buffer writes during voice text rendering.
- **No `unwrap`/`panic`/`expect` on untrusted input** — all inputs are untrusted Markdown strings. Return typed `thiserror` errors.

## Commands

```sh
cargo build  --workspace
cargo test   --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo bench  -p mjx-md-voiceover-core     # Verify performance (<10ms target)
```

## Git / Commits

- **Project-setup commits go on `main`;** feature development uses **branch per feature + PR**.
- **Atomic commits** — one self-contained change, green build & green test before committing.
- **Do NOT add `Co-Authored-By` or AI-attribution trailers** to commit messages.
