
DB_URL ?= DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/coprocessor

.PHONY: build
build:
	cargo build

.PHONY: cleanup
cleanup:
	docker compose down -v

.PHONY: init_db
init_db:
	docker compose up -d db migration
	sleep 3
	$(DB_URL) sqlx db create
	$(DB_URL) sqlx migrate run

.PHONY: recreate_db
recreate_db:
	$(MAKE) cleanup
	$(MAKE) init_db

.PHONY: run
run:
	docker compose up -d

.PHONY: clean_run
clean_run:
	$(MAKE) recreate_db
	RUST_BACKTRACE=1 $(DB_URL) cargo run --release -- --run-server --run-bg-worker
