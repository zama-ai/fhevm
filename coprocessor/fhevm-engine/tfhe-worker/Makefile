
DB_URL ?= DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/coprocessor
OPTIMIZATION_TARGET ?= throughput
GPU_STREAMS_PER_DEVICE ?= 16
BENCH_ONE_SHOT_SCENARIO ?= independent_300
# Canonical reportable one-shot suite, in reporting order. This list is the
# single source of truth for CI: the benchmark workflows run exactly these
# scenarios when the "main_block_one_shot" benchmark set is selected.
BENCH_ONE_SHOT_CANONICAL_SCENARIOS = independent_300 dependent_1000_50x20 cross_tx_dependent_1000_50x20 auction_300 traffic_1000_200x5_10x100_lag2 traffic_join_1000_200x5_20acct_lag2
# Subset of the suite to run. Overriding BENCH_ONE_SHOT_SCENARIO on a suite
# target is not supported: it would pin every iteration to one scenario.
BENCH_ONE_SHOT_SCENARIOS ?= $(BENCH_ONE_SHOT_CANONICAL_SCENARIOS)
BENCH_ONE_SHOT_UNKNOWN_SCENARIOS = $(filter-out $(BENCH_ONE_SHOT_CANONICAL_SCENARIOS),$(BENCH_ONE_SHOT_SCENARIOS))
# Reject an empty or unrecognized scenario list before it reaches a recipe
# shell. This runs entirely inside make, so a value coming from a CI input (or
# any other environment) is never expanded into a command: an empty list would
# otherwise "succeed" while measuring nothing, and an arbitrary word would be
# executed rather than rejected.
BENCH_ONE_SHOT_SCENARIO_CHECK = $(if $(strip $(BENCH_ONE_SHOT_SCENARIOS)),,$(error BENCH_ONE_SHOT_SCENARIOS is empty: no one-shot scenario to run))$(if $(BENCH_ONE_SHOT_UNKNOWN_SCENARIOS),$(error unknown one-shot scenario requested; expected a subset of BENCH_ONE_SHOT_CANONICAL_SCENARIOS = $(BENCH_ONE_SHOT_CANONICAL_SCENARIOS)),)
# Reset the benchmark database between scenarios so each measurement starts from
# a freshly migrated database rather than the previous scenario's rows. Uses
# reset_bench_db, which recreates the data without rebuilding the image.
BENCH_ONE_SHOT_RECREATE_DB ?= 0
# Wall-clock budget per scenario, including its build, in seconds; 0 disables it.
# The whole suite must fit the CI job timeout with room for instance setup, so
# this bounds one wedged scenario rather than letting it starve the others:
# 6 canonical scenarios x 3600s leaves half of a 720-minute job spare.
BENCH_ONE_SHOT_SCENARIO_TIMEOUT ?= 3600
# Keep each canonical logical block in one Main scheduling acquisition. Callers
# may still override this for a deliberate dispatch-shape experiment.
BENCH_WORK_ITEMS_BATCH_SIZE ?= $(if $(filter dependent_1000_50x20 cross_tx_dependent_1000_50x20,$(BENCH_ONE_SHOT_SCENARIO)),5000,$(if $(filter auction_300,$(BENCH_ONE_SHOT_SCENARIO)),4800,1500))
BENCH_DEPENDENCE_CHAINS_PER_BATCH ?= 300
# Share each bounded work window across acquired DCIDs. Off by default, matching
# the worker's own default, so a reportable run measures the shipped
# configuration; set to true to measure the adaptive window instead.
BENCH_DCID_ADAPTIVE_BATCH_EXECUTION ?= false
MAIN_BLOCK_BASELINE_CLEARS = -u BENCHMARK_TYPE -u FHEVM_BENCH_MAIN_BLOCK_ONLY -u FHEVM_BENCH_ONE_SHOT -u FHEVM_BENCH_ONE_SHOT_SCENARIO -u FHEVM_BENCH_DIRECT_SMOKE -u FHEVM_TEST_NUM_SAMPLES -u FHEVM_BENCH_RUN_MODE -u FHEVM_BENCH_ISOLATED_DB -u FHEVM_BENCH_DATABASE_URL -u FHEVM_BENCH_STARTUP_TIMEOUT_SECS -u FHEVM_BENCH_WAIT_TIMEOUT_SECS -u FHEVM_BENCH_SHUTDOWN_TIMEOUT_SECS -u FHEVM_BENCH_BUILD_REVISION -u FHEVM_BENCH_SOURCE_STATE -u FHEVM_BENCH_SOURCE_FINGERPRINT -u FHEVM_BENCH_EFFECTIVE_LTO -u CARGO_PROFILE_BENCH_LTO -u COPROCESSOR_TEST_LOCAL_DB -u FHEVM_GPU_STREAMS_PER_DEVICE -u FHEVM_BENCH_WORK_ITEMS_BATCH_SIZE -u FHEVM_BENCH_DEPENDENCE_CHAINS_PER_BATCH -u FHEVM_DCID_BATCH_EXECUTION -u FHEVM_BENCH_DCID_BATCH_EXECUTION -u FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION -u FHEVM_BENCH_DCID_ADAPTIVE_BATCH_EXECUTION -u DATABASE_URL

