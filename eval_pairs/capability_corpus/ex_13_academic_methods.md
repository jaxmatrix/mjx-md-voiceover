# Methods Section: Evaluating Speech Fidelity of Markdown Transformers

We evaluate whether a Markdown-to-speech transformer removes syntax noise without erasing semantic content. The corpus mixes code, mathematics, admonitions, diagrams, and tables so each plugin pathway is exercised.

## Hypotheses

H1: Plugin-enabled conversion reduces TTS synthesis time versus raw Markdown. H2: Latency stays under ten milliseconds for documents near one to three kilobytes. H3: Structural table parsing yields spoken column names while hyphen-pipe prose remains intact.

## Materials

Documents are stored under `eval_pairs/capability_corpus`. Each file exceeds one thousand characters. Ground-truth expectations are qualitative cues rather than exact string matches, except for small plugin fixtures where exactness is practical.

Inline identity $E = mc^2$ and block forms appear together:

$$
F = ma
$$

## Procedure

1. Convert each document with the full plugin registry.
2. Record wall-clock latency, input length, output length, and readout ratios.
3. Score binary checks for forbidden raw tokens and required spoken cues.
4. Aggregate shard results into `CAPABILITY_EVAL_REPORT.md`.

## Analysis Sketch

```python
import json
from statistics import median
rows = json.load(open("eval_pairs/shards/shard1.json"))
lat = [r["latency_ms"] for r in rows["results"]]
print(median(lat), max(lat))
```

Failures must quote a short excerpt of offending speech text so maintainers can reproduce quickly.
