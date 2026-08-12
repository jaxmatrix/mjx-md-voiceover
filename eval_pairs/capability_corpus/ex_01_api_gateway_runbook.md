# API Gateway Production Runbook

This runbook describes how on-call engineers diagnose latency regressions in the edge gateway that fronts the voiceover conversion service. Every section is written so a TTS agent can read it aloud without stumbling over Markdown syntax.

## Symptoms

Operators typically observe p99 latency climbing above 35 milliseconds at the CDN edge while origin CPU remains below 40 percent. Concurrently, error budgets for the `convert_markdown_to_voiceover` route begin to burn faster than the weekly allowance. Confirm the regression with the shared Grafana board titled Gateway Voiceover SLA before paging the platform team.

## Immediate Triage Checklist

1. Verify that the most recent deployment did not silently disable the Mermaid-before-Code plugin registration order.
2. Sample ten live requests and confirm response bodies still omit raw fence markers.
3. Compare warm-cache hit rates for WASM isolates against the prior seven-day baseline.
4. If tables appear as empty speech output, escalate as a parser regression rather than a TTS issue.

## Reference Configuration Snippet

```bash
# Reload edge workers with the pinned WASM artifact
wrangler deploy --env production --compatibility-date=2026-08-01
curl -sS https://edge.example/healthz | jq .voiceover_version
```

## Escalation Matrix

| Severity | Condition | Owner | Page within |
| --- | --- | --- | --- |
| SEV-1 | Error rate > 5% for 5 minutes | Platform on-call | 5 minutes |
| SEV-2 | p99 > 50ms for 15 minutes | Voiceover maintainers | 15 minutes |
| SEV-3 | Single-region soft failure | Regional SRE | 30 minutes |

Document every mitigation in the incident timeline channel so postmortems can reconstruct decision latency accurately. Prefer rollback over hotfixes when the speech fidelity suite regresses on more than two fixtures.
