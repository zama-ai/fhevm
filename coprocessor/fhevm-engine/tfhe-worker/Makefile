
DB_URL ?= DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/coprocessor
OPTIMIZATION_TARGET ?= throughput
GPU_STREAMS_PER_DEVICE ?= 16
BENCH_ONE_SHOT_SCENARIO ?= independent_300
# Keep each canonical logical block in one Main scheduling acquisition. Callers
# may still override this for a deliberate dispatch-shape experiment.
BENCH_WORK_ITEMS_BATCH_SIZE ?= $(if $(filter dependent_1000_50x20 cross_tx_dependent_1000_50x20,$(BENCH_ONE_SHOT_SCENARIO)),5000,$(if $(filter auction_300,$(BENCH_ONE_SHOT_SCENARIO)),4800,1500))
BENCH_DEPENDENCE_CHAINS_PER_BATCH ?= 300
MAIN_BLOCK_BASELINE_CLEARS = -u BENCHMARK_TYPE -u FHEVM_BENCH_MAIN_BLOCK_ONLY -u FHEVM_BENCH_ONE_SHOT -u FHEVM_BENCH_ONE_SHOT_SCENARIO -u FHEVM_BENCH_DIRECT_SMOKE -u FHEVM_TEST_NUM_SAMPLES -u FHEVM_BENCH_RUN_MODE -u FHEVM_BENCH_ISOLATED_DB -u FHEVM_BENCH_DATABASE_URL -u FHEVM_BENCH_STARTUP_TIMEOUT_SECS -u FHEVM_BENCH_WAIT_TIMEOUT_SECS -u FHEVM_BENCH_SHUTDOWN_TIMEOUT_SECS -u FHEVM_BENCH_BUILD_REVISION -u FHEVM_BENCH_SOURCE_STATE -u FHEVM_BENCH_SOURCE_FINGERPRINT -u FHEVM_BENCH_EFFECTIVE_LTO -u CARGO_PROFILE_BENCH_LTO -u COPROCESSOR_TEST_LOCAL_DB -u FHEVM_GPU_STREAMS_PER_DEVICE -u FHEVM_BENCH_WORK_ITEMS_BATCH_SIZE -u FHEVM_BENCH_DEPENDENCE_CHAINS_PER_BATCH -u FHEVM_DCID_BATCH_EXECUTION -u FHEVM_BENCH_DCID_BATCH_EXECUTION -u DATABASE_URL

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
	RUST_BACKTRACE=1 $(DB_URL) cargo run --release -- --run-bg-worker


#
# Benchmarks
#

.PHONY: check_main_block_baseline_clean_source
check_main_block_baseline_clean_source:
	@test -z "$$(git status --porcelain --untracked-files=normal)" || (echo "reportable main_block_baseline requires a clean committed worktree" >&2; exit 2)

.PHONY: benchmark_erc20_main_block_baseline_gpu
benchmark_erc20_main_block_baseline_gpu: check_main_block_baseline_clean_source
	@test -n "$$FHEVM_BENCH_DATABASE_URL" || (echo "FHEVM_BENCH_DATABASE_URL must name a dedicated database" >&2; exit 2)
	env $(MAIN_BLOCK_BASELINE_CLEARS) CARGO_PROFILE_BENCH_LTO=false FHEVM_BENCH_EFFECTIVE_LTO=false BENCHMARK_TYPE=THROUGHPUT FHEVM_BENCH_MAIN_BLOCK_ONLY=1 FHEVM_BENCH_ONE_SHOT=1 FHEVM_BENCH_ONE_SHOT_SCENARIO=$(BENCH_ONE_SHOT_SCENARIO) FHEVM_BENCH_RUN_MODE=reportable FHEVM_BENCH_ISOLATED_DB=1 COPROCESSOR_TEST_LOCAL_DB=1 FHEVM_GPU_STREAMS_PER_DEVICE=$(GPU_STREAMS_PER_DEVICE) FHEVM_BENCH_WORK_ITEMS_BATCH_SIZE=$(BENCH_WORK_ITEMS_BATCH_SIZE) FHEVM_BENCH_DEPENDENCE_CHAINS_PER_BATCH=$(BENCH_DEPENDENCE_CHAINS_PER_BATCH) FHEVM_DCID_BATCH_EXECUTION=true FHEVM_BENCH_DATABASE_URL="$$FHEVM_BENCH_DATABASE_URL" FHEVM_BENCH_BUILD_REVISION="$$(git rev-parse HEAD)" FHEVM_BENCH_SOURCE_STATE=clean FHEVM_BENCH_SOURCE_FINGERPRINT="$$(git rev-parse HEAD)" RUSTFLAGS="-C target-cpu=native" cargo +nightly bench --bench erc20 --features=gpu,bench,$(OPTIMIZATION_TARGET) -- --quick

