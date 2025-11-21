# Enterprise Monitoring

Statistics, logs, and alerts.

## Statistics

```bash
# Cluster stats
redisctl enterprise stats cluster

# Stream stats continuously
redisctl enterprise stats cluster --follow

# Database stats
redisctl enterprise stats database <id>

# Node stats
redisctl enterprise stats node <id>
```

## Logs

```bash
redisctl enterprise logs list
```

## Alerts

```bash
redisctl enterprise alerts list
```

TODO: Consolidate from monitoring/stats.md, monitoring/logs.md, monitoring/alerts.md
