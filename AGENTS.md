# AGENTS.md

Vendor-neutral entry point for coding agents working in **mjx-md-voiceover**.

The full, authoritative guidance lives in:

- [`CLAUDE.md`](CLAUDE.md) — architecture rules, performance budget (<1-10 ms), WASM constraints, plugin architecture, FFI bindings, process guidelines, and commands.
- [`CONTRIBUTING.md`](CONTRIBUTING.md) — test-driven workflow, speech fidelity verification tiers, and git/commit conventions.
- [`PLAN.md`](PLAN.md) — the phased development roadmap and current status.
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — deep dive into AST parsing, speech synthesis token conversion, and plugin pipeline mechanics.

## The short version

- **Speech-first, Pure-Rust, WASM-compliant, Sub-10ms.** Convert Markdown syntax noise (e.g. `###`, `**`, `code`, `$math$`) into natural human speech text so TTS voice agents sound conversational instead of reading syntax symbols verbatim.
- **Strict Performance Budget:** Total parsing and voice text conversion MUST execute within **1-10 ms** for typical documents (target sub-millisecond <1ms). Implementations exceeding 10 ms are marked as **weak**.
- **WASM Compatibility is Mandatory:** Every core crate must compile cleanly to `wasm32-unknown-unknown`. No system I/O, no C/native library dependencies, no thread-local OS primitives in `core`.
- **Plugin System Extensibility:** Core handles standard CommonMark syntax. Extensible plugin ecosystem manages specialized domain tokens (Code execution/explanation, LaTeX math verbalization, Callouts/Admonitions, Tables).
- **Multi-platform FFI Bindings:** First-class bindings for TypeScript/Node/Web (via WASM / `wasm-bindgen`) and Python (via PyO3 / Maturin).
- **Layering points downward only:** `mjx-md-voiceover-core` → `mjx-md-voiceover-plugins` → `mjx-md-voiceover-wasm` / `mjx-md-voiceover-py`.
- **Test-driven & incremental:** Write failing speech-fidelity and latency tests first; keep every increment green.
- **Atomic commits, no `Co-Authored-By`/AI-attribution trailers:** Project setup on `main`, then feature branches + PRs.
- **Do the work thoroughly and correctly — no monkey-patching.** Optimize allocations, cache locality, and string builder patterns *before* coding.
