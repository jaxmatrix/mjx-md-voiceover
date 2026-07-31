use mjx_md_voiceover_core::parse_and_format;
use std::time::Instant;

#[test]
fn test_latency_sla_under_10ms() {
    // Generate a 50 KB Markdown document with headings, lists, blockquotes, code spans, links
    let mut large_doc = String::with_capacity(50_000);
    large_doc.push_str("# SLA Audit Document\n\n");
    for i in 0..500 {
        large_doc.push_str(&format!(
            "## Section {}\nThis is paragraph {} with *emphasis* and `code_fn()` span.\n- List item 1\n- List item 2\n\n> Quote {}\n\n",
            i, i, i
        ));
    }

    let start = Instant::now();
    let speech = parse_and_format(&large_doc).expect("Parsing failed");
    let elapsed = start.elapsed();

    assert!(!speech.is_empty());
    println!("50KB document processed in: {:?}", elapsed);
    let max_allowed_ms = if cfg!(debug_assertions) { 25 } else { 10 };
    assert!(
        elapsed.as_millis() < max_allowed_ms,
        "SLA Violation: processing 50KB doc took {:?}, exceeding {} ms budget!",
        elapsed,
        max_allowed_ms
    );
}