.PHONY: build
build:
	cargo build

.PHONY: cleanup
cleanup:
	docker compose down -v --remove-orphans

.PHONY: init_db
init_db:
	docker compose up -d --build db-migration
	@status="$$(docker wait db-migration)"; \
	if [ "$$status" != "0" ]; then \
	  echo "db-migration exited with status $$status" >&2; \
	  docker logs db-migration >&2; \
	  exit 1; \
	fi
	@echo "Database migration completed"

# Database for the reportable benchmarks: postgres plus this repository's
# migrations, applied with the sqlx CLI the benchmark runners already install.
#
# Deliberately not init_db. That target builds the migration image, which drags
# in the contract stages and a full workspace cargo build: minutes of
# whole-machine load, and pages of gateway deployment output, for a benchmark
# that needs a schema and nothing else. The worker under test is the one the
# benchmark itself builds, never the one in that image.
BENCH_DB_URL ?= postgresql://postgres:postgres@127.0.0.1:5432/coprocessor
BENCH_DB_MIGRATIONS ?= ../db-migration/migrations

.PHONY: bench_db_up
bench_db_up:
	docker compose up -d db
	@echo "Waiting for postgres..."
	@for attempt in $$(seq 1 60); do \
	  if docker compose exec -T db pg_isready -U postgres >/dev/null 2>&1; then break; fi; \
	  if [ "$$attempt" = "60" ]; then echo "postgres did not become ready" >&2; exit 1; fi; \
	  sleep 2; \
	done
	$(MAKE) migrate_bench_db

.PHONY: migrate_bench_db
migrate_bench_db:
	DATABASE_URL="$(BENCH_DB_URL)" sqlx database create
	DATABASE_URL="$(BENCH_DB_URL)" sqlx migrate run --source "$(BENCH_DB_MIGRATIONS)"
	@echo "Benchmark database migrated"

# Return the benchmark database to a freshly migrated state between scenarios.
.PHONY: reset_bench_db
reset_bench_db:
	docker compose exec -T db psql -U postgres -d postgres \
	  -c "DROP DATABASE IF EXISTS coprocessor WITH (FORCE);"
	$(MAKE) migrate_bench_db

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
	@test -z "$$(git status --porcelain --untracked-files=normal)" || ( \
	  echo "reportable main_block_baseline requires a clean committed worktree; offending entries:" >&2; \
	  git status --porcelain --untracked-files=normal >&2; \
	  exit 2)

