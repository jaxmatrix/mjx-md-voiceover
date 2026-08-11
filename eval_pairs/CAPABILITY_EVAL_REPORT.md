# Capability Evaluation Report

**Branch:** `feature/AIPLUG-3-table-plugin`  
**Engine:** mjx-md-voiceover (full plugin registry: Mermaid → Code → Latex → Admonition → Table)  
**Corpus:** 20 long documents (>1000 chars each) + 5 per-plugin fixtures  
**Method:** 3 parallel shard runners via `eval_pairs/tools`

## Overall

| Metric | Value |
| --- | --- |
| Files evaluated | 25 |
| Passed | 23 |
| Failed | 2 |
| Latency p50 (ms) | 0.0610 |
| Latency p99 (ms) | 0.2700 |
| Max latency (ms) | 0.2700 |
| Release budget | < 10 ms (flag weak if exceeded) |
| Debug harness allowance | < 25 ms |

## Per-shard

- **Shard 1:** passed=7 failed=2 files=9
- **Shard 2:** passed=9 failed=0 files=9
- **Shard 3:** passed=7 failed=0 files=7

## Failures

### `ex_03_rust_service_architecture`
- failures: ['contains raw [!NOTE]']
- latency_ms: 0.146
- preview: `Heading: Voiceover Core Service Architecture Brief. The mjx-md-voiceover-core crate owns CommonMark parsing, speech formatting, and the plugin trait. Downstream WASM and Python bindings must never rei`

### `plugin_latex`
- failures: ['plugin_latex: raw math with ^/_/\\ still inside $...$']
- latency_ms: 0.04
- preview: `Heading: LaTeX Math Speechifier Fixture. Inline Pythagorean identity: a squared plus b squared equals c squared. Block Schrödinger form:. $$. -\frac{\hbar^2}{2m} \nabla^2 \Psi + V\Psi = E\Psi $$. Also`


## Failure analysis

1. **`ex_03_rust_service_architecture`** — likely a **false positive**. The source document discusses `[!NOTE]` markers in prose as a failure-mode description; the judge flagged the literal token even though it is not an admonition block. Latency was fine (0.15 ms).
2. **`plugin_latex`** — **real gap**: inline `$…$` verbalizes correctly (`a squared plus b squared…`), but **standalone `$$` block paragraphs** still leave `$$` / `\frac` / `\hbar` in the speech stream. Follow-up: teach LatexMathPlugin (or the parser) to claim block-math text nodes that are mostly delimited formulas.

All 25 conversions finished well under the 10 ms release budget (p99 ≈ 0.27 ms).

## Notes

- TablePlugin now speaks GFM tables as `Table with columns …` with a 3-row preview; prose traps with `|` / `-` stay as prose.
- Plugin fixtures live under `eval_pairs/plugin_fixtures/` and `tests/dataset/plugin_*.md`.
- Long corpus: `eval_pairs/capability_corpus/ex_01` … `ex_20` (each >1000 characters).
- `dataset_eval_pairs.json` regenerated with full plugin set (10 dataset files including plugin fixtures).
- Plane: AIPLUG-3 (table) and AIPLUG-4…7 (admonition/mermaid/code/latex) marked Done.
