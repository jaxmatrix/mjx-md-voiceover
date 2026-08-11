# Python Batch Conversion Pipeline Guide

Data science teams convert large Markdown corpora offline before feeding Kokoro. The PyO3 module exposes convert helpers that should eventually register the same default plugin set as WASM.

## Installation

```bash
pip install mjx-md-voiceover
maturin develop -m crates/mjx-md-voiceover-py/Cargo.toml
pytest -q
```

## Example Driver

```python
from pathlib import Path
from mjx_md_voiceover_py import convert_markdown_to_voiceover

corpus = Path("eval_pairs/capability_corpus")
for path in sorted(corpus.glob("ex_*.md")):
    text = path.read_text(encoding="utf-8")
    speech = convert_markdown_to_voiceover(text)
    assert len(speech) > 0
    assert "```" not in speech
    print(path.name, len(text), "->", len(speech))
```

## Quality Gates

1. Every document longer than one thousand characters must convert under ten milliseconds in release mode.
2. Math-heavy pages must not leave `$` delimiters around formulas once LatexMathPlugin is wired.
3. Code fences must collapse to language summaries instead of reading braces and indentation aloud.
4. Admonition markers must become callout cues.
5. Tables must announce headers rather than vanishing because the parser ignored table tags.

Teams comparing raw Markdown TTS against parsed voiceover should record synthesis seconds saved per document and archive WAV pairs for human listening sessions. Prefer deterministic fixtures over generative fuzzing when validating speech fidelity for release candidates.
