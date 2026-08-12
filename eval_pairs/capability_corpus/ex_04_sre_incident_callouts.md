# Multi-Region Failover Playbook with Auditory Callouts

Site reliability engineers rely on spoken runbooks during paging events when visual dashboards are hard to parse under stress. Callout plugins must turn GitHub-style admonitions into clear auditory alerts.

## Pre-Failover Checks

> [!NOTE]
> Database backup snapshots are automatically created every six hours and retained for fourteen days in the cold storage bucket.

> [!TIP]
> Use the `--dry-run` flag on the schema migrator to validate pending changes before applying them to the primary writer.

> [!IMPORTANT]
> Drain all active worker nodes and wait for queue depth to reach zero before terminating instance groups in the failing region.

## Risk Statements

> [!WARNING]
> Dropping columns without a zero-downtime deprecation strategy will break active API clients still pinned to the previous schema revision.

> [!CAUTION]
> Hard resetting the primary database cluster will result in irreversible data loss for any transactions not yet replicated to the standby region.

## Nested Escalation Quote

> If primary region fails:
> > Notify site reliability engineering immediately.
> > > Initiate multi-region failover DNS routing and freeze deploys.

After spoken conversion, reviewers must hear “Note callout”, “Warning callout”, and related cues instead of “greater than left bracket exclamation mark”. Nested blockquotes should retain conversational quote framing without reading Markdown markers aloud.
