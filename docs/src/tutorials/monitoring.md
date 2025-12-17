# Setting Up Monitoring

Learn how to monitor Redis Cloud and Enterprise deployments using redisctl with various monitoring stacks.

## Overview

Effective monitoring requires:
- Regular health checks
- Metric collection
- Alert configuration
- Dashboard visualization
- Log aggregation

## Monitoring Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  redisctl   │────▶│ Redis APIs   │────▶│   Metrics   │
│   Scripts   │     │ Cloud/Ent.   │     │  Exporters  │
└─────────────┘     └──────────────┘     └─────────────┘
                                                │
                                                ▼
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   Grafana   │◀────│  Prometheus  │◀────│   Format    │
│  Dashboards │     │   Storage    │     │ Conversion  │
└─────────────┘     └──────────────┘     └─────────────┘
```

## Basic Health Monitoring

### Health Check Script

Create a basic health monitor:

```bash
#!/bin/bash
# health-check.sh

set -euo pipefail

# Configuration
PROFILE="${REDIS_PROFILE:-prod-cloud}"
CHECK_INTERVAL="${CHECK_INTERVAL:-60}"
ALERT_WEBHOOK="${ALERT_WEBHOOK}"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1"
}

send_alert() {
    local level=$1
    local message=$2
    
    if [ -n "$ALERT_WEBHOOK" ]; then
        curl -X POST "$ALERT_WEBHOOK" \
            -H 'Content-Type: application/json' \
            -d "{\"level\": \"$level\", \"message\": \"$message\"}"
    fi
    
    case $level in
        ERROR)   echo -e "${RED}[ERROR]${NC} $message" ;;
        WARNING) echo -e "${YELLOW}[WARN]${NC} $message" ;;
        INFO)    echo -e "${GREEN}[INFO]${NC} $message" ;;
    esac
}

check_databases() {
    local subscription_id=$1
    
    # Get all databases
    local databases=$(redisctl --profile $PROFILE cloud database list \
        --subscription-id $subscription_id \
        -q "[].{id: databaseId, name: name, status: status}" 2>/dev/null)
    
    if [ -z "$databases" ]; then
        send_alert "ERROR" "Failed to fetch databases for subscription $subscription_id"
        return 1
    fi
    
    # Process each database - extract fields directly with JMESPath
    for db_entry in $(echo "$databases" | jq -c '.[]'); do
        local id=$(echo $db_entry | jq -r .id)
        local name=$(echo $db_entry | jq -r .name)
        local status=$(echo $db_entry | jq -r .status)
        
        if [ "$status" != "active" ]; then
            send_alert "ERROR" "Database $name ($id) is not active: $status"
        else
            log "Database $name ($id) is healthy"
        fi
    done
}

# Main monitoring loop
while true; do
    log "Starting health check..."
    
    # Get all subscriptions
    SUBSCRIPTIONS=$(redisctl --profile $PROFILE cloud subscription list \
        -q "[].id" --raw 2>/dev/null)
    
    for sub_id in $SUBSCRIPTIONS; do
        check_databases $sub_id
    done
    
    log "Health check complete. Sleeping for ${CHECK_INTERVAL}s..."
    sleep $CHECK_INTERVAL
done
```

## Prometheus Integration

### Metrics Exporter

Create a Prometheus exporter for Redis metrics:

```python
#!/usr/bin/env python3
# redis_exporter.py

import json
import subprocess
import time
from prometheus_client import start_http_server, Gauge, Counter
import os

# Prometheus metrics
db_memory_used = Gauge('redis_memory_used_mb', 'Memory used in MB', ['database', 'subscription'])
db_memory_limit = Gauge('redis_memory_limit_gb', 'Memory limit in GB', ['database', 'subscription'])
db_connections = Gauge('redis_connections_used', 'Connections used', ['database', 'subscription'])
db_ops = Gauge('redis_operations_per_second', 'Operations per second', ['database', 'subscription'])
db_status = Gauge('redis_database_status', 'Database status (1=active, 0=inactive)', ['database', 'subscription'])

def get_databases(profile, subscription_id):
    """Fetch database list using redisctl"""
    cmd = [
        'redisctl', '--profile', profile, 'cloud', 'database', 'list',
        '--subscription-id', str(subscription_id), '-o', 'json'
    ]
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return json.loads(result.stdout)
    except Exception as e:
        print(f"Error fetching databases: {e}")
        return []

def get_database_details(profile, subscription_id, database_id):
    """Fetch detailed database metrics"""
    cmd = [
        'redisctl', '--profile', profile, 'cloud', 'database', 'get',
        '--subscription-id', str(subscription_id),
        '--database-id', str(database_id),
        '-o', 'json'
    ]
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return json.loads(result.stdout)
    except Exception as e:
        print(f"Error fetching database {database_id}: {e}")
        return None