.PHONY: smoke_erc20_main_block_baseline_gpu
smoke_erc20_main_block_baseline_gpu:
	env $(MAIN_BLOCK_BASELINE_CLEARS) CARGO_PROFILE_BENCH_LTO=false FHEVM_BENCH_EFFECTIVE_LTO=false BENCHMARK_TYPE=THROUGHPUT FHEVM_BENCH_MAIN_BLOCK_ONLY=1 FHEVM_BENCH_RUN_MODE=smoke_only FHEVM_BENCH_DIRECT_SMOKE=1 FHEVM_GPU_STREAMS_PER_DEVICE=$(GPU_STREAMS_PER_DEVICE) RUSTFLAGS="-C target-cpu=native" cargo +nightly bench --bench erc20 --features=gpu,bench,$(OPTIMIZATION_TARGET) -- --quick

.PHONY: benchmark_erc20_main_block_baseline_cpu
benchmark_erc20_main_block_baseline_cpu: check_main_block_baseline_clean_source
	@test -n "$$FHEVM_BENCH_DATABASE_URL" || (echo "FHEVM_BENCH_DATABASE_URL must name a dedicated database" >&2; exit 2)
	env $(MAIN_BLOCK_BASELINE_CLEARS) CARGO_PROFILE_BENCH_LTO=false FHEVM_BENCH_EFFECTIVE_LTO=false BENCHMARK_TYPE=THROUGHPUT FHEVM_BENCH_MAIN_BLOCK_ONLY=1 FHEVM_BENCH_ONE_SHOT=1 FHEVM_BENCH_ONE_SHOT_SCENARIO=$(BENCH_ONE_SHOT_SCENARIO) FHEVM_BENCH_RUN_MODE=reportable FHEVM_BENCH_ISOLATED_DB=1 COPROCESSOR_TEST_LOCAL_DB=1 FHEVM_BENCH_WORK_ITEMS_BATCH_SIZE=$(BENCH_WORK_ITEMS_BATCH_SIZE) FHEVM_BENCH_DEPENDENCE_CHAINS_PER_BATCH=$(BENCH_DEPENDENCE_CHAINS_PER_BATCH) FHEVM_DCID_BATCH_EXECUTION=true FHEVM_BENCH_DATABASE_URL="$$FHEVM_BENCH_DATABASE_URL" FHEVM_BENCH_BUILD_REVISION="$$(git rev-parse HEAD)" FHEVM_BENCH_SOURCE_STATE=clean FHEVM_BENCH_SOURCE_FINGERPRINT="$$(git rev-parse HEAD)" RUSTFLAGS="-C target-cpu=native" cargo +nightly bench --bench erc20 --features=bench,$(OPTIMIZATION_TARGET) -- --quick

.PHONY: smoke_erc20_main_block_baseline_cpu
smoke_erc20_main_block_baseline_cpu:
	env $(MAIN_BLOCK_BASELINE_CLEARS) CARGO_PROFILE_BENCH_LTO=false FHEVM_BENCH_EFFECTIVE_LTO=false BENCHMARK_TYPE=THROUGHPUT FHEVM_BENCH_MAIN_BLOCK_ONLY=1 FHEVM_BENCH_RUN_MODE=smoke_only FHEVM_BENCH_DIRECT_SMOKE=1 RUSTFLAGS="-C target-cpu=native" cargo +nightly bench --bench erc20 --features=bench,$(OPTIMIZATION_TARGET) -- --quick

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
