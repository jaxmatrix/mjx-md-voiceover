//! # `mjx-md-voiceover-py`
//!
//! Python C-extension binding layer for `mjx-md-voiceover` using PyO3.

use mjx_md_voiceover_core::{parse_and_format, VoiceAstParser};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Converts Markdown text into continuous, natural speech text for TTS audio synthesis.
#[pyfunction]
#[pyo3(name = "convert_markdown_to_voiceover")]
pub fn convert_markdown_to_voiceover(markdown: &str) -> PyResult<String> {
    parse_and_format(markdown).map_err(|err| PyValueError::new_err(err.to_string()))
}

/// Returns the JSON representation of the parsed Markdown Voice AST.
#[pyfunction]
#[pyo3(name = "parse_markdown_ast_json")]
pub fn parse_markdown_ast_json(markdown: &str) -> PyResult<String> {
    let ast = VoiceAstParser::parse(markdown).map_err(|err| PyValueError::new_err(err.to_string()))?;
    serde_json::to_string(&ast).map_err(|err| PyValueError::new_err(err.to_string()))
}

/// Native Python C-extension module entry point.
#[pymodule]
pub fn mjx_md_voiceover_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert_markdown_to_voiceover, m)?)?;
    m.add_function(wrap_pyfunction!(parse_markdown_ast_json, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py_convert_markdown() {
        let md = "# Hello Python";
        let res = convert_markdown_to_voiceover(md);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Heading: Hello Python.");
    }

    #[test]
    fn test_py_parse_json() {
        let md = "## Section";
        let res = parse_markdown_ast_json(md);
        assert!(res.is_ok());
        assert!(res.unwrap().contains("Heading"));
    }
}
