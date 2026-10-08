#!/usr/bin/env bash
# Fails when a consumer's CI path filter misses a solana/ path the consumer reads: a crate its
# Cargo workspace links, directly or through another solana/ crate, or the shared test fixtures
# its tests load. Without the entry, a change to that path alone does not run the consumer's
# checks, and the break surfaces later on an unrelated PR. Other solana/ inputs, such as the
# environment files a build script reads, are not checked.
#
# The Cargo manifests are the one source of the crates. Per-image docker-build filters are not
# checked here; coprocessor-docker-build's are checked by
# test-suite/fhevm/src/coprocessor-build-filters.test.ts.
set -euo pipefail

cd "$(dirname "$0")/../.."
root="$(pwd -P)/"
workflows=.github/workflows
fixtures=solana/test-fixtures

# Each solana/ crate's own path dependencies. Its dev-dependencies never reach a consumer.
solana_graph=$(
  cargo metadata --format-version 1 --no-deps --manifest-path solana/Cargo.toml |
    jq -c '[.packages[] | {
      key: (.manifest_path | rtrimstr("/Cargo.toml")),
      value: [.dependencies[] | select(.path != null and .kind != "dev") | .path]
    }] | from_entries'
)

# The repo-relative solana/ crates a Cargo workspace links, one per line, dev-dependencies
# included: its tests compile them. Fails on a linked solana/ crate outside the solana/
# workspace, whose own path dependencies the graph cannot follow.
linked_crates() {
  cargo metadata --format-version 1 --no-deps --manifest-path "$1/Cargo.toml" |
    jq -r --argjson graph "$solana_graph" --arg root "$root" '
      def grow: [.[], (.[] | $graph[.] // [] | .[])] | unique;
      [.packages[].dependencies[] | select(.path != null) | .path] | unique
      | until(grow == .; grow)
      | .[] | select(startswith($root + "solana/"))
      | if $graph[.] then ltrimstr($root)
        else error("\(ltrimstr($root)) is not a member of solana/Cargo.toml") end'
}

# The entries of a paths-filter block, one per line, or of `on.pull_request.paths` when no
# filter is named.
filter_entries() {
  local workflow=$workflows/$1 filter=$2
  if [[ -z $filter ]]; then
    yq '.on.pull_request.paths[]' "$workflow"
  else
    yq '.jobs[].steps[]? | select(.uses // "" | test("paths-filter")) | .with.filters' "$workflow" |
      yq ".\"$filter\" // [] | .[]"
  fi
}

# Whether one of the entries is a `dir/**` glob at or above the path.
covered() {
  local path=$1 entries=$2 entry
  while IFS= read -r entry; do
    if [[ $entry == *'/**' && $path/ == "${entry%\*\*}"* ]]; then
      return 0
    fi
  done <<<"$entries"
  return 1
}

failures=0
# check WORKFLOW FILTER CONSUMER_DIR CARGO TESTS
#   FILTER: the paths-filter key, or '' for `on.pull_request.paths`.
#   CARGO: whether CONSUMER_DIR is a Cargo workspace whose solana/ crates the filter must name.
#   TESTS: whether the filter gates a job that compiles the consumer's tests, including
#     `clippy --all-targets`, which must run when a fixture they load changes.
check() {
  local workflow=$1 filter=$2 consumer=$3 cargo=$4 tests=$5 entries required="" path
  entries=$(filter_entries "$workflow" "$filter")
  if [[ -z $entries ]]; then
    echo "$workflow: no filter ${filter:-on.pull_request.paths}" >&2
    failures=$((failures + 1))
    return
  fi
  if [[ $cargo == cargo ]]; then
    required=$(linked_crates "$consumer")
  fi
  if [[ $tests == tests ]] &&
    grep -rqF "$fixtures/" "$consumer" --include='*.rs' --include='*.ts' --exclude-dir=node_modules --exclude-dir=target; then
    required+=$'\n'$fixtures
  fi
  while IFS= read -r path; do
    [[ -n $path ]] || continue
    if ! covered "$path" "$entries"; then
      echo "$workflow (${filter:-on.pull_request.paths}): misses $path/**, which $consumer uses" >&2
      failures=$((failures + 1))
    fi
  done <<<"$required"
}

check coprocessor-cargo-tests.yml rust-files coprocessor/fhevm-engine cargo tests
check coprocessor-cargo-clippy.yml rust-files coprocessor/fhevm-engine cargo tests
check coprocessor-dependency-analysis.yml rust-files coprocessor/fhevm-engine cargo -
check kms-connector-tests.yml connector kms-connector cargo tests
check kms-connector-dependency-analysis.yml rust-files kms-connector cargo -
check relayer-tests.yaml rust-files relayer cargo tests
check relayer-dependency-analysis.yml rust-files relayer cargo -
check js-sdk-tests.yml '' sdk/js-sdk - tests

if ((failures > 0)); then
  echo "$failures path filter entries are missing." >&2
  exit 1
fi
echo "Every consumer path filter covers the solana/ paths its consumer reads."
