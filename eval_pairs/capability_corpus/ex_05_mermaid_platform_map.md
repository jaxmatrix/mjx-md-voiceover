# Platform Topology Narrated Through Mermaid

Architecture diagrams are useless to voice agents unless Mermaid fences become short spoken overviews. This document mixes several diagram kinds that the Mermaid plugin must classify.

## Request Path

```mermaid
graph TD
  Client[Voice Agent Client] --> Edge[Edge WASM Isolate]
  Edge --> Core[VoiceAstParser]
  Core --> Plugins[PluginRegistry]
  Plugins --> Fmt[SpeechFormatter]
  Fmt --> TTS[TTS Engine]
```

## Session Handshake

```mermaid
sequenceDiagram
  participant Agent
  participant WASM
  participant Core
  Agent->>WASM: convert_markdown_to_voiceover
  WASM->>Core: parse + format_with_registry
  Core-->>WASM: speech text
  WASM-->>Agent: Result String
```

## Domain Model

```mermaid
classDiagram
  class VoiceAst {
    +nodes Vec
  }
  class VoiceAstNode
  class SpeechToken
  VoiceAst --> VoiceAstNode
  VoiceAstNode --> SpeechToken : via formatter
```

## Timeline

```mermaid
gantt
  title Plugin Hardening Sprint
  dateFormat  YYYY-MM-DD
  section Parser
  Table frames           :a1, 2026-08-11, 2d
  section Plugins
  Table verbalizer       :a2, after a1, 1d
  Eval corpus            :a3, after a2, 1d
```

Judges should hear flowchart, sequence, class, and Gantt summaries rather than raw Mermaid keywords read character by character.
