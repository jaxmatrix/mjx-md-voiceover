# Capacity Planning Tables for Voiceover Throughput

Capacity reviews depend on accurate spoken summaries of GFM tables. The TablePlugin must announce column headers, row counts, and a capped preview of early rows without swallowing ordinary prose that happens to contain pipes.

## Regional Throughput

| Region | RPS Peak | p50 ms | p99 ms | Error % |
| --- | --- | --- | --- | --- |
| us-east-1 | 4200 | 0.6 | 3.1 | 0.02 |
| us-west-2 | 3100 | 0.7 | 3.4 | 0.03 |
| eu-west-1 | 2800 | 0.8 | 4.0 | 0.04 |
| ap-south-1 | 1900 | 0.9 | 4.6 | 0.05 |
| sa-east-1 | 900 | 1.1 | 5.2 | 0.06 |

## Plugin Contribution Matrix

| Plugin | Claims | Spoken Cue | Latency Budget |
| --- | --- | --- | --- |
| CodeBlockPlugin | fenced code | Code snippet in | < 50 us |
| LatexMathPlugin | dollar math | squared / fraction | < 80 us |
| MermaidPlugin | mermaid fences | diagram illustration | < 40 us |
| AdmonitionPlugin | [!NOTE] quotes | callout | < 30 us |
| TablePlugin | GFM tables | Table with columns | < 60 us |

## Non-Table Prose Guard

Operators sometimes write sentences such as choose red - blue | green for routing colors. That sentence must remain ordinary speech and must never collapse into “Structured data table.” Evaluation fails if the plugin matches on hyphen-plus-pipe heuristics inside Paragraph text nodes.

Keep spoken table previews limited to three body rows so long capacity sheets remain listenable.
