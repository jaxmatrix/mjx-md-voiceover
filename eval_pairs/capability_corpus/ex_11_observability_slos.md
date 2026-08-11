# Observability and SLO Handbook for Speech Conversion

Service level objectives focus on conversion latency, speech fidelity, and error rate. This handbook is long-form Markdown intentionally dense so evaluation agents can judge pacing across headings, tables, and code.

## Latency SLO

| Window | Target | Page Threshold |
| --- | --- | --- |
| Rolling 1 hour | p99 < 5 ms | p99 > 10 ms for 10 minutes |
| Rolling 24 hour | p95 < 2 ms | burn rate > 2x for 1 hour |
| Release candidate | max < 10 ms on corpus | any weak case fails gate |

## Fidelity Signals

Reviewers listen for absences of hash characters before headings, asterisk runs around emphasis, backtick fences, raw dollar math delimiters, admonition bang markers, and pipe grids. Presence of cues such as “Heading:”, “Code snippet in Rust.”, “Note callout.”, “Architecture flowchart diagram.”, and “Table with columns” indicates plugins fired.

## Example Probe

```bash
for f in eval_pairs/capability_corpus/ex_*.md; do
  cargo run -q -p mjx-md-voiceover-core --example convert -- "$f" || true
done
```

If probes are unavailable, prefer `cargo test -p mjx-md-voiceover-core --test dataset_eval_test -- --nocapture` and capture printed latencies. Store shard JSON under `eval_pairs/shards` when parallel agents evaluate subsets of the corpus.
