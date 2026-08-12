# Draft: Voiceover Conversion Protocol Considerations

This informal RFC-style note describes interoperability expectations for clients calling Markdown voiceover services over HTTP or in-process FFI.

## Abstract

Clients submit UTF-8 Markdown and receive UTF-8 speech text suitable for TTS. Servers MAY register plugins. Clients MUST NOT assume raw CommonMark symbols survive. Latency SHOULD remain under ten milliseconds for typical payloads under eight kilobytes.

## Terminology

The key words MUST, MUST NOT, SHOULD, and MAY are interpreted as described in BCP 14 when capitalized. “Weak” means any conversion exceeding ten milliseconds on the standard corpus in release builds.

## Message Example

```http
POST /v1/convert HTTP/1.1
Host: voiceover.example
Content-Type: text/markdown; charset=utf-8
Content-Length: 128

# Title

Hello **world** with `code` and a link to [docs](https://example.com).
```

## Response Expectations

Speech text SHOULD begin with “Heading:” for top-level titles. Emphasis markers MUST NOT appear as asterisk runs. Links SHOULD speak display text without bracket noise. Optional plugin behaviors include code summaries, math verbalization, callout cues, diagram overviews, and table column announcements.

## Security Considerations

Treat Markdown as untrusted. Cap input size at the gateway. Prefer typed errors over panics. Do not execute embedded HTML or scripts even if a future HTML plugin summarizes markup for speech.
