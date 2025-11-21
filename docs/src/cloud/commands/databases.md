# Cloud Databases

Database management commands for Redis Cloud.

## List Databases

```bash
redisctl cloud database list
redisctl cloud database list --subscription-id 123456
```

## Get Database

```bash
redisctl cloud database get <subscription-id> <database-id>
```

## Create Database

```bash
redisctl cloud database create \
  --subscription-id 123456 \
  --name mydb \
  --memory-limit-in-gb 1
```

## Update Database

```bash
redisctl cloud database update <subscription-id> <database-id> \
  --name newname
```

## Delete Database

```bash
redisctl cloud database delete <subscription-id> <database-id>
```

TODO: Consolidate from core-resources/databases.md
