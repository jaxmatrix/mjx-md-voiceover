# Onboarding Tutorial: First Voiceover Integration

Welcome engineers joining the voiceover guild. This tutorial walks through converting Markdown locally, registering plugins correctly, and validating speech output before opening a pull request.

## Step One: Clone and Build

```bash
git clone https://github.com/mjx/mjx-md-voiceover.git
cd mjx-md-voiceover
cargo build --workspace
cargo test --workspace
```

## Step Two: Convert a Sample

Create a scratch Markdown file containing a heading, a Rust fence, a small math expression $a^2+b^2=c^2$, a note callout, a mermaid flowchart, and a three-column table. Convert it with the WASM demo or a unit test helper and read the speech aloud.

> [!TIP]
> Register MermaidPlugin before CodeBlockPlugin every time you build a custom registry.

## Step Three: Listen Critically

Ask whether any hash, asterisk, backtick, dollar delimiter, bang-note marker, or pipe grid survived. Ask whether table headers were named. Ask whether latency printed under ten milliseconds.

## Step Four: Expand Coverage

Add fixtures under `tests/dataset` for focused plugins and long documents under `eval_pairs/capability_corpus`. Keep each long document above one thousand characters so TTS duration differences are meaningful in dual-synthesis experiments.

When finished, open a PR whose description links the Plane AIPLUG work item and includes a short capability report excerpt.
