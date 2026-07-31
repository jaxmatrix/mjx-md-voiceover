//! # `mjx-md-voiceover-plugins`
//!
//! Umbrella plugin crate for `mjx-md-voiceover`.
//! Re-exports individual modular plugin packages under `crates/plugins/`.

pub use mjx_md_voiceover_plugin_admonition::AdmonitionPlugin;
pub use mjx_md_voiceover_plugin_code::CodeBlockPlugin;
pub use mjx_md_voiceover_plugin_latex::LatexMathPlugin;
pub use mjx_md_voiceover_plugin_mermaid::MermaidPlugin;
pub use mjx_md_voiceover_plugin_table::TablePlugin;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugins_reexport() {
        let _p1 = AdmonitionPlugin::new();
        let _p2 = CodeBlockPlugin::new();
        let _p3 = LatexMathPlugin::new();
        let _p4 = MermaidPlugin::new();
        let _p5 = TablePlugin::new();
    }
}
