# Cost Reports

Step-by-step guides for generating and analyzing Redis Cloud billing data.

## Prerequisites

- Redis Cloud account with API credentials configured
- `redisctl` installed and authenticated

```bash
# Verify your setup
redisctl cloud subscription list
```

## Generate Your First Cost Report

### Step 1: Generate the Report

```bash
redisctl cloud cost-report generate \
  --start-date 2025-01-01 \
  --end-date 2025-01-31 \
  --wait
```

The `--wait` flag tells redisctl to poll until the report is ready. You'll see output like:

```
Task abc123-def456 completed successfully
```

### Step 2: Get the Cost Report ID

The task response contains the `costReportId` needed for download:

```bash
redisctl cloud cost-report generate \
  --start-date 2025-01-01 \
  --end-date 2025-01-31 \
  --wait \
  -o json -q 'response.resourceId'
```

This outputs just the ID:

```
cost-report-xyz789
```

### Step 3: Download the Report

```bash
# Save to file
redisctl cloud cost-report download cost-report-xyz789 \
  --file january-costs.csv

# Or view directly
redisctl cloud cost-report download cost-report-xyz789
```

## One-Liner: Generate and Download

Combine steps using shell substitution:

```bash
# Generate, wait, extract ID, and download in one command
redisctl cloud cost-report download \
  $(redisctl cloud cost-report generate \
      --start-date 2025-01-01 \
      --end-date 2025-01-31 \
      --wait \
      -o json -q 'response.resourceId') \
  --file january-costs.csv
```

## Analyze Costs with JMESPath

Using JSON format with JMESPath queries lets you analyze costs directly without external tools.

### Get Total Cost

```bash
redisctl cloud cost-report download $REPORT_ID \
  --format json \
  -q 'sum([].BilledCost)'
```

### List All Resources and Costs

```bash
redisctl cloud cost-report download $REPORT_ID \
  --format json \
  -q '[].{name: ResourceName, type: ResourceType, cost: BilledCost}'
```

### Find Top 5 Most Expensive Resources

```bash
redisctl cloud cost-report download $REPORT_ID \
  --format json \
  -q 'sort_by(@, &BilledCost) | reverse(@) | [:5].{name: ResourceName, cost: BilledCost}'
```

### Filter by Region

```bash
redisctl cloud cost-report download $REPORT_ID \
  --format json \
  -q '[?Region == `us-east-1`].{name: ResourceName, cost: BilledCost}'
```

### Sum Costs by Region

Using extended JMESPath functions:

```bash
redisctl cloud cost-report download $REPORT_ID \
  --format json \
  -q 'group_by(@, &Region) | @.{region: [0].Region, total: sum([].BilledCost)}'
```

### Filter by Resource Type

```bash
# Only database costs
redisctl cloud cost-report download $REPORT_ID \
  --format json \
  -q '[?ResourceType == `Database`]'

# Only subscription-level costs  
redisctl cloud cost-report download $REPORT_ID \
  --format json \
  -q '[?ResourceType == `Subscription`]'
```

### Extract Costs for a Specific Tag

```bash
redisctl cloud cost-report download $REPORT_ID \
  --format json \
  -q '[?Tags.team == `platform`].{name: ResourceName, cost: BilledCost}'
```

## Monthly Cost Report Script

Automate monthly cost reporting with this script:

```bash
#!/bin/bash
# monthly-cost-report.sh
# Generate and download the previous month's cost report

set -e

# Calculate previous month's date range
if [[ "$OSTYPE" == "darwin"* ]]; then
  # macOS
  START_DATE=$(date -v-1m -v1d +%Y-%m-%d)
  END_DATE=$(date -v1d -v-1d +%Y-%m-%d)
else
  # Linux
  START_DATE=$(date -d "last month" +%Y-%m-01)
  END_DATE=$(date -d "$(date +%Y-%m-01) - 1 day" +%Y-%m-%d)
fi

MONTH_NAME=$(date -d "$START_DATE" +%B-%Y 2>/dev/null || date -j -f "%Y-%m-%d" "$START_DATE" +%B-%Y)
OUTPUT_FILE="redis-cloud-costs-${MONTH_NAME}.csv"

echo "Generating cost report for $START_DATE to $END_DATE..."

# Generate and get report ID
REPORT_ID=$(redisctl cloud cost-report generate \
  --start-date "$START_DATE" \
  --end-date "$END_DATE" \
  --wait \
  -o json -q 'response.resourceId')

echo "Report ID: $REPORT_ID"

# Download
redisctl cloud cost-report download "$REPORT_ID" --file "$OUTPUT_FILE"

echo "Saved to $OUTPUT_FILE"

# Print summary
echo ""
echo "=== Cost Summary ==="
if command -v csvstat &> /dev/null; then
  csvstat "$OUTPUT_FILE" --columns BilledCost
else
  echo "Install csvkit for automatic summary: pip install csvkit"
fi
```

