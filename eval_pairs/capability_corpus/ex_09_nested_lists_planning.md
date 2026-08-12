# Comprehensive Delivery Breakdown for Plugin Hardening

## Phase 1: Planning and Setup

1. Requirements Gathering
   - Interview TTS reviewers about unlistenable syntax noise
   - Define benchmark targets under ten milliseconds
   - Capture speech fidelity tiers in CONTRIBUTING.md
2. Architecture Design
   - Extend VoiceAst with Table frames
   - Keep plugin registration first-match-wins
   - Preserve WASM and Python binding thinness
3. Stakeholder Alignment
   - Track work in Plane project AIPLUG
   - Keep Linear identifiers in commit subjects when relevant

## Phase 2: Implementation Steps

* Core Engine
  * Parse GFM table events into headers and rows
  * Emit natural language formatter fallbacks
  * Maintain zero-copy slices where possible
* Extensible Plugins
  * Code block verbalization
  * LaTeX math formula conversion
  * Callout box auditory alerts
  * Mermaid diagram overviews
  * Table spoken summaries with three-row caps
* Cross-Platform Bindings
  * WebAssembly wasm-bindgen package
  * Python PyO3 and maturin wheels

## Task Checklists

- [x] Create workspace Cargo.toml
- [x] Build core speech parser
- [x] Add GitHub Actions CI
- [ ] Complete table parser frames
- [ ] Expand capability corpus to twenty long documents
- [ ] Publish capability evaluation report

Spoken lists should use ordinal cues for ordered items and avoid reading raw dashes or checkbox brackets.