.PHONY: benchmark_erc20_main_block_baseline_gpu
benchmark_erc20_main_block_baseline_gpu: check_main_block_baseline_clean_source
	@test -n "$$FHEVM_BENCH_DATABASE_URL" || (echo "FHEVM_BENCH_DATABASE_URL must name a dedicated database" >&2; exit 2)
	env $(MAIN_BLOCK_BASELINE_CLEARS) CARGO_PROFILE_BENCH_LTO=false FHEVM_BENCH_EFFECTIVE_LTO=false BENCHMARK_TYPE=THROUGHPUT FHEVM_BENCH_MAIN_BLOCK_ONLY=1 FHEVM_BENCH_ONE_SHOT=1 FHEVM_BENCH_ONE_SHOT_SCENARIO=$(BENCH_ONE_SHOT_SCENARIO) FHEVM_BENCH_RUN_MODE=reportable FHEVM_BENCH_ISOLATED_DB=1 COPROCESSOR_TEST_LOCAL_DB=1 FHEVM_GPU_STREAMS_PER_DEVICE=$(GPU_STREAMS_PER_DEVICE) FHEVM_BENCH_WORK_ITEMS_BATCH_SIZE=$(BENCH_WORK_ITEMS_BATCH_SIZE) FHEVM_BENCH_DEPENDENCE_CHAINS_PER_BATCH=$(BENCH_DEPENDENCE_CHAINS_PER_BATCH) FHEVM_DCID_BATCH_EXECUTION=true FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION=$(BENCH_DCID_ADAPTIVE_BATCH_EXECUTION) FHEVM_BENCH_DATABASE_URL="$$FHEVM_BENCH_DATABASE_URL" FHEVM_BENCH_BUILD_REVISION="$$(git rev-parse HEAD)" FHEVM_BENCH_SOURCE_STATE=clean FHEVM_BENCH_SOURCE_FINGERPRINT="$$(git rev-parse HEAD)" RUSTFLAGS="-C target-cpu=native" cargo +nightly bench --bench erc20 --features=gpu,bench,$(OPTIMIZATION_TARGET) -- --quick

.PHONY: smoke_erc20_main_block_baseline_gpu
smoke_erc20_main_block_baseline_gpu:
	env $(MAIN_BLOCK_BASELINE_CLEARS) CARGO_PROFILE_BENCH_LTO=false FHEVM_BENCH_EFFECTIVE_LTO=false BENCHMARK_TYPE=THROUGHPUT FHEVM_BENCH_MAIN_BLOCK_ONLY=1 FHEVM_BENCH_RUN_MODE=smoke_only FHEVM_BENCH_DIRECT_SMOKE=1 FHEVM_GPU_STREAMS_PER_DEVICE=$(GPU_STREAMS_PER_DEVICE) RUSTFLAGS="-C target-cpu=native" cargo +nightly bench --bench erc20 --features=gpu,bench,$(OPTIMIZATION_TARGET) -- --quick

.PHONY: benchmark_erc20_main_block_baseline_cpu
benchmark_erc20_main_block_baseline_cpu: check_main_block_baseline_clean_source
	@test -n "$$FHEVM_BENCH_DATABASE_URL" || (echo "FHEVM_BENCH_DATABASE_URL must name a dedicated database" >&2; exit 2)
	env $(MAIN_BLOCK_BASELINE_CLEARS) CARGO_PROFILE_BENCH_LTO=false FHEVM_BENCH_EFFECTIVE_LTO=false BENCHMARK_TYPE=THROUGHPUT FHEVM_BENCH_MAIN_BLOCK_ONLY=1 FHEVM_BENCH_ONE_SHOT=1 FHEVM_BENCH_ONE_SHOT_SCENARIO=$(BENCH_ONE_SHOT_SCENARIO) FHEVM_BENCH_RUN_MODE=reportable FHEVM_BENCH_ISOLATED_DB=1 COPROCESSOR_TEST_LOCAL_DB=1 FHEVM_BENCH_WORK_ITEMS_BATCH_SIZE=$(BENCH_WORK_ITEMS_BATCH_SIZE) FHEVM_BENCH_DEPENDENCE_CHAINS_PER_BATCH=$(BENCH_DEPENDENCE_CHAINS_PER_BATCH) FHEVM_DCID_BATCH_EXECUTION=true FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION=$(BENCH_DCID_ADAPTIVE_BATCH_EXECUTION) FHEVM_BENCH_DATABASE_URL="$$FHEVM_BENCH_DATABASE_URL" FHEVM_BENCH_BUILD_REVISION="$$(git rev-parse HEAD)" FHEVM_BENCH_SOURCE_STATE=clean FHEVM_BENCH_SOURCE_FINGERPRINT="$$(git rev-parse HEAD)" RUSTFLAGS="-C target-cpu=native" cargo +nightly bench --bench erc20 --features=bench,$(OPTIMIZATION_TARGET) -- --quick

