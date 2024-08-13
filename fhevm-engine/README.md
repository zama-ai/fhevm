# Fhevm Coprocessor

## Dependencies installation

- `docker-compose`
- `rust`
- `sqlx-cli`:
```
cargo install sqlx-cli
```

## Development

Start the database
```
docker compose up -d
```

Export database url for development
```
export DATABASE_URL="postgres://postgres:postgres@localhost/coprocessor"
```

Create the database
```
sqlx db create
```

Run the migrations
```
sqlx migrate run
```

## Debugging database

Exec into postgresql shell
```
docker exec -u postgres -it fhevm-coprocessor-db-1 psql coprocessor
```

## Running tests
```
cargo test
```

## Running the first working fhevm coprocessor smoke test

Reload database and apply schemas from scratch
```
make recreate_db
```
Run the server and background fhe worker
```
cargo run -- --run-server --run-bg-worker
```