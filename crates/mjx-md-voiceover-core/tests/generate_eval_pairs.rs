use mjx_md_voiceover_core::{PluginRegistry, SpeechFormatter, VoiceAstParser};
use mjx_md_voiceover_plugins::{AdmonitionPlugin, CodeBlockPlugin, LatexMathPlugin};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct EvalPair {
    pub id: String,
    pub name: String,
    pub input_markdown: String,
    pub output_voiceover_text: String,
    pub input_char_count: usize,
    pub output_char_count: usize,
    pub input_word_count: usize,
    pub output_word_count: usize,
    pub readout_char_ratio: f64,
    pub readout_word_ratio: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvalDataset {
    pub dataset_version: String,
    pub total_pairs: usize,
    pub average_char_readout_ratio: f64,
    pub average_word_readout_ratio: f64,
    pub pairs: Vec<EvalPair>,
}

#[test]
fn generate_input_output_eval_pairs() {
    let dataset_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/dataset");
    let out_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../eval_pairs");
    fs::create_dir_all(out_dir).expect("Failed to create eval_pairs directory");

    let entries = fs::read_dir(dataset_dir).expect("Failed to read dataset directory");

    let mut registry = PluginRegistry::new();
    registry.register(CodeBlockPlugin::new());
    registry.register(LatexMathPlugin::new());
    registry.register(AdmonitionPlugin::new());

    let mut eval_pairs = Vec::new();
    let mut total_char_ratio_sum = 0.0;
    let mut total_word_ratio_sum = 0.0;

    for entry in entries {
        let entry = entry.expect("Invalid dataset entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
        let markdown = fs::read_to_string(&path).expect("Failed to read markdown file");

        let ast = VoiceAstParser::parse(&markdown).expect("Failed to parse markdown AST");
        let speech_text = SpeechFormatter::format_with_registry(&ast, &registry);

        let input_chars = markdown.chars().count();
        let output_chars = speech_text.chars().count();

        let input_words = markdown.split_whitespace().count();
        let output_words = speech_text.split_whitespace().count();

        let char_ratio = if input_chars > 0 {
            (output_chars as f64) / (input_chars as f64)
        } else {
            0.0
        };

        let word_ratio = if input_words > 0 {
            (output_words as f64) / (input_words as f64)
        } else {
            0.0
        };

        total_char_ratio_sum += char_ratio;
        total_word_ratio_sum += word_ratio;

        eval_pairs.push(EvalPair {
            id: filename.clone(),
            name: filename.replace('_', " ").to_uppercase(),
            input_markdown: markdown,
            output_voiceover_text: speech_text,
            input_char_count: input_chars,
            output_char_count: output_chars,
            input_word_count: input_words,
            output_word_count: output_words,
            readout_char_ratio: (char_ratio * 1000.0).round() / 1000.0,
            readout_word_ratio: (word_ratio * 1000.0).round() / 1000.0,
        });
    }

    eval_pairs.sort_by(|a, b| a.id.cmp(&b.id));

    let total_pairs = eval_pairs.len();
    let avg_char_ratio = if total_pairs > 0 {
        ((total_char_ratio_sum / total_pairs as f64) * 1000.0).round() / 1000.0
    } else {
        0.0
    };

    let avg_word_ratio = if total_pairs > 0 {
        ((total_word_ratio_sum / total_pairs as f64) * 1000.0).round() / 1000.0
    } else {
        0.0
    };

    let dataset = EvalDataset {
        dataset_version: "1.0.0".to_string(),
        total_pairs,
        average_char_readout_ratio: avg_char_ratio,
        average_word_readout_ratio: avg_word_ratio,
        pairs: eval_pairs,
    };

    let json_output = serde_json::to_string_pretty(&dataset).expect("JSON serialization failed");
    let json_path = Path::new(out_dir).join("dataset_eval_pairs.json");
    fs::write(&json_path, json_output).expect("Failed to write dataset_eval_pairs.json");

    println!("--------------------------------------------------");
    println!("SUCCESSFULLY GENERATED EVALUATION PAIRS DATASET!");
    println!("File Location: {:?}", json_path);
    println!("Total Input/Output Pairs: {}", total_pairs);
    println!("Average Character Readout Ratio: {}", avg_char_ratio);
    println!("Average Word Readout Ratio: {}", avg_word_ratio);
    println!("--------------------------------------------------");
}
