# fhevm-sql-exporter

Helm chart that ships [burningalchemist/sql_exporter](https://github.com/burningalchemist/sql_exporter)
preconfigured to export Coprocessor Postgres metrics to Prometheus.

## Configuration

The exporter is configured via a YAML file: `config/queries.yml`.

## Install in Kubernetes

```bash
helm dependency update
helm upgrade --install fhevm-sql-exporter . -n coprocessor
```

The `database-credentials` Secret with keys `endpoint`, `username`, and
`password` must already exist in the release namespace.

## Run locally

Useful when iterating on `config/queries.yml`.

### Validate config syntax

```bash
sql_exporter -config.file config/queries.yml -config.check
```

This parses YAML and resolves collector references but does not connect to
Postgres, so the DATABASE env vars do not need to be set.

### Scrape a real database

Point the exporter at a reachable Postgres (e.g. a local instance):

```bash
export DATA_SOURCE_NAME='postgres://coprocessor:$DATABASE_PASSWORD@localhost:5432/coprocessor'
sql_exporter -config.file config/queries.yml
curl -s localhost:9399/metrics
```

The `sql_exporter` binary is available on their [release page](https://github.com/burningalchemist/sql_exporter/releases).