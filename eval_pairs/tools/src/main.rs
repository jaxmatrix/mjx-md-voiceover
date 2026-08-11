//! One-shot capability eval harness for mjx-md-voiceover shards.
use mjx_md_voiceover_core::{PluginRegistry, SpeechFormatter, VoiceAstParser};
use mjx_md_voiceover_plugins::{
    AdmonitionPlugin, CodeBlockPlugin, LatexMathPlugin, MermaidPlugin, TablePlugin,
};
use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::time::Instant;

#[derive(Serialize)]
struct FileResult {
    id: String,
    input_chars: usize,
    output_chars: usize,
    latency_ms: f64,
    pass_latency: bool,
    pass_no_syntax_noise: bool,
    pass_plugin_cues: bool,
    failures: Vec<String>,
    speech_preview: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    char_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

#[derive(Serialize)]
struct Summary {
    passed: usize,
    failed: usize,
}

#[derive(Serialize)]
struct ShardReport {
    shard: u32,
    files: Vec<String>,
    results: Vec<FileResult>,
    summary: Summary,
}

fn build_registry() -> PluginRegistry {
    // Mermaid before Code (first-match-wins); Table last.
    let mut registry = PluginRegistry::new();
    registry.register(MermaidPlugin::new());
    registry.register(CodeBlockPlugin::new());
    registry.register(LatexMathPlugin::new());
    registry.register(AdmonitionPlugin::new());
    registry.register(TablePlugin::new());
    registry
}

fn check_syntax_noise(speech: &str) -> Vec<String> {
    let mut fails = Vec::new();
    if speech.contains("```") {
        fails.push("contains triple-backtick fence ```".into());
    }
    if speech.contains("[!NOTE]") {
        fails.push("contains raw [!NOTE]".into());
    }
    if speech.contains("[!WARNING]") {
        fails.push("contains raw [!WARNING]".into());
    }
    if speech.contains("[!TIP]") {
        fails.push("contains raw [!TIP]".into());
    }
    if speech.contains("[!IMPORTANT]") {
        fails.push("contains raw [!IMPORTANT]".into());
    }
    if speech.contains("[!CAUTION]") {
        fails.push("contains raw [!CAUTION]".into());
    }
    if speech.contains("| --- |") || speech.contains("|---|") {
        fails.push("contains pipe separator | --- |".into());
    }
    // Raw ### heading marks (as standalone heading syntax leftovers)
    for line in speech.lines() {
        let t = line.trim_start();
        if t.starts_with("### ") || t == "###" || t.starts_with("## ") || t.starts_with("# ") {
            fails.push(format!("raw heading mark in output: {}", &t[..t.len().min(40)]));
            break;
        }
    }
    fails
}

fn check_plugin_cues(id: &str, speech: &str) -> Vec<String> {
    let mut fails = Vec::new();
    let id_lower = id.to_lowercase();

    if id_lower.contains("plugin_code") {
        let ok = speech.contains("Code snippet")
            || speech.contains("SQL")
            || speech.contains("Shell");
        if !ok {
            fails.push(
                "plugin_code: expected cue containing \"Code snippet\" or \"SQL\" or \"Shell\""
                    .into(),
            );
        }
    }

    if id_lower.contains("plugin_latex") {
        // Must not leave $a^2 style math; currency $5 may remain.
        if speech.contains("$a^2") || speech.contains("$a^{") {
            fails.push("plugin_latex: left raw $a^2-style math".into());
        }
        // Detect display/inline math leftovers: $$...$$ or $...$ with latex tokens.
        let has_raw_math_dollar = {
            let mut found = false;
            let chars: Vec<char> = speech.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                if chars[i] == '$' {
                    let display = i + 1 < chars.len() && chars[i + 1] == '$';
                    let start = if display { i + 2 } else { i + 1 };
                    let mut j = start;
                    while j < chars.len() {
                        if chars[j] == '$' {
                            if display && j + 1 < chars.len() && chars[j + 1] == '$' {
                                break;
                            }
                            if !display {
                                break;
                            }
                        }
                        j += 1;
                    }
                    if j > start && j < chars.len() {
                        let inner: String = chars[start..j].iter().collect();
                        let looks_math = inner.contains('^')
                            || inner.contains('_')
                            || inner.contains('\\')
                            || inner.contains('{')
                            || inner.contains('}');
                        // Currency like $5 is digits/punctuation only — not math.
                        let currency_like = !display
                            && inner
                                .chars()
                                .all(|c| c.is_ascii_digit() || c == '.' || c == ',');
                        if looks_math && !currency_like {
                            found = true;
                            break;
                        }
                        i = if display { j + 2 } else { j + 1 };
                        continue;
                    }
                }
                i += 1;
            }
            found
        };
        if has_raw_math_dollar {
            fails.push("plugin_latex: raw math with ^/_/\\ still inside $...$".into());
        }
    }

    if id_lower.contains("plugin_admonition") {
        if !speech.to_lowercase().contains("callout") {
            fails.push("plugin_admonition: expected cue containing \"callout\"".into());
        }
    }

    if id_lower.contains("plugin_mermaid") {
        let ok = speech.contains("diagram")
            || speech.contains("Sequence")
            || speech.contains("flowchart")
            || speech.contains("Architecture")
            || speech.contains("Gantt")
            || speech.contains("class");
        if !ok {
            fails.push(
                "plugin_mermaid: expected cue containing diagram|Sequence|flowchart|Architecture|Gantt|class"
                    .into(),
            );
        }
    }

