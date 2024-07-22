
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

