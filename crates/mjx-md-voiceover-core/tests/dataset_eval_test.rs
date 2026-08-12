use mjx_md_voiceover_core::{parse_and_format, PluginRegistry, SpeechFormatter, VoiceAstParser};
use mjx_md_voiceover_plugins::{
    AdmonitionPlugin, CodeBlockPlugin, LatexMathPlugin, MermaidPlugin, TablePlugin,
};
use std::fs;
use std::time::Instant;

#[test]
fn test_markdown_dataset_conversion_and_benchmarks() {
    let dataset_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/dataset");
    let entries = fs::read_dir(dataset_dir).expect("Failed to read dataset directory");

    // Mermaid before Code (first-match-wins); Table last (structural match only).
    let mut registry = PluginRegistry::new();
    registry.register(MermaidPlugin::new());
    registry.register(CodeBlockPlugin::new());
    registry.register(LatexMathPlugin::new());
    registry.register(AdmonitionPlugin::new());
    registry.register(TablePlugin::new());

    let mut test_count = 0;

    for entry in entries {
        let entry = entry.expect("Invalid dataset directory entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let markdown = fs::read_to_string(&path).expect("Failed to read markdown file");
        let doc_bytes = markdown.len();

        // Measure Core parsing + formatting latency
        let start_core = Instant::now();
        let core_speech = parse_and_format(&markdown).expect("Core parsing failed");
        let core_latency = start_core.elapsed();

        // Measure Plugin pipeline parsing + transformation + formatting latency
        let start_plugin = Instant::now();
        let ast = VoiceAstParser::parse(&markdown).expect("Plugin AST parsing failed");
        let plugin_speech = SpeechFormatter::format_with_registry(&ast, &registry);
        let plugin_latency = start_plugin.elapsed();

        test_count += 1;

        println!("--------------------------------------------------");
        println!("DATASET FILE: {}", file_name);
        println!("Size: {} bytes", doc_bytes);
        println!("Core Engine Latency: {:?}", core_latency);
        println!("Plugin Ecosystem Latency: {:?}", plugin_latency);
        println!(
            "Core Speech Preview (first 120 chars): {:.120}...",
            core_speech.replace('\n', " ")
        );
        println!(
            "Plugin Speech Preview (first 120 chars): {:.120}...",
            plugin_speech.replace('\n', " ")
        );

        // Assertions
        assert!(
            !core_speech.is_empty(),
            "Core speech string is empty for {}",
            file_name
        );
        assert!(
            !plugin_speech.is_empty(),
            "Plugin speech string is empty for {}",
            file_name
        );

        let max_allowed_ms = if cfg!(debug_assertions) { 25 } else { 10 };
        assert!(
            plugin_latency.as_millis() < max_allowed_ms,
            "Latency SLA breach for {}: {:?} > {}ms",
            file_name,
            plugin_latency,
            max_allowed_ms
        );
    }

    assert!(
        test_count >= 5,
        "Dataset must contain at least 5 markdown files"
    );
    println!("--------------------------------------------------");
    println!("Successfully evaluated {} dataset files!", test_count);
}
