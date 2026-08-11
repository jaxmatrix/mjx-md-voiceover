# Performance Regression Diary — August 2026

This diary captures narrative context around latency spikes observed while hardening table support. It intentionally includes code, tables, and callouts so conversion stress remains realistic.

## Timeline

| Time UTC | Observation | Action |
| --- | --- | --- |
| 14:05 | p99 rose to 12 ms on math_heavy | Checked Latex allocations |
| 14:22 | Mermaid masked by Code plugin in a fork | Fixed registry order |
| 15:10 | Table plugin false-positive on prose | Removed text heuristic |
| 16:40 | ENABLE_TABLES without frames | Implemented Table AST |

> [!WARNING]
> Do not ship TablePlugin text heuristics even as a temporary workaround; they corrupt unrelated paragraphs.

## Reproduction Snippet

```rust
let md = "choose red - blue | green\n\n| A | B |\n| --- | --- |\n| 1 | 2 |\n";
let ast = VoiceAstParser::parse(md).unwrap();
// Expect both a paragraph and a Table node, not a single CustomSpeech collapse.
```

## Lessons

Structural parsing beats string heuristics. First-match plugin order is part of the public behavior contract. Evaluation corpora must include trap sentences beside genuine tables. Release gates should fail on any corpus member exceeding ten milliseconds in release mode, while debug builds may allow a slightly higher threshold as existing tests already document.
