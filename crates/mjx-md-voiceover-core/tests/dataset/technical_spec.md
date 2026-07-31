# Ultra-Fast Voiceover Engine Specification

The `mjx-md-voiceover` project provides an **ultra-fast**, **WASM-compliant** engine converting Markdown syntax noise into clear, natural human speech.

## System Overview

Standard TTS engines read syntax symbols like `###`, `**`, or `$` literally, creating jarring audio experiences. `mjx-md-voiceover` resolves this by transforming raw CommonMark syntax into smooth spoken prose.

### Architectural Rules

1. **Sub-10ms SLA:** Parsing and conversion MUST complete in $< 10\text{ ms}$ (target $< 1\text{ ms}$).
2. **WASM First:** Target `wasm32-unknown-unknown` without OS I/O or browser DOM dependencies.
3. **Thread Safety:** All plugins implement `Send + Sync`.

> [!NOTE]
> All plugin transformations are applied during AST traversal before speech string formatting.

For more information, visit the [Documentation Site](https://mjx.dev/docs).
