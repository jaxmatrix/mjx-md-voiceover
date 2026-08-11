# Voiceover Conversion Rules for CommonMark

This document specifies how raw Markdown syntax tokens map to speech-friendly text for Text-to-Speech (TTS) voice synthesis.

| Markdown Syntax                | Raw TTS Problem                                       | Voiceover Conversion Target                      |
| :----------------------------- | :---------------------------------------------------- | :----------------------------------------------- |
| `# Heading 1`                  | Reads "Hash Heading 1"                                | `Heading: Heading 1.` (with pause)               |
| `## Section`                   | Reads "Hash hash Section"                             | `Section: Section.`                              |
| `**bold text**`                | Reads "Asterisk asterisk bold text asterisk asterisk" | `bold text` (spoken with natural emphasis/pause) |
| `*italic text*`                | Reads "Asterisk italic text asterisk"                 | `italic text`                                    |
| `- Item 1`                     | Reads "Dash Item 1"                                   | `Item 1.` (spoken with item pause)               |
| `1. First item`                | Reads "One period First item"                         | `First, First item.`                             |
| `[Google](https://google.com)` | Reads "Left bracket Google right bracket link..."     | `Google` (or optional link cue)                  |
| `` `code` ``                   | Reads "Backtick code backtick"                        | `code` (spoken clearly)                          |
| `> Quote`                      | Reads "Greater than Quote"                            | `Quote: Quote.`                                  |
| `---`                          | Reads "Hyphen hyphen hyphen"                          | `[Pause]` (transitional pause)                   |
| GFM table (`\| A \| B \|` …)   | Reads pipes, dashes, and cells as raw symbols         | `Table with columns A and B. N data rows. Row 1: ….` (via `TablePlugin`; core fallback `Table.`) |

## Pacing and Punctuation Injection

TTS voice synthesis models interpret standard punctuation (periods, commas, ellipsis) to generate speech pauses and pitch modulation.
`mjx-md-voiceover` injects canonical sentence-ending punctuation and pause indicators into generated text to ensure voice agents sound natural and human-like.
