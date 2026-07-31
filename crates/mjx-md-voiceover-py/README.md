# mjx-md-voiceover (Python Bindings)

Ultra-fast, WASM-compliant Markdown to Voiceover speech engine in Python.

## Installation

```bash
pip install mjx-md-voiceover
```

## Quickstart

```python
import mjx_md_voiceover_py as voiceover

# Convert Markdown to natural speech text
markdown = "# Title\n\n- Item 1\n- Item 2"
speech_text = voiceover.convert_markdown_to_voiceover(markdown)
print(speech_text)
# Output: "Heading: Title. Item 1. Item 2."
```
