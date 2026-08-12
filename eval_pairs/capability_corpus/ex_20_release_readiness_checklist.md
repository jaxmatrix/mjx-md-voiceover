# Release Readiness Checklist for mjx-md-voiceover 0.1.x

This checklist is the final long-form evaluation input. Completing it aloud should feel like a release manager briefing, not a reading of punctuation soup.

## Engineering Gates

- [x] Workspace builds with clippy warnings denied
- [x] Core benches exist for sub-ten-millisecond targets
- [ ] Table AST frames merged and tested
- [ ] WASM registry includes TablePlugin safely
- [ ] Capability corpus of twenty documents evaluated in parallel shards
- [ ] CAPABILITY_EVAL_REPORT.md published under eval_pairs

## Binding Matrix

| Binding | Entry Point | Plugins |
| --- | --- | --- |
| Native core | SpeechFormatter::format_with_registry | caller-supplied |
| WASM | convert_markdown_to_voiceover | default registry |
| Python | convert_markdown_to_voiceover | currently core-only unless updated |

## Final Smoke Commands

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p mjx-md-voiceover-core --test dataset_eval_test -- --nocapture
```

> [!CAUTION]
> Do not mark Plane AIPLUG-3 done until table false-positive tests pass and a sample GFM table speaks column names.

Release managers should confirm Mermaid still wins over Code, LaTeX still spares currency, admonitions still cue, and tables announce columns. Only then should the version tag move forward.
