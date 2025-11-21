# Enterprise Databases

Database management commands.

## List Databases

```bash
redisctl enterprise database list
```

## Get Database

```bash
redisctl enterprise database get <id>
```

## Create Database

```bash
redisctl enterprise database create \
  --name mydb \
  --memory-size 1073741824
```

TODO: Consolidate from core-resources/databases.md