    if id_lower.contains("plugin_table") {
        let has_table_cue =
            speech.contains("Table with columns") || speech.contains("Table with");
        if !has_table_cue {
            fails.push(
                "plugin_table: expected cue containing \"Table with columns\" or \"Table with\""
                    .into(),
            );
        }
        let trimmed = speech.trim();
        if trimmed == "Structured data table."
            || (trimmed.starts_with("Structured data table.")
                && speech.chars().filter(|c| *c == '.').count() <= 1)
        {
            fails.push(
                "plugin_table: output is only the weak \"Structured data table.\" summary".into(),
            );
        }
        // Prose trap (red - blue | green) must survive; must not wipe doc into one fake table summary.
        let lower = speech.to_lowercase();
        let prose_survives = lower.contains("red")
            && lower.contains("blue")
            && lower.contains("green");
        if !prose_survives {
            fails.push(
                "plugin_table: prose trap (red/blue/green) wiped — expected non-table prose retained"
                    .into(),
            );
        }
    }

    fails
}

fn eval_file(path: &Path, registry: &PluginRegistry) -> FileResult {
    let id = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();
    let markdown = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", path.display(), e);
        process::exit(1);
    });

    let start = Instant::now();
    let ast = VoiceAstParser::parse(&markdown).unwrap_or_else(|e| {
        eprintln!("Parse failed for {}: {}", path.display(), e);
        process::exit(1);
    });
    let speech = SpeechFormatter::format_with_registry(&ast, registry);
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    let input_chars = markdown.chars().count();
    let output_chars = speech.chars().count();
    let char_ratio = if input_chars > 0 {
        Some(((output_chars as f64 / input_chars as f64) * 1000.0).round() / 1000.0)
    } else {
        None
    };

    let mut failures: Vec<String> = Vec::new();

    let pass_latency = latency_ms < 25.0;
    if !pass_latency {
        failures.push(format!(
            "latency {:.3} ms >= 25 ms debug threshold",
            latency_ms
        ));
    }

    let mut note = None;
    if latency_ms > 10.0 {
        note = Some(format!(
            "weak vs release budget: {:.3} ms > 10 ms",
            latency_ms
        ));
    }

    let noise = check_syntax_noise(&speech);
    let pass_no_syntax_noise = noise.is_empty();
    failures.extend(noise);

    let cues = check_plugin_cues(&id, &speech);
    let pass_plugin_cues = cues.is_empty();
    failures.extend(cues);

    if output_chars == 0 {
        failures.push("empty speech output".into());
    }

    // Soft latex preference note
    if id.to_lowercase().contains("plugin_latex") {
        let lower = speech.to_lowercase();
        let has_spoken = lower.contains("squared")
            || lower.contains("fraction")
            || lower.contains("to the power");
        if !has_spoken {
            let soft = "prefer spoken squared/fraction cues when math present".to_string();
            note = Some(match note {
                Some(n) => format!("{}; {}", n, soft),
                None => soft,
            });
        }
    }

    let preview: String = speech.chars().take(200).collect();

    FileResult {
        id,
        input_chars,
        output_chars,
        latency_ms: (latency_ms * 1000.0).round() / 1000.0,
        pass_latency,
        pass_no_syntax_noise,
        pass_plugin_cues,
        failures,
        speech_preview: preview,
        char_ratio,
        note,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut out_path: Option<String> = None;
    let mut shard: u32 = 1;
    let mut files: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                out_path = args.get(i).cloned();
            }
            "--shard" => {
                i += 1;
                shard = args
                    .get(i)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);
            }
            "--files" => {
                i += 1;
                while i < args.len() && !args[i].starts_with("--") {
                    files.push(args[i].clone());
                    i += 1;
                }
                continue;
            }
            other if !other.starts_with("--") => {
                files.push(other.to_string());
            }
            other => {
                eprintln!("Unknown arg: {}", other);
                process::exit(2);
            }
        }
        i += 1;
    }

    let out_path = out_path.unwrap_or_else(|| {
        eprintln!("Usage: run_eval --out <path> [--shard N] --files <md>...");
        process::exit(2);
    });

    if files.is_empty() {
        eprintln!("No input files provided");
        process::exit(2);
    }

    let registry = build_registry();
    let mut results = Vec::new();
    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut file_ids = Vec::new();

    for f in &files {
        let path = Path::new(f);
        file_ids.push(
            path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(f)
                .to_string(),
        );
        let r = eval_file(path, &registry);
        let ok = r.failures.is_empty()
            && r.pass_latency
            && r.pass_no_syntax_noise
            && r.pass_plugin_cues
            && r.output_chars > 0;
        if ok {
            passed += 1;
        } else {
            failed += 1;
        }
        results.push(r);
    }

    let report = ShardReport {
        shard,
        files: file_ids,
        results,
        summary: Summary { passed, failed },
    };

    if let Some(parent) = Path::new(&out_path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    let json = serde_json::to_string_pretty(&report).expect("serialize");
    fs::write(&out_path, json).unwrap_or_else(|e| {
        eprintln!("Failed to write {}: {}", out_path, e);
        process::exit(1);
    });

    println!(
        "Wrote {} (passed={}, failed={})",
        out_path, passed, failed
    );
}
