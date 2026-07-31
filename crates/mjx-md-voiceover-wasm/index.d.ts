/**
 * Converts Markdown syntax string into continuous, natural speech text for TTS synthesis.
 *
 * @param markdown Input Markdown formatted string.
 * @returns Natural spoken voiceover prose string.
 */
export function convert_markdown_to_voiceover(markdown: string): string;

/**
 * Returns the JSON string representation of the parsed Markdown Voice AST.
 *
 * @param markdown Input Markdown formatted string.
 * @returns JSON string representing the parsed VoiceAst hierarchy.
 */
export function parse_markdown_ast_json(markdown: string): string;
