# Per-plugin expected cues (capability judging)

Use these qualitative checks when scoring plugin fixtures and long corpus docs.

| Fixture | Required cue fragments (case-insensitive OK) | Forbidden raw tokens |
| --- | --- | --- |
| `plugin_code.md` | `Code snippet in Rust`, `Code snippet in Python`, `Shell command`, `SQL Schema` | triple backticks, language fence markers as syntax |
| `plugin_latex.md` | `squared`, `fraction` or spoken Schrödinger terms; currency `$5` preserved | `$a^2`, `$$`, `\frac` as raw TeX |
| `plugin_admonition.md` | `Note callout` / `Warning callout` / tip/important/caution cues | `[!NOTE]`, `[!WARNING]` |
| `plugin_mermaid.md` | `flowchart` or `Architecture`, `Sequence`, `class` diagram cue | raw `graph TD`, `-->` noise |
| `plugin_table.md` | `Table with columns` listing Service/Latency/Status (or similar) | pipe-grid `| --- |`; prose trap must NOT become only `Structured data table.` |

## Latency

Release: &lt; 10 ms per document. Debug tests may allow 25 ms (existing harness).

## Registry order for generators / eval harness

1. `MermaidPlugin`
2. `CodeBlockPlugin`
3. `LatexMathPlugin`
4. `AdmonitionPlugin`
5. `TablePlugin`
