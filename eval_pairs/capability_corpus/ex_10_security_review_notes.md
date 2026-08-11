# Security Review Notes for Markdown Voiceover Inputs

All Markdown inputs are untrusted. Parsers and plugins must never panic on malformed fences, unterminated math, or hostile table layouts. This note captures review expectations for auditors.

## Threat Themes

Attackers may submit extremely nested lists, megabyte-scale fences, or tables with thousands of columns to amplify allocation costs. The service should fail closed with typed errors rather than aborting the process. `unsafe_code` remains denied workspace-wide.

## Safe Patterns

```rust
pub fn parse_and_format(markdown: &str) -> Result<String> {
    let ast = VoiceAstParser::parse(markdown)?;
    Ok(SpeechFormatter::format(&ast))
}
```

Prefer `thiserror` variants over stringly errors at API boundaries. Do not call `unwrap` or `expect` on user Markdown. Benchmarks that allocate heavily in plugins should be treated as performance defects even if functional tests pass.

## Table-Specific Risks

A plugin that matches any text containing `|` and `-` can censor legitimate prose and hide secrets that happen to include those characters by replacing the entire paragraph with a fixed summary. Correct matching is structural: only AST table nodes. Evaluation corpora intentionally include hyphen-pipe prose traps beside real GFM tables.

## Residual Acceptance

Auditors accept `Box::leak` for short-lived request arenas in WASM isolates where arena reset is process-lifetime, matching existing LaTeX plugin practice, provided leaks remain proportional to conversion output size and not to attacker-controlled quadratic expansions.