Make it executable and run:

```bash
chmod +x monthly-cost-report.sh
./monthly-cost-report.sh
```

## Team Cost Allocation (Chargeback)

Generate separate reports for each team using tags:

```bash
#!/bin/bash
# team-cost-reports.sh

START_DATE="2025-01-01"
END_DATE="2025-01-31"
TEAMS=("platform" "backend" "frontend" "data")

for team in "${TEAMS[@]}"; do
  echo "Generating report for team: $team"
  
  REPORT_ID=$(redisctl cloud cost-report generate \
    --start-date "$START_DATE" \
    --end-date "$END_DATE" \
    --tag "team:$team" \
    --wait \
    -o json -q 'response.resourceId')
  
  redisctl cloud cost-report download "$REPORT_ID" \
    --file "costs-${team}-$(date +%Y-%m).csv"
  
  # Show total for this team
  TOTAL=$(redisctl cloud cost-report download "$REPORT_ID" \
    --format json \
    -q 'sum([].BilledCost)')
  
  echo "  Team $team total: \$${TOTAL}"
done
```

## Export to Google Sheets

### Option 1: Direct CSV Upload

1. Generate and download the CSV report
2. Open Google Sheets
3. File → Import → Upload → Select your CSV file
4. Choose "Replace spreadsheet" or "Insert new sheet"

### Option 2: Automated with gcloud

```bash
# Generate report
REPORT_ID=$(redisctl cloud cost-report generate \
  --start-date 2025-01-01 \
  --end-date 2025-01-31 \
  --wait \
  -o json -q 'response.resourceId')

# Download and upload to Cloud Storage
redisctl cloud cost-report download "$REPORT_ID" \
  --file /tmp/costs.csv

gsutil cp /tmp/costs.csv gs://your-bucket/cost-reports/

# Then use Google Sheets IMPORTDATA or Apps Script to pull from GCS
```

## Cron-Based Automation

Add to crontab for automated monthly reports:

```bash
# Edit crontab
crontab -e

# Add this line to run on the 2nd of each month at 6 AM
0 6 2 * * /path/to/monthly-cost-report.sh >> /var/log/cost-reports.log 2>&1
```

## Integration with FinOps Tools

### CloudHealth / VMware Aria

Export CSV and configure CloudHealth to ingest from your storage location.

### Kubecost

If running Redis alongside Kubernetes workloads, export FOCUS-format data to your Kubecost data pipeline.

### Custom Dashboards

Use JSON format for direct integration with visualization tools:

```bash
# Export as JSON for Grafana/Superset/etc.
redisctl cloud cost-report download "$REPORT_ID" \
  --format json \
  --file costs.json
```

## Troubleshooting

### "Date range exceeds 40 days"

Split into multiple reports:

```bash
# January 1-31 (31 days) - OK
redisctl cloud cost-report generate \
  --start-date 2025-01-01 \
  --end-date 2025-01-31 \
  --wait

# For longer periods, generate multiple reports
for month in 01 02 03; do
  redisctl cloud cost-report generate \
    --start-date "2025-${month}-01" \
    --end-date "2025-${month}-28" \
    --wait
done
```

### Report takes too long to generate

Large accounts may need more time:

```bash
# Increase timeout to 10 minutes
redisctl cloud cost-report generate \
  --start-date 2025-01-01 \
  --end-date 2025-01-31 \
  --wait \
  --wait-timeout 600
```

### Empty report

- Verify the date range has actual usage
- Check that tag filters match existing resources
- Try without filters to confirm data exists

```bash
# Generate without filters first
redisctl cloud cost-report generate \
  --start-date 2025-01-01 \
  --end-date 2025-01-31 \
  --wait
```

## Related

- [Cost Report Commands](../../cloud/commands/cost-report.md) - Full command reference
- [JMESPath Queries](../../common/jmespath.md) - Query syntax guide
- [Async Operations](../../common/async-operations.md) - Understanding --wait
