# CI/CD Integration

Integrate redisctl into your continuous integration and deployment pipelines.

## Overview

This tutorial covers:
- Automated database provisioning
- Environment promotion
- Testing with Redis
- GitOps workflows
- Blue-green deployments

## GitHub Actions

### Database Provisioning Workflow

```yaml
# .github/workflows/provision-redis.yml
name: Provision Redis Database

on:
  workflow_dispatch:
    inputs:
      environment:
        description: 'Environment to deploy to'
        required: true
        type: choice
        options:
          - development
          - staging
          - production
      memory_gb:
        description: 'Memory limit in GB'
        required: true
        default: '4'

jobs:
  provision:
    runs-on: ubuntu-latest
    environment: ${{ github.event.inputs.environment }}
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install redisctl
        run: |
          curl -L https://github.com/redis-developer/redisctl/releases/latest/download/redisctl-linux-amd64.tar.gz | tar xz
          sudo mv redisctl /usr/local/bin/
          redisctl --version
      
      - name: Configure credentials
        env:
          REDIS_CLOUD_API_KEY: ${{ secrets.REDIS_CLOUD_API_KEY }}
          REDIS_CLOUD_API_SECRET: ${{ secrets.REDIS_CLOUD_API_SECRET }}
        run: |
          redisctl profile set ci-${{ github.event.inputs.environment }} \
            --deployment cloud \
            --api-key "$REDIS_CLOUD_API_KEY" \
            --api-secret "$REDIS_CLOUD_API_SECRET"
      
      - name: Create database configuration
        run: |
          cat > database.json <<EOF
          {
            "name": "${{ github.event.inputs.environment }}-${{ github.sha }}",
            "memoryLimitInGb": ${{ github.event.inputs.memory_gb }},
            "protocol": "redis",
            "replication": true,
            "dataPersistence": "aof-every-1-second",
            "dataEvictionPolicy": "allkeys-lru"
          }
          EOF
      
      - name: Provision database
        id: provision
        run: |
          OUTPUT=$(redisctl cloud database create \
            --subscription-id ${{ vars.SUBSCRIPTION_ID }} \
            --data @database.json \
            --wait \
            --output json)
          
          DB_ID=$(echo "$OUTPUT" | jq -r .databaseId)
          ENDPOINT=$(echo "$OUTPUT" | jq -r .publicEndpoint)
          PASSWORD=$(echo "$OUTPUT" | jq -r .password)
          
          echo "database_id=$DB_ID" >> $GITHUB_OUTPUT
          echo "endpoint=$ENDPOINT" >> $GITHUB_OUTPUT
          echo "::add-mask::$PASSWORD"
          echo "password=$PASSWORD" >> $GITHUB_OUTPUT
      
      - name: Update deployment configuration
        run: |
          # Update Kubernetes secret
          kubectl create secret generic redis-${{ github.event.inputs.environment }} \
            --from-literal=endpoint=${{ steps.provision.outputs.endpoint }} \
            --from-literal=password=${{ steps.provision.outputs.password }} \
            --dry-run=client -o yaml | kubectl apply -f -
      
      - name: Run smoke tests
        run: |
          redis-cli -h ${{ steps.provision.outputs.endpoint }} \
            -a ${{ steps.provision.outputs.password }} \
            PING
```

### Environment Promotion

