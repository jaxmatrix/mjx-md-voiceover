# Production Deployment Guidelines

Please review all safety requirements prior to executing production migrations.

> [!NOTE]
> Database backup snapshots are automatically created every 6 hours.

> [!TIP]
> Use `--dry-run` flag to validate schema migrations before applying changes.

> [!IMPORTANT]
> Ensure all active worker nodes are drained before terminating instance groups.

> [!WARNING]
> Dropping columns without zero-downtime deprecation strategy will break active API clients.

> [!CAUTION]
> Hard resetting the primary database cluster will result in irreversible data loss.

## Operational Escalation Procedure

> If primary region fails:
> > Notify site reliability engineering team immediately.
> > > Initiate multi-region failover DNS routing.
