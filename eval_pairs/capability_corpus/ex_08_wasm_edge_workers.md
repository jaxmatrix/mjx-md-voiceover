# Deploying Voiceover WASM on Edge Workers

Edge runtimes need a tiny `wasm32-unknown-unknown` artifact with no filesystem or thread primitives. The WASM crate registers Mermaid before Code so diagram fences are not masked, and after Table hardening it must also register TablePlugin.

## Build Commands

```bash
rustup target add wasm32-unknown-unknown
cargo build -p mjx-md-voiceover-wasm --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir pkg ./target/wasm32-unknown-unknown/release/mjx_md_voiceover_wasm.wasm
```

## JavaScript Smoke Test

```javascript
import init, { convert_markdown_to_voiceover } from "./pkg/mjx_md_voiceover_wasm.js";
await init();
const md = "# Hello\n\n| A | B |\n| --- | --- |\n| 1 | 2 |\n";
const speech = convert_markdown_to_voiceover(md);
console.log(speech);
```

## Operational Constraints

Workers must keep CPU time well below isolate limits. Sub-millisecond conversion leaves ample budget for network and TTS. Avoid registering the historical TablePlugin text heuristic because ordinary marketing copy containing pipes would become “Structured data table.” Only structured `VoiceAstNode::Table` nodes are safe to claim.

Document registry order in code comments whenever a catch-all plugin could mask a specialized one. Continuous evaluation on long corpora at the edge should sample at least twenty documents weekly.