```yaml
# .github/workflows/promote-environment.yml
name: Promote Database Configuration

on:
  workflow_dispatch:
    inputs:
      from_env:
        description: 'Source environment'
        required: true
        type: choice
        options:
          - development
          - staging
      to_env:
        description: 'Target environment'
        required: true
        type: choice
        options:
          - staging
          - production

jobs:
  promote:
    runs-on: ubuntu-latest
    
    steps:
      - name: Export source configuration
        run: |
          SOURCE_DB=$(redisctl cloud database get \
            --subscription-id ${{ vars.SUBSCRIPTION_ID }} \
            --database-id ${{ vars[format('{0}_DATABASE_ID', inputs.from_env)] }} \
            --output json)
          
          # Extract configuration
          echo "$SOURCE_DB" | jq '{
            memoryLimitInGb: .memoryLimitInGb,
            throughputMeasurement: .throughputMeasurement,
            modules: .modules,
            alerts: .alerts,
            dataEvictionPolicy: .dataEvictionPolicy,
            dataPersistence: .dataPersistence
          }' > config.json
      
      - name: Apply to target environment
        run: |
          redisctl cloud database update \
            --subscription-id ${{ vars.SUBSCRIPTION_ID }} \
            --database-id ${{ vars[format('{0}_DATABASE_ID', inputs.to_env)] }} \
            --data @config.json \
            --wait
      
      - name: Verify promotion
        run: |
          TARGET_CONFIG=$(redisctl cloud database get \
            --subscription-id ${{ vars.SUBSCRIPTION_ID }} \
            --database-id ${{ vars[format('{0}_DATABASE_ID', inputs.to_env)] }} \
            --output json)
          
          # Compare configurations
          SOURCE_MEMORY=$(cat config.json | jq .memoryLimitInGb)
          TARGET_MEMORY=$(echo "$TARGET_CONFIG" | jq .memoryLimitInGb)
          
          if [ "$SOURCE_MEMORY" != "$TARGET_MEMORY" ]; then
            echo "Configuration mismatch!"
            exit 1
          fi
```

## GitLab CI/CD

### Pipeline Configuration

```yaml
# .gitlab-ci.yml
stages:
  - validate
  - deploy
  - test
  - promote

variables:
  REDISCTL_VERSION: "latest"

before_script:
  - curl -L https://github.com/redis-developer/redisctl/releases/${REDISCTL_VERSION}/download/redisctl-linux-amd64.tar.gz | tar xz
  - mv redisctl /usr/local/bin/
  - redisctl profile set gitlab --deployment cloud --api-key "$REDIS_API_KEY" --api-secret "$REDIS_API_SECRET"

validate:config:
  stage: validate
  script:
    - |
      for file in configs/*.json; do
        echo "Validating $file"
        jq empty "$file" || exit 1
      done

deploy:development:
  stage: deploy
  environment: development
  script:
    - |
      redisctl cloud database update \
        --subscription-id "$DEV_SUBSCRIPTION_ID" \
        --database-id "$DEV_DATABASE_ID" \
        --data @configs/development.json \
        --wait
  only:
    - develop

deploy:staging:
  stage: deploy
  environment: staging
  script:
    - |
      redisctl cloud database update \
        --subscription-id "$STAGING_SUBSCRIPTION_ID" \
        --database-id "$STAGING_DATABASE_ID" \
        --data @configs/staging.json \
        --wait
  only:
    - main

test:integration:
  stage: test
  script:
    - |
      # Get database endpoint
      ENDPOINT=$(redisctl cloud database get \
        --subscription-id "$DEV_SUBSCRIPTION_ID" \
        --database-id "$DEV_DATABASE_ID" \
        -q "publicEndpoint")
      
      # Run tests
      npm test -- --redis-endpoint="$ENDPOINT"
  dependencies:
    - deploy:development

promote:to:production:
  stage: promote
  environment: production
  when: manual
  script:
    - |
      # Export staging config
      redisctl cloud database get \
        --subscription-id "$STAGING_SUBSCRIPTION_ID" \
        --database-id "$STAGING_DATABASE_ID" \
        -o json > staging-config.json
      
      # Apply to production
      redisctl cloud database update \
        --subscription-id "$PROD_SUBSCRIPTION_ID" \
        --database-id "$PROD_DATABASE_ID" \
        --data @staging-config.json \
        --wait
  only:
    - main
```

## Jenkins Pipeline

### Jenkinsfile

