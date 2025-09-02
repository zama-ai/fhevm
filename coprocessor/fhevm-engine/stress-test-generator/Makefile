DB_URL ?= DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/coprocessor

.PHONY: build
build:
	cargo build --release

.PHONY: prepare
prepare:
	$(DB_URL) cargo sqlx prepare --

.PHONY: run
run:
	cargo run --release