def collect_metrics():
    """Collect metrics from all databases"""
    profile = os.getenv('REDIS_PROFILE', 'prod-cloud')
    subscriptions = os.getenv('REDIS_SUBSCRIPTIONS', '').split(',')
    
    for sub_id in subscriptions:
        if not sub_id:
            continue
            
        databases = get_databases(profile, sub_id)
        
        for db in databases:
            db_id = db.get('databaseId')
            db_name = db.get('name', f'db-{db_id}')
            
            # Get detailed metrics
            details = get_database_details(profile, sub_id, db_id)
            if not details:
                continue
            
            # Update Prometheus metrics
            labels = {'database': db_name, 'subscription': sub_id}
            
            db_memory_used.labels(**labels).set(details.get('memoryUsageInMB', 0))
            db_memory_limit.labels(**labels).set(details.get('memoryLimitInGB', 0))
            db_connections.labels(**labels).set(details.get('connectionsUsed', 0))
            
            throughput = details.get('throughputMeasurement', {})
            db_ops.labels(**labels).set(throughput.get('value', 0))
            
            status_value = 1 if details.get('status') == 'active' else 0
            db_status.labels(**labels).set(status_value)
            
            print(f"Updated metrics for {db_name}")

def main():
    """Main exporter loop"""
    port = int(os.getenv('EXPORTER_PORT', '9090'))
    interval = int(os.getenv('SCRAPE_INTERVAL', '30'))
    
    # Start Prometheus HTTP server
    start_http_server(port)
    print(f"Exporter listening on port {port}")
    
    while True:
        try:
            collect_metrics()
        except Exception as e:
            print(f"Error collecting metrics: {e}")
        
        time.sleep(interval)

if __name__ == '__main__':
    main()
```

### Prometheus Configuration

Configure Prometheus to scrape the exporter:

```yaml
# prometheus.yml
global:
  scrape_interval: 30s
  evaluation_interval: 30s

scrape_configs:
  - job_name: 'redis-metrics'
    static_configs:
      - targets: ['localhost:9090']
        labels:
          environment: 'production'
          service: 'redis'

# Alert rules
rule_files:
  - 'redis_alerts.yml'
```

### Alert Rules

Define Prometheus alert rules:

```yaml
# redis_alerts.yml
groups:
  - name: redis_alerts
    interval: 30s
    rules:
      - alert: RedisHighMemoryUsage
        expr: |
          (redis_memory_used_mb / (redis_memory_limit_gb * 1024)) > 0.8
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage on {{ $labels.database }}"
          description: "Database {{ $labels.database }} is using {{ $value | humanizePercentage }} of available memory"
      
      - alert: RedisDatabaseDown
        expr: redis_database_status == 0
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Database {{ $labels.database }} is down"
          description: "Database {{ $labels.database }} has been inactive for more than 2 minutes"
      
      - alert: RedisHighConnections
        expr: redis_connections_used > 900
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High connection count on {{ $labels.database }}"
          description: "Database {{ $labels.database }} has {{ $value }} active connections"
      
      - alert: RedisLowThroughput
        expr: redis_operations_per_second < 100
        for: 10m
        labels:
          severity: info
        annotations:
          summary: "Low throughput on {{ $labels.database }}"
          description: "Database {{ $labels.database }} has only {{ $value }} ops/sec"
```

## Grafana Dashboards

### Dashboard Configuration

Create a comprehensive Grafana dashboard:

```json
{
  "dashboard": {
    "title": "Redis Production Monitoring",
    "panels": [
      {
        "title": "Database Status",
        "type": "stat",
        "targets": [
          {
            "expr": "sum(redis_database_status)",
            "legendFormat": "Active Databases"
          }
        ]
      },
      {
        "title": "Memory Usage",
        "type": "graph",
        "targets": [
          {
            "expr": "redis_memory_used_mb",
            "legendFormat": "{{ database }}"
          }
        ]
      },
      {
        "title": "Operations/Second",
        "type": "graph",
        "targets": [
          {
            "expr": "redis_operations_per_second",
            "legendFormat": "{{ database }}"
          }
        ]
      },
      {
        "title": "Connection Count",
        "type": "graph",
        "targets": [
          {
            "expr": "redis_connections_used",
            "legendFormat": "{{ database }}"
          }
        ]
      }
    ]
  }
}
```

## Log Monitoring

### Centralized Logging with ELK

Ship Redis logs to Elasticsearch:

```bash
#!/bin/bash
# ship-logs.sh

# For Redis Enterprise - using jq for JSON transformation to ELK format
redisctl enterprise logs list \
  --profile prod-enterprise \
  --output json | \
  jq -c '.[] | {
    "@timestamp": .time,
    "level": .severity,
    "message": .message,
    "node": .node_uid,
    "component": .component
  }' | \
  while read log; do
    curl -X POST "http://elasticsearch:9200/redis-logs/_doc" \
      -H 'Content-Type: application/json' \
      -d "$log"
  done
```

### Logstash Configuration

Process logs with Logstash:

```ruby
# logstash.conf
input {
  exec {
    command => "redisctl enterprise logs list --output json"
    interval => 60
    codec => "json"
  }
}

filter {
  date {
    match => [ "time", "ISO8601" ]
    target => "@timestamp"
  }
  
  mutate {
    add_field => { "environment" => "production" }
  }
  
  if [severity] == "error" {
    mutate {
      add_tag => [ "alert" ]
    }
  }
}

