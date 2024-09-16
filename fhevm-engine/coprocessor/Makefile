
.PHONY: build
build:
	cargo build

.PHONY: cleanup
cleanup:
	docker compose down -v

.PHONY: init_db
init_db:
	docker compose up -d
	sleep 3
	sqlx db create
	sqlx migrate run

.PHONY: recreate_db
recreate_db:
	$(MAKE) cleanup
	$(MAKE) init_db

.PHONY: docker_compose_setup
docker_compose_setup:
	$(MAKE) init_db
	cargo test setup_test_user -- --nocapture --ignored

.PHONY: clean_run
clean_run:
	$(MAKE) recreate_db
	RUST_BACKTRACE=1 cargo run -- --run-server --run-bg-worker
