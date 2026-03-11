.PHONY: coverage coverage-crate coverage-changed

# Run coverage for the full workspace
coverage:
	cargo llvm-cov clean --workspace --profile coverage
	DATABASE_URL=postgresql://postgres:postgres@localhost:5432/coprocessor \
	TEST_GLOBAL_LOCALSTACK=1 \
	cargo llvm-cov --no-report --workspace --profile coverage -- --test-threads=1
	cargo llvm-cov report --profile coverage 2>&1 | tee coverage-report.txt
	@echo "# commit: $$(git rev-parse HEAD)" >> coverage-report.txt
	@echo ""
	@echo "Coverage report saved to coverage-report.txt"
	@tail -2 coverage-report.txt

# Run coverage for a specific crate
# Usage: make coverage-crate CRATE=host-listener
coverage-crate:
	@test -n "$(CRATE)" || (echo "Usage: make coverage-crate CRATE=<crate-name>" && exit 1)
	cargo llvm-cov clean --workspace --profile coverage
	DATABASE_URL=postgresql://postgres:postgres@localhost:5432/coprocessor \
	TEST_GLOBAL_LOCALSTACK=1 \
	cargo llvm-cov --no-report --package $(CRATE) --profile coverage -- --test-threads=1
	cargo llvm-cov report --profile coverage 2>&1 | tee coverage-report.txt
	@echo "# commit: $$(git rev-parse HEAD)" >> coverage-report.txt
	@echo ""
	@echo "Coverage report saved to coverage-report.txt"
	@tail -2 coverage-report.txt

# Auto-detect changed crates vs main and run coverage only for them
coverage-changed:
	@sh scripts/coverage-changed.sh
