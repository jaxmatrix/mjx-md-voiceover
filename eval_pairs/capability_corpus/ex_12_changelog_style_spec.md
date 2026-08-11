# Voiceover Engine Changelog Style Specification

Release notes themselves are Markdown and therefore must convert cleanly. This specification doubles as an evaluation document exceeding one thousand characters with mixed syntax.

## Added

- Table AST frames for GFM tables with header and body cell children
- TablePlugin spoken summaries with three-row preview caps
- Capability corpus with twenty long-form documents for regression judging
- WASM registry inclusion of TablePlugin once matching is structural

## Changed

- Mermaid remains registered before CodeBlockPlugin to avoid masking
- LatexMathPlugin continues to protect lone currency dollar signs
- Dataset evaluation harness registers the full plugin set including Mermaid and Table

## Fixed

- Prose containing pipes and hyphens no longer collapses into a fake table summary
- Table events are no longer dropped despite ENABLE_TABLES being set
- Evaluation pairs generator no longer omits Mermaid and Table from the registry

## Code Sample in Notes

```typescript
import { convertMarkdownToVoiceover } from "@mjx/md-voiceover";
export function preview(md: string): string {
  return convertMarkdownToVoiceover(md);
}
```

Authors should write changelogs that remain intelligible when spoken. Avoid ASCII art tables; use GFM tables so the plugin can summarize columns for listeners who cannot see the grid.
