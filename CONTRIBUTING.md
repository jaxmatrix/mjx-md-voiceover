# Contributing to mjx-md-voiceover

This project is built **deliberately, test-first, performance-first, and incrementally**. The project grows in small, always-green, benchmarked steps.

## The development loop (TDD + BDD)

Every change follows **red → green → refactor & benchmark**:

1. **Red** — write a failing speech transformation test or benchmark test first. Assert that Markdown syntax token `### Header` produces natural speech text like `"Header."` or `"Section: Header."` rather than literal markdown punctuation.
2. **Green** — write the minimum AST transformation / plugin code to pass.
3. **Refactor & Benchmark** — optimize string buffer allocations to keep runtime **strictly under 1-10 ms**.

Before writing code for a non-trivial piece, do the **Plan → Plan-Optimization** step: optimize memory footprint, string allocations, and iteration speed *first*.

## Speech Fidelity & Latency Tiers

1. **Standard Markdown Tier:** CommonMark elements (Headers, Paragraphs, Lists, Quotes, Links, Bold, Italic, Code spans) must convert to fluent, human-sounding voiceover text without reading syntax symbols verbatim.
2. **Plugin Extensibility Tier:** Non-standard or heavy syntax (Code blocks, LaTeX formulas, Tables, Callouts) are handed off to plugins.
3. **Latency SLA Tier:** Every parser pass must process representative document inputs in **< 1-10 ms**. Benchmarks measure microsecond throughput.

## Required checks before committing

```sh
cargo fmt --all
cargo build  --workspace
cargo test   --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Cross-target verification:
```sh
cargo build -p mjx-md-voiceover-core --target wasm32-unknown-unknown
```

## Git & Commit Conventions

- **Atomic commits** — one logical change per commit.
- **No `Co-Authored-By` or AI-attribution trailers.** Plain imperative messages (e.g. `feat(core): implement AST speech emission`, `perf(plugin): optimize code block verbalizer`).
- **Branching:** project-setup commits on `main`, feature branches + PRs for features.
