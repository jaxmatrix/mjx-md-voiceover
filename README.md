# mjx-md-voiceover

Ultra-fast, WASM-clean Markdown → voiceover speech engine in Rust.

`mjx-md-voiceover` parses Markdown into a zero-copy Voice AST and renders it as natural, speech-friendly prose — so a TTS engine says *"Heading: Title."* instead of *"hash Title"*, and *"Code snippet in Rust."* instead of reading out every brace and semicolon.

```text
Input:   # Title
         
         - Item 1
         - Item 2

Output:  "Heading: Title. Item 1. Item 2."
```

## Why

Feeding raw Markdown to a TTS model has two costs:

1. **Listenability** — the model reads syntax aloud: "hash", "asterisk asterisk", "greater than left bracket exclamation mark NOTE right bracket".
2. **Compute** — syntax noise inflates the character count the neural TTS model has to synthesize. In our [dual-TTS evaluation](eval_pairs/READOUT_EVALUATION_REPORT.md) with Kokoro-82M, pre-converting with `mjx-md-voiceover` cut TTS inference time by **+81.6%** on code-heavy documents and **+42.7% on average** across five domain datasets.

The parser itself runs in **50–340 µs** on real documents (10–28 MB/s throughput), so the conversion is effectively free next to the TTS inference it replaces.

## Features

- **Zero-copy Voice AST** — nodes borrow `&str` slices directly from the input buffer.
- **Sub-millisecond latency** — a CI-enforced SLA test asserts a 50 KB document parses and formats in under 10 ms (release builds land far under 1 ms).
- **WASM-clean core** — no DOM, no OS syscalls, no threads, no C bindings; builds for `wasm32-unknown-unknown`.
- **`#![deny(unsafe_code)]`** workspace-wide, typed `thiserror` errors, and a safety test suite that feeds malformed input (`"###"`, unclosed fences, dangling links) and asserts no panics.
- **Plugin system** — first-match-wins registry for domain-specific readouts: code blocks, LaTeX math, Mermaid diagrams, GitHub-style admonitions, tables.
- **Bindings** — WebAssembly (browsers, Node.js, Cloudflare Workers, Deno) via `wasm-bindgen`, and Python via PyO3/maturin.

## How it works

```text
&str  ──▶  VoiceAstParser  ──▶  VoiceAst<'a>  ──▶  SpeechFormatter  ──▶  String
           (pulldown-cmark        (zero-copy         + PluginRegistry
            event stream)          borrowed nodes)    (first-match-wins)
```

The parser wraps the `pulldown-cmark` event stream into a `VoiceAst` whose nodes borrow slices of the input. The formatter walks the tree, offering each node to the plugin registry before falling back to the built-in CommonMark rules, and writes into a single pre-allocated output buffer. See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the full design.

### Core conversion rules

| Markdown | Spoken output |
| --- | --- |
| `# Title` | `Heading: Title.` |
| `## Section` | `Section: Section.` |
| `**bold**` / `*italic*` | plain text (emphasis cue, no symbols) |
| `1. First item` | `First, First item.` (ordinals through Tenth, then "Next") |
| `> quoted text` | `Quote: quoted text.` |
| `---` | pause |

Full mapping table: [`docs/VOICEOVER_RULES.md`](docs/VOICEOVER_RULES.md).

### Plugins

| Plugin | Trigger | Example spoken output |
| --- | --- | --- |
| `CodeBlockPlugin` | fenced code block | `Code snippet in Rust.` / `SQL Schema Definition.` / `Shell command script snippet.` |
| `LatexMathPlugin` | `$…$`, `$$…$$` | `a squared plus b squared equals c squared` (`\sqrt` → "square root of", `\frac` → "fraction", greek letters, …) |
| `MermaidPlugin` | ` ```mermaid ` fence | `Architecture flowchart diagram.` / `Sequence flow diagram illustration.` |
| `AdmonitionPlugin` | `> [!NOTE]`-style callouts | `Note callout. …` / `Warning alert callout. …` |
| `TablePlugin` | pipe-and-dash text runs | `Structured data table.` |

Registration order matters: dispatch is first-match-wins, so `MermaidPlugin` must be registered before `CodeBlockPlugin` (which claims every fenced block). The WASM bindings register Mermaid → Code → LaTeX → Admonition and deliberately omit `TablePlugin`, whose broad trigger would swallow ordinary prose. Plugin authoring guide: [`docs/PLUGIN_SPEC.md`](docs/PLUGIN_SPEC.md).

## Workspace layout

```text
crates/
  mjx-md-voiceover-core/       parser, Voice AST, formatter, plugin trait + registry
  mjx-md-voiceover-plugins/    umbrella crate re-exporting all plugins
  plugins/
    mjx-md-voiceover-plugin-code/
    mjx-md-voiceover-plugin-latex/
    mjx-md-voiceover-plugin-mermaid/
    mjx-md-voiceover-plugin-admonition/
    mjx-md-voiceover-plugin-table/
  mjx-md-voiceover-wasm/       wasm-bindgen bindings (npm: @mjx/md-voiceover)
  mjx-md-voiceover-py/         PyO3 bindings (module: mjx_md_voiceover_py)