```groovy
// Jenkinsfile
pipeline {
    agent any
    
    environment {
        REDIS_CLOUD_API_KEY = credentials('redis-cloud-api-key')
        REDIS_CLOUD_API_SECRET = credentials('redis-cloud-api-secret')
    }
    
    stages {
        stage('Setup') {
            steps {
                sh '''
                    curl -L https://github.com/redis-developer/redisctl/releases/latest/download/redisctl-linux-amd64.tar.gz | tar xz
                    chmod +x redisctl
                    ./redisctl profile set jenkins \
                        --deployment cloud \
                        --api-key "$REDIS_CLOUD_API_KEY" \
                        --api-secret "$REDIS_CLOUD_API_SECRET"
                '''
            }
        }
        
        stage('Provision Database') {
            when {
                expression { params.PROVISION_NEW == true }
            }
            steps {
                script {
                    def dbConfig = readJSON file: 'database-config.json'
                    dbConfig.name = "${env.JOB_NAME}-${env.BUILD_NUMBER}"
                    
                    writeJSON file: 'temp-config.json', json: dbConfig
                    
                    def result = sh(
                        script: """
                            ./redisctl cloud database create \
                                --subscription-id ${params.SUBSCRIPTION_ID} \
                                --data @temp-config.json \
                                --wait \
                                --output json
                        """,
                        returnStdout: true
                    ).trim()
                    
                    def db = readJSON text: result
                    env.DATABASE_ID = db.databaseId
                    env.DATABASE_ENDPOINT = db.publicEndpoint
                }
            }
        }
        
        stage('Run Tests') {
            steps {
                sh '''
                    export REDIS_ENDPOINT="${DATABASE_ENDPOINT}"
                    npm test
                '''
            }
        }
        
        stage('Cleanup') {
            when {
                expression { params.CLEANUP == true }
            }
            steps {
                sh """
                    ./redisctl cloud database delete \
                        --subscription-id ${params.SUBSCRIPTION_ID} \
                        --database-id ${env.DATABASE_ID} \
                        --wait
                """
            }
        }
    }
    
    post {
        always {
            cleanWs()
        }
    }
}
```

## ArgoCD GitOps

### Application Manifest

```yaml
# argocd/redis-app.yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: redis-databases
  namespace: argocd
spec:
  project: default
  source:
    repoURL: https://github.com/your-org/redis-config
    targetRevision: HEAD
    path: environments/production
  destination:
    server: https://kubernetes.default.svc
  syncPolicy:
    automated:
      prune: false
      selfHeal: true
    syncOptions:
      - CreateNamespace=true
  hooks:
    - name: provision-redis
      manifest: |
        apiVersion: batch/v1
        kind: Job
        metadata:
          name: provision-redis
        spec:
          template:
            spec:
              containers:
              - name: redisctl
                image: redisctl/redisctl:latest
                command:
                  - /bin/sh
                  - -c
                  - |
                    redisctl cloud database update \
                      --subscription-id $SUBSCRIPTION_ID \
                      --database-id $DATABASE_ID \
                      --data @/config/database.json \
                      --wait
                volumeMounts:
                - name: config
                  mountPath: /config
              volumes:
              - name: config
                configMap:
                  name: redis-config
```

## Terraform Integration

### Redis Resource Management

```hcl
# terraform/redis.tf
terraform {
  required_providers {
    shell = {
      source  = "scottwinkler/shell"
      version = "~> 1.7"
    }
  }
}

resource "shell_script" "redis_database" {
  lifecycle_commands {
    create = <<-EOT
      redisctl cloud database create \
        --subscription-id ${var.subscription_id} \
        --data '${jsonencode(var.database_config)}' \
        --wait \
        --output json
    EOT
    
    read = <<-EOT
      redisctl cloud database get \
        --subscription-id ${var.subscription_id} \
        --database-id $(cat database_id.txt) \
        --output json
    EOT
    
    update = <<-EOT
      redisctl cloud database update \
        --subscription-id ${var.subscription_id} \
        --database-id $(cat database_id.txt) \
        --data '${jsonencode(var.database_config)}' \
        --wait \
        --output json
    EOT
    
    delete = <<-EOT
      redisctl cloud database delete \
        --subscription-id ${var.subscription_id} \
        --database-id $(cat database_id.txt) \
        --wait
    EOT
  }
  
  environment = {
    REDIS_CLOUD_API_KEY    = var.redis_api_key
    REDIS_CLOUD_API_SECRET = var.redis_api_secret
  }
}

output "redis_endpoint" {
  value = jsondecode(shell_script.redis_database.output)["publicEndpoint"]
}

output "redis_password" {
  value     = jsondecode(shell_script.redis_database.output)["password"]
  sensitive = true
}
```

