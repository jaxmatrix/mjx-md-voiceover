# Customer Support Macros for Voice Agent Escalations

Support engineers paste Markdown macros into ticketing systems that voice agents later narrate to specialists. Macros must remain clear after conversion.

## Password Reset Macro

> [!IMPORTANT]
> Never ask the customer to paste a password into chat. Send a time-limited reset link instead.

Steps the agent should speak:

1. Verify account email ownership using the secondary factor on file.
2. Trigger the reset token API with a fifteen-minute expiry.
3. Confirm the customer can open the link on a trusted device.
4. Document the ticket with the correlation identifier only.

## Billing Dispute Macro

| Field | Example | Spoken Handling |
| --- | --- | --- |
| Invoice ID | INV-2048 | Speak digits clearly |
| Amount | $42.50 | Currency dollars, not math |
| Status | disputed | Plain adjective |
| Priority | high | Plain adjective |

## Snippet for Internal Tools

```sql
SELECT ticket_id, created_at, priority
FROM support_tickets
WHERE status = 'open' AND queue = 'voiceover'
ORDER BY created_at ASC
LIMIT 25;
```

Agents must not hear backtick fences or pipe characters. Currency amounts inside tables should not be treated as LaTeX. This macro document is intentionally longer than one thousand characters to stress mixed plugin interactions in a single conversion call.