docs/                          architecture, voiceover rules, plugin spec, perf notes
eval_pairs/                    TTS evaluation dataset + readout report
app.py                         Streamlit dual-TTS latency & audio evaluator
```

## Usage

> The crates are not yet published to crates.io, npm, or PyPI — use a git or path dependency and build the bindings locally.

### Rust

```rust
use mjx_md_voiceover_core::parse_and_format;

let speech = parse_and_format("# Title\n\n- Item 1\n- Item 2")?;
assert_eq!(speech, "Heading: Title. Item 1. Item 2.");
```

For plugin-aware formatting, build a registry and use `SpeechFormatter::format_with_registry`:

```rust
use mjx_md_voiceover_core::{PluginRegistry, SpeechFormatter, VoiceAstParser};
use mjx_md_voiceover_plugins::{CodeBlockPlugin, MermaidPlugin};

let mut registry = PluginRegistry::new();
registry.register(MermaidPlugin::new()); // before CodeBlockPlugin — first match wins
registry.register(CodeBlockPlugin::new());

let ast = VoiceAstParser::parse("```rust\nfn main() {}\n```")?;
let speech = SpeechFormatter::format_with_registry(&ast, &registry);
assert_eq!(speech, "Code snippet in Rust.");
```

### JavaScript / TypeScript (WASM)

Build with [wasm-pack](https://rustwasm.github.io/wasm-pack/):

```sh
wasm-pack build crates/mjx-md-voiceover-wasm
```

```ts
import { convert_markdown_to_voiceover, convert_markdown_core_only, parse_markdown_ast_json } from "@mjx/md-voiceover";

convert_markdown_to_voiceover("# Title\n\nHello WASM!"); // "Heading: Title. Hello WASM!"
convert_markdown_core_only("```rust\nfn main() {}\n```"); // bare CommonMark rules, no plugins
parse_markdown_ast_json("# Test");                        // JSON of the Voice AST
```

### Python

Build with [maturin](https://www.maturin.rs/):

```sh
cd crates/mjx-md-voiceover-py
maturin develop
```

```python
import mjx_md_voiceover_py as voiceover

voiceover.convert_markdown_to_voiceover("# Title\n\n- Item 1")  # "Heading: Title. Item 1."
voiceover.parse_markdown_ast_json("# Title")                     # Voice AST as JSON
```

Note: the Python binding currently uses the core CommonMark rules only (no plugins).

## Development

```sh
cargo fmt --all
cargo build  --workspace
cargo test   --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo bench  -p mjx-md-voiceover-core
cargo build  -p mjx-md-voiceover-core --target wasm32-unknown-unknown
```

CI runs formatting, checks, tests, clippy (warnings denied), and WASM-target checks on every push and PR to `main`. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the workflow and commit conventions.

### Benchmarks & evaluation

Criterion benchmarks cover small/medium documents and plugin-heavy dataset files (`cargo bench -p mjx-md-voiceover-core`). Parser-side numbers are in [`docs/ABSTRACTION_PERFORMANCE_NOTE.md`](docs/ABSTRACTION_PERFORMANCE_NOTE.md).

The end-to-end TTS evaluation harness compares raw-Markdown vs parsed-voiceover synthesis with Kokoro-82M:

```sh
streamlit run app.py                       # side-by-side audio + latency dashboard
python scripts/generate_audio_kokoro.py    # regenerate WAVs + timing data
```

(Python deps: `streamlit`, `kokoro`, `torch`, `soundfile`, `numpy`.) Results: [`eval_pairs/READOUT_EVALUATION_REPORT.md`](eval_pairs/READOUT_EVALUATION_REPORT.md).

## Roadmap

- SIMD/`memchr`-based scanning in hot paths
- O(1) plugin dispatch keyed on node discriminant (replacing linear first-match)
- Depth-aware ordinal prefixes for nested lists
- Zero-copy output variant for the WASM boundary (`convert_markdown_to_voiceover_into_buffer`)

## License

Dual-licensed under MIT OR Apache-2.0.
