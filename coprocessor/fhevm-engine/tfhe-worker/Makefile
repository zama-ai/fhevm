
DB_URL ?= DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/coprocessor
OPTIMIZATION_TARGET ?= throughput

.PHONY: build
build:
	cargo build

.PHONY: cleanup
cleanup:
	docker compose down -v --remove-orphans

.PHONY: init_db
init_db:
	docker compose up -d --build db-migration
	docker wait db-migration
	@echo "Database migration completed"

.PHONY: recreate_db
recreate_db:
	$(MAKE) cleanup
	$(MAKE) init_db

.PHONY: run
run:
	docker compose up -d

.PHONY: rerun
rerun:
	$(MAKE) cleanup
	$(MAKE) run

.PHONY: clean_run
clean_run:
	$(MAKE) recreate_db
	RUST_BACKTRACE=1 $(DB_URL) cargo run --release -- --run-server --run-bg-worker


#
# Benchmarks
#

.PHONY: benchmark_erc20_gpu # Run ECR20 transfer benchmarks on GPU
benchmark_erc20_gpu:
	RUSTFLAGS="-C target-cpu=native" $(DB_URL) cargo +nightly bench --bench erc20 --features=gpu,bench,$(OPTIMIZATION_TARGET) -- --quick

.PHONY: benchmark_dex_gpu # Run DEX benchmarks on GPU
benchmark_dex_gpu:
	RUSTFLAGS="-C target-cpu=native" $(DB_URL) cargo +nightly bench --bench dex --features=gpu,bench,$(OPTIMIZATION_TARGET) -- --quick

.PHONY: benchmark_synthetics_gpu # Run all benchmarks on GPU
benchmark_synthetics_gpu:
	RUSTFLAGS="-C target-cpu=native" $(DB_URL) cargo +nightly bench --bench synthetics --features=gpu,bench,$(OPTIMIZATION_TARGET) -- --quick

.PHONY: benchmark_all_gpu # Run all benchmarks on GPU
benchmark_all_gpu:
	RUSTFLAGS="-C target-cpu=native" $(DB_URL) cargo +nightly bench --bench erc20 --bench dex --bench synthetics --features=gpu,bench,$(OPTIMIZATION_TARGET) -- --quick


.PHONY: benchmark_erc20_cpu # Run ECR20 transfer benchmarks on CPU
benchmark_erc20_cpu:
	RUSTFLAGS="-C target-cpu=native" $(DB_URL) cargo +nightly bench --bench erc20 --features=bench -- --quick

.PHONY: benchmark_dex_cpu # Run DEX benchmarks on CPU
benchmark_dex_cpu:
	RUSTFLAGS="-C target-cpu=native" $(DB_URL) cargo +nightly bench --bench dex --features=bench -- --quick

.PHONY: benchmark_synthetics_cpu # Run all benchmarks on CPU
benchmark_synthetics_cpu:
	RUSTFLAGS="-C target-cpu=native" $(DB_URL) cargo +nightly bench --bench synthetics --features=bench -- --quick

.PHONY: benchmark_all_cpu # Run all benchmarks on CPU
benchmark_all_cpu:
	RUSTFLAGS="-C target-cpu=native" $(DB_URL) cargo +nightly bench --bench erc20 --bench dex --bench synthetics --features=bench -- --quick