## Blue-Green Deployments

### Deployment Script

```bash
#!/bin/bash
# blue-green-deploy.sh

set -euo pipefail

# Configuration
SUBSCRIPTION_ID="${SUBSCRIPTION_ID}"
BLUE_DB_ID="${BLUE_DATABASE_ID}"
GREEN_DB_ID="${GREEN_DATABASE_ID}"
LOAD_BALANCER="${LOAD_BALANCER_NAME}"

# Determine current active environment
CURRENT_ACTIVE=$(kubectl get service redis-active -o jsonpath='{.spec.selector.version}')
echo "Current active: $CURRENT_ACTIVE"

if [ "$CURRENT_ACTIVE" = "blue" ]; then
    TARGET="green"
    TARGET_DB_ID="$GREEN_DB_ID"
else
    TARGET="blue"
    TARGET_DB_ID="$BLUE_DB_ID"
fi

echo "Deploying to $TARGET environment (Database: $TARGET_DB_ID)"

# Update target database
echo "Updating $TARGET database configuration..."
redisctl cloud database update \
    --subscription-id "$SUBSCRIPTION_ID" \
    --database-id "$TARGET_DB_ID" \
    --data @new-config.json \
    --wait

# Run health checks
echo "Running health checks on $TARGET..."
ENDPOINT=$(redisctl cloud database get \
    --subscription-id "$SUBSCRIPTION_ID" \
    --database-id "$TARGET_DB_ID" \
    -q "publicEndpoint")

for i in {1..10}; do
    if redis-cli -h "$ENDPOINT" PING | grep -q PONG; then
        echo "Health check passed"
        break
    fi
    sleep 5
done

# Run smoke tests
echo "Running smoke tests..."
./run-smoke-tests.sh "$ENDPOINT"

# Switch traffic
echo "Switching traffic to $TARGET..."
kubectl patch service redis-active \
    -p '{"spec":{"selector":{"version":"'$TARGET'"}}}'

# Monitor for errors
echo "Monitoring for errors..."
sleep 30

ERROR_COUNT=$(kubectl logs -l app=redis,version=$TARGET --tail=100 | grep -c ERROR || true)
if [ "$ERROR_COUNT" -gt 0 ]; then
    echo "Errors detected! Rolling back..."
    kubectl patch service redis-active \
        -p '{"spec":{"selector":{"version":"'$CURRENT_ACTIVE'"}}}'
    exit 1
fi

echo "Deployment successful! $TARGET is now active"
```

## Best Practices

1. **Store credentials securely** - Use secrets management systems
2. **Use dedicated CI/CD profiles** - Don't reuse production credentials
3. **Implement rollback mechanisms** - Always have a way to revert
4. **Test in staging first** - Never deploy directly to production
5. **Monitor deployments** - Watch for errors during and after deployment
6. **Use infrastructure as code** - Version control your Redis configurations
7. **Implement approval gates** - Require manual approval for production
8. **Audit all changes** - Log who changed what and when
9. **Use immutable deployments** - Create new resources rather than updating
10. **Automate validation** - Test configurations before applying

## Next Steps

- [Managing Production Databases](./production-databases.md)
- [Disaster Recovery](./disaster-recovery.md)
- [Network Security](./network-security.md)