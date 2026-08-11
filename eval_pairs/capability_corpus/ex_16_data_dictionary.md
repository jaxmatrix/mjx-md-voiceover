# Data Dictionary for Voice Session Analytics

Analytics warehouses store conversion metrics for capacity and fidelity reporting. This dictionary uses tables heavily and includes surrounding prose for trap testing.

## Fact Table Columns

| Column | Type | Description |
| --- | --- | --- |
| session_id | UUID | Unique conversion attempt identifier |
| created_at | timestamptz | Event time in UTC |
| input_bytes | int | Raw Markdown byte length |
| output_bytes | int | Speech text byte length |
| latency_us | int | End-to-end conversion microseconds |
| plugin_mask | int | Bitfield of plugins that claimed nodes |
| error_code | text | Null on success |

## Dimension: Plugin Mask Bits

| Bit | Plugin | Meaning when set |
| --- | --- | --- |
| 0 | Code | At least one fence verbalized |
| 1 | Latex | At least one math span rewritten |
| 2 | Mermaid | At least one diagram summarized |
| 3 | Admonition | At least one callout cued |
| 4 | Table | At least one table summarized |

## Narrative Caution

When analysts write notes like compare east - west | failover modes, those notes are not tables. The converter must leave them as prose. Only GFM tables with header separator rows become `VoiceAstNode::Table` values and therefore TablePlugin speech.

## Example Query

```sql
SELECT date_trunc('hour', created_at) AS hour,
       approx_percentile(latency_us, 0.99) AS p99_us
FROM voice_sessions
WHERE created_at > now() - interval '7 days'
GROUP BY 1
ORDER BY 1;
```