output {
  elasticsearch {
    hosts => ["localhost:9200"]
    index => "redis-logs-%{+YYYY.MM.dd}"
  }
  
  if "alert" in [tags] {
    email {
      to => "ops-team@example.com"
      subject => "Redis Error Alert"
      body => "Error detected: %{message}"
    }
  }
}
```

## Alerting Integration

### Slack Notifications

Send alerts to Slack:

```bash
#!/bin/bash
# slack-alert.sh

send_slack_alert() {
    local level=$1
    local message=$2
    local webhook_url="${SLACK_WEBHOOK_URL}"
    
    local color="good"
    case $level in
        ERROR)   color="danger" ;;
        WARNING) color="warning" ;;
    esac
    
    curl -X POST "$webhook_url" \
        -H 'Content-Type: application/json' \
        -d "{
            \"attachments\": [{
                \"color\": \"$color\",
                \"title\": \"Redis Alert: $level\",
                \"text\": \"$message\",
                \"footer\": \"redisctl monitoring\",
                \"ts\": $(date +%s)
            }]
        }"
}

# Monitor and alert
while true; do
    STATUS=$(redisctl cloud database get \
        --subscription-id 123456 \
        --database-id 789 \
        -q "status")
    
    if [ "$STATUS" != "active" ]; then
        send_slack_alert "ERROR" "Database 789 is $STATUS"
    fi
    
    sleep 60
done
```

### PagerDuty Integration

Integrate with PagerDuty for critical alerts:

```python
#!/usr/bin/env python3
# pagerduty_alert.py

import pdpyras
import subprocess
import json
import os

def check_redis_health():
    """Check Redis database health"""
    cmd = [
        'redisctl', 'cloud', 'database', 'list',
        '--subscription-id', os.getenv('SUBSCRIPTION_ID'),
        '-o', 'json'
    ]
    
    result = subprocess.run(cmd, capture_output=True, text=True)
    databases = json.loads(result.stdout)
    
    alerts = []
    for db in databases:
        if db['status'] != 'active':
            alerts.append({
                'database': db['name'],
                'status': db['status'],
                'id': db['databaseId']
            })
    
    return alerts

def send_pagerduty_alert(session, alerts):
    """Send alert to PagerDuty"""
    for alert in alerts:
        session.trigger_incident(
            summary=f"Redis database {alert['database']} is {alert['status']}",
            source="redisctl-monitoring",
            severity="error",
            custom_details=alert
        )

def main():
    api_key = os.getenv('PAGERDUTY_API_KEY')
    session = pdpyras.APISession(api_key)
    
    alerts = check_redis_health()
    if alerts:
        send_pagerduty_alert(session, alerts)

if __name__ == '__main__':
    main()
```

## Custom Metrics Collection

### Performance Baseline

Establish performance baselines:

```bash
#!/bin/bash
# baseline.sh

# Collect baseline metrics for 24 hours
DURATION=86400
INTERVAL=60
OUTPUT="baseline_$(date +%Y%m%d).csv"

echo "timestamp,database,ops,latency,memory,cpu" > $OUTPUT

END=$(($(date +%s) + DURATION))
while [ $(date +%s) -lt $END ]; do
    TIMESTAMP=$(date +%s)
    
    # Use JMESPath for field extraction, then format as CSV
    METRICS=$(redisctl cloud database get \
        --subscription-id 123456 \
        --database-id 789 \
        -q "[throughputMeasurement.value, latency, memoryUsageInMB, cpuUsagePercentage]" --raw)
    
    echo "$TIMESTAMP,prod-db,$METRICS" | tr -d '[]' >> $OUTPUT
    
    sleep $INTERVAL
done

# Analyze baseline
echo "Baseline collection complete. Analyzing..."
python3 analyze_baseline.py $OUTPUT
```

## Automation with Cron

Schedule monitoring tasks:

```bash
# crontab -e

# Health check every 5 minutes
*/5 * * * * /opt/monitoring/health-check.sh

# Collect metrics every minute
* * * * * /opt/monitoring/collect-metrics.sh

# Daily report
0 8 * * * /opt/monitoring/daily-report.sh

# Weekly capacity planning
0 0 * * 0 /opt/monitoring/capacity-planning.sh

# Backup monitoring config
0 2 * * * /opt/monitoring/backup-monitoring.sh
```

## Best Practices

1. **Monitor proactively** - Set up alerts before issues occur
2. **Use multiple data sources** - Combine metrics, logs, and traces
3. **Set appropriate thresholds** - Avoid alert fatigue
4. **Automate responses** - Use runbooks for common issues
5. **Track trends** - Look for patterns over time
6. **Test alert paths** - Ensure alerts reach the right people
7. **Document procedures** - Have clear escalation paths
8. **Review regularly** - Update monitoring as systems evolve

## Next Steps

- [Disaster Recovery](./disaster-recovery.md)
- [Network Security](./network-security.md)
- [CI/CD Integration](./cicd.md)