.PHONY: smoke_erc20_main_block_baseline_cpu
smoke_erc20_main_block_baseline_cpu:
	env $(MAIN_BLOCK_BASELINE_CLEARS) CARGO_PROFILE_BENCH_LTO=false FHEVM_BENCH_EFFECTIVE_LTO=false BENCHMARK_TYPE=THROUGHPUT FHEVM_BENCH_MAIN_BLOCK_ONLY=1 FHEVM_BENCH_RUN_MODE=smoke_only FHEVM_BENCH_DIRECT_SMOKE=1 RUSTFLAGS="-C target-cpu=native" cargo +nightly bench --bench erc20 --features=bench,$(OPTIMIZATION_TARGET) -- --quick

.PHONY: print_bench_one_shot_scenarios # Print the one-shot scenarios a suite run would cover
print_bench_one_shot_scenarios:
	@$(BENCH_ONE_SHOT_SCENARIO_CHECK)echo $(BENCH_ONE_SHOT_SCENARIOS)

# Run every scenario of the reportable one-shot suite on one backend.
# A failing scenario does not abort the suite: the remaining scenarios still run
# and report, and the suite exits non-zero at the end. Each scenario is bounded
# in wall clock so a wedged one cannot consume the CI job's whole budget: the
# bench's own wait timeout is per wait, not per run, so an unbounded suite of N
# scenarios can outlast the job and be cancelled with nothing reported at all.
# $(1): backend suffix, cpu or gpu
define run_one_shot_suite
	@$(BENCH_ONE_SHOT_SCENARIO_CHECK)set -u; \
	: "A dirty worktree is not a per-scenario failure: report it once up front"; \
	$(MAKE) check_main_block_baseline_clean_source || exit 2; \
	failed=""; \
	for scenario in $(BENCH_ONE_SHOT_SCENARIOS); do \
	  if [ "$(BENCH_ONE_SHOT_RECREATE_DB)" = "1" ]; then \
	    $(MAKE) reset_bench_db || exit 2; \
	  fi; \
	  echo "=== reportable one-shot $(1) scenario: $$scenario (budget $(BENCH_ONE_SHOT_SCENARIO_TIMEOUT)s) ==="; \
	  started=$$(date +%s); \
	  if [ "$(BENCH_ONE_SHOT_SCENARIO_TIMEOUT)" = "0" ]; then \
	    $(MAKE) benchmark_erc20_main_block_baseline_$(1) BENCH_ONE_SHOT_SCENARIO=$$scenario; \
	    status=$$?; \
	  else \
	    timeout --kill-after=60 "$(BENCH_ONE_SHOT_SCENARIO_TIMEOUT)" \
	      $(MAKE) benchmark_erc20_main_block_baseline_$(1) BENCH_ONE_SHOT_SCENARIO=$$scenario; \
	    status=$$?; \
	  fi; \
	  elapsed=$$(($$(date +%s) - started)); \
	  if [ "$$status" -eq 0 ]; then \
	    echo "=== one-shot scenario $$scenario completed in $${elapsed}s ==="; \
	  else \
	    if [ "$$status" -eq 124 ] || [ "$$status" -eq 137 ]; then \
	      echo "!!! reportable one-shot scenario timed out after $${elapsed}s: $$scenario" >&2; \
	    else \
	      echo "!!! reportable one-shot scenario failed after $${elapsed}s (status $$status): $$scenario" >&2; \
	    fi; \
	    failed="$$failed $$scenario"; \
	  fi; \
	done; \
	if [ -n "$$failed" ]; then \
	  echo "failed reportable one-shot scenarios:$$failed" >&2; \
	  exit 1; \
	fi
endef

.PHONY: benchmark_erc20_main_block_one_shot_suite_cpu # Run the reportable one-shot suite on CPU
benchmark_erc20_main_block_one_shot_suite_cpu:
	$(call run_one_shot_suite,cpu)

.PHONY: benchmark_erc20_main_block_one_shot_suite_gpu # Run the reportable one-shot suite on GPU
benchmark_erc20_main_block_one_shot_suite_gpu:
	$(call run_one_shot_suite,gpu)

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
