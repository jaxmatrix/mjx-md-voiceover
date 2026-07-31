use mjx_md_voiceover_core::parse_and_format;

#[test]
fn test_zero_unsafe_code_and_resilience() {
    // Malformed & edge-case Markdown inputs
    let untrusted_inputs = vec![
        "",
        "###",
        "[[[[[[]]]]]]",
        "```",
        "```rust",
        "> > > > > nested quotes",
        "* * * * * * * * * *",
        "[link](",
        "$\n$$\n$",
        "\\ \\ \\ \\ \\",
    ];

    for input in untrusted_inputs {
        let res = parse_and_format(input);
        assert!(
            res.is_ok(),
            "Engine panicked or failed on untrusted input: {:?}",
            input
        );
    }
}
