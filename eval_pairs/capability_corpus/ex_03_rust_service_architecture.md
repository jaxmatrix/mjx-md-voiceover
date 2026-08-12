# Voiceover Core Service Architecture Brief

The `mjx-md-voiceover-core` crate owns CommonMark parsing, speech formatting, and the plugin trait. Downstream WASM and Python bindings must never reimplement speech rules; they only register plugins and call into the formatter.

## Crate Boundaries

- `mjx-md-voiceover-core`: AST, parser, formatter, registry
- `mjx-md-voiceover-plugins`: facade re-exports for Code, LaTeX, Mermaid, Admonition, Table
- `mjx-md-voiceover-wasm`: browser and edge entry points
- `mjx-md-voiceover-py`: PyO3 module for server-side batch conversion

## Critical Path Code

```rust
use mjx_md_voiceover_core::{PluginRegistry, SpeechFormatter, VoiceAstParser};
use mjx_md_voiceover_plugins::{CodeBlockPlugin, MermaidPlugin, TablePlugin};

pub fn convert(md: &str) -> Result<String, String> {
    let mut registry = PluginRegistry::new();
    registry.register(MermaidPlugin::new());
    registry.register(CodeBlockPlugin::new());
    registry.register(TablePlugin::new());
    let ast = VoiceAstParser::parse(md).map_err(|e| e.to_string())?;
    Ok(SpeechFormatter::format_with_registry(&ast, &registry))
}
```

## Performance Notes

The entire pipeline must finish within one to ten milliseconds for typical documents. Criterion benches flag any case above ten milliseconds as weak. Prefer slice references over owned strings until a plugin must emit a dynamic summary, at which point `Box::leak` mirrors the LaTeX plugin pattern.

## Failure Modes

If Mermaid is registered after CodeBlockPlugin, every mermaid fence becomes a generic code snippet. Tables that never form AST frames produce silent omissions. Admonitions that remain as raw `> [!NOTE]` markers indicate the callout plugin was not registered. Continuous evaluation corpora exist specifically to catch these regressions before release.
