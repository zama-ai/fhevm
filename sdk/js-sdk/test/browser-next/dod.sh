#!/usr/bin/env bash
set -uo pipefail

# Definition-of-done for browser-next: bring the infra up once, run a set of cells
# (each = a spec + a NEXT_PUBLIC_* config), then tear it down. Cells are DATA (the
# CELLS table below), so they can be filtered precisely — run one cell, one spec, one
# thread mode, etc. — instead of the full (slow) suite. Runs all matching cells even
# if one fails; exits non-zero if any failed.
#
#   ./dod.sh --help
#
# Requires foundry (anvil/cast/forge) on PATH.

BN_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" # test/browser-next
INFRA_DIR="$(cd "$BN_DIR/../infra" && pwd)"
PW=(npx playwright test --config playwright.config.ts --reporter=line)

# ──────────────────────────────────────────────────────────────────────────────
# Cells:  spec | lib | threads | coop | wasmLoad | module | description
# Empty field = leave that env var unset (its default applies; COOP defaults to 1).
# ──────────────────────────────────────────────────────────────────────────────
CELLS=(
  "gw-skeleton.spec.ts||||||skeleton (gateway rpc + relayer keys, same-origin under COOP/COEP)"
  "encrypt.spec.ts|viem|st||||encrypt + viem (st)"
  "encrypt.spec.ts|ethers|st||||encrypt + ethers (st)"
  "encrypt.spec.ts|viem|mt|1|||encrypt + viem (mt, coop) — multi-threaded TFHE"
  "encrypt.spec.ts|viem|mt|0|||encrypt + viem (mt, no-coop) — must degrade to ST"
  "coexist.spec.ts|viem|st||||coexistence legacy + current + cleartext (st)"
  "coexist.spec.ts|viem|mt|1|||coexistence legacy + current + cleartext (mt, coop)"
  "encrypt-ssr.spec.ts|viem|st||||ssr-node + viem (st)"
  "encrypt-ssr.spec.ts|viem|mt||||ssr-node + viem (mt) — server-side worker_threads"
  "encrypt-ssr.spec.ts|ethers|st||||ssr-node + ethers (st)"
  "encrypt-edge.spec.ts|viem|st||||ssr-edge + viem (st) — edge runtime, JS-inflate WASM"
  "encrypt-edge.spec.ts|viem|mt|1|||ssr-edge + viem (mt) — must degrade to ST (no edge worker backend)"
  "encrypt-mixed.spec.ts|viem|st||||mixed ssr + csr legs (st)"
  "encrypt-mixed.spec.ts|viem|mt|1|||mixed ssr + csr legs (mt, coop) — both legs multi-threaded"
  "wasm-load.spec.ts|viem|st||verified-blob||wasm-load verified-blob (st)"
  "wasm-load.spec.ts|viem|mt|1|verified-blob||wasm-load verified-blob (mt, coop) — verified worker URL"
  "wasm-load.spec.ts|viem|st||trusted-direct-url||wasm-load trusted-direct-url (st)"
  "wasm-load.spec.ts|viem|mt|1|trusted-direct-url||wasm-load trusted-direct-url (mt, coop) — new Worker(url), no verify"
  "wasm-load.spec.ts|viem|st||precheck-direct-url||wasm-load precheck-direct-url (st)"
  "wasm-load.spec.ts|viem|mt|1|precheck-direct-url||wasm-load precheck-direct-url (mt, coop) — precheck + new Worker(url)"
  "wasm-load.spec.ts|viem|st||auto||wasm-load auto (st)"
  "wasm-load.spec.ts|viem|mt|1|auto||wasm-load auto (mt, coop)"
  "module.spec.ts|viem||||kms|module kms: TKMS keygen — runs the TKMS wasm (viem)"
  "module.spec.ts|ethers||||kms|module kms: TKMS keygen (ethers)"
  "module.spec.ts|viem||||cleartext|module cleartext: mock runtime — encrypt + transport keypair (viem)"
  "module.spec.ts|ethers||||cleartext|module cleartext: mock runtime (ethers)"
)
N=${#CELLS[@]}

# ──────────────────────────────────────────────────────────────────────────────
# CLI
# ──────────────────────────────────────────────────────────────────────────────
INDEX_FILTER=""
SPEC_FILTER=""
LIB_FILTER=""
THREADS_FILTER=""
COOP_FILTER=""
WASMLOAD_FILTER=""
MODULE_FILTER=""
BROWSERS="" # space-separated; empty → chromium only
LIST=0
KEEP_INFRA=0
NO_UP=0

usage() {
  cat <<EOF
Usage: ./dod.sh [filters] [options]

Runs the browser-next definition-of-done cells (currently $N). With no filters, runs
them all. Filters combine with AND so you can test precisely without the slow full run.

Filters:
  -i, --index <list>      Only these cell indices. Comma/range ok:  18  |  15,16  |  15-18
      --test-index <list> Alias for --index. Repeatable.
  -s, --spec <name>       Only cells whose spec matches (substring):
                            skeleton encrypt coexist ssr edge mixed wasm-load module
      --lib <viem|ethers> Only this lib.
      --mt | --st         Only this thread mode.
      --coop | --no-coop  Only COOP-on / COOP-off cells (COEP is coupled to COOP;
                          --coep / --no-coep are accepted as aliases).
      --wasm-load <mode>  Only this load mode:
                            embedded-base64 verified-blob trusted-direct-url
                            precheck-direct-url auto
      --module <name>     Only this module cell:  kms | cleartext

Browsers (default: chromium only):
      --firefox           Also run on Firefox.
      --webkit | --safari Also run on WebKit (Safari's engine).
      --browser <name>    Add a browser: chromium | firefox | webkit. Repeatable.
      --all-browsers      Run on chromium + firefox + webkit.
                          (Install engines first: npx playwright install firefox webkit)

Options:
  -l, --list              List matching cells and exit (no run).
      --no-up             Don't start infra (assume 'up.sh -d' is already running).
      --keep-infra        Don't tear down infra at the end (reuse it for the next run).
  -h, --help              Show this help.

Examples:
  ./dod.sh                                 # full suite (all $N cells)
  ./dod.sh --index 18                      # only cell 18
  ./dod.sh --index 15-22                   # only the wasm-load cells
  ./dod.sh --spec wasm-load --mt           # all multi-threaded wasm-load cells
  ./dod.sh --spec encrypt --lib ethers     # ethers encrypt cells
  ./dod.sh --module kms                    # the TKMS keygen cells
  ./dod.sh --mt --no-coop                  # MT cells with COOP off
  ./dod.sh --index 4 --all-browsers        # cell 4 on chromium + firefox + webkit
  ./dod.sh --spec encrypt --firefox        # encrypt cells on chromium + firefox
  ./dod.sh --list                          # show every cell's index + config
  ./dod.sh --list --spec ssr               # list just the ssr cells

  # Fast iteration — bring infra up once, then re-run single cells with --no-up:
  ../infra/up.sh -d
  ./dod.sh --index 18 --no-up
  ./dod.sh --module cleartext --no-up
  ../infra/down.sh
EOF
}

# "15-18,20 22" -> "15 16 17 18 20 22"
expand_indices() {
  local out="" tok lo hi i
  for tok in $(echo "$1" | tr ',' ' '); do
    if [[ "$tok" == *-* ]]; then
      lo="${tok%-*}"
      hi="${tok#*-}"
      for ((i = lo; i <= hi; i++)); do out+="$i "; done
    else
      out+="$tok "
    fi
  done
  echo "$out"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -i | --index | --test-index)
      INDEX_FILTER+="$(expand_indices "${2:?--index requires a value}") "
      shift 2
      ;;
    -s | --spec)
      SPEC_FILTER="${2:?--spec requires a value}"
      shift 2
      ;;
    --lib)
      LIB_FILTER="${2:?--lib requires a value}"
      shift 2
      ;;
    --mt) THREADS_FILTER="mt" && shift ;;
    --st) THREADS_FILTER="st" && shift ;;
    --coop | --coep) COOP_FILTER="1" && shift ;;
    --no-coop | --no-coep) COOP_FILTER="0" && shift ;;
    --wasm-load)
      WASMLOAD_FILTER="${2:?--wasm-load requires a value}"
      shift 2
      ;;
    --module)
      MODULE_FILTER="${2:?--module requires a value}"
      shift 2
      ;;
    --browser)
      BROWSERS+="${2:?--browser requires a value} "
      shift 2
      ;;
    --firefox) BROWSERS+="firefox " && shift ;;
    --webkit | --safari) BROWSERS+="webkit " && shift ;;
    --all-browsers) BROWSERS="chromium firefox webkit" && shift ;;
    -l | --list) LIST=1 && shift ;;
    --no-up) NO_UP=1 && shift ;;
    --keep-infra) KEEP_INFRA=1 && shift ;;
    -h | --help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      echo "Run './dod.sh --help' for usage." >&2
      exit 2
      ;;
  esac
done

# Build Playwright --project args from the selected browsers (default: chromium).
[[ -z "${BROWSERS// /}" ]] && BROWSERS="chromium"
PROJECT_ARGS=()
_seen=" "
for b in $BROWSERS; do
  case "$b" in
    chromium | firefox | webkit) ;;
    *)
      echo "Unknown browser: '$b' (use chromium | firefox | webkit)" >&2
      exit 2
      ;;
  esac
  case "$_seen" in *" $b "*) ;; *)
    PROJECT_ARGS+=(--project="$b")
    _seen+="$b "
    ;;
  esac
done

# Returns 0 if the cell at $1 (fields $2..$7) passes every active filter.
matches() {
  local idx="$1" spec="$2" lib="$3" threads="$4" coop="$5" wasmload="$6" module="$7"
  local eff_coop="${coop:-1}" # unset COOP defaults to 1 (next.config)
  if [[ -n "$INDEX_FILTER" ]]; then
    case " $INDEX_FILTER " in *" $idx "*) ;; *) return 1 ;; esac
  fi
  if [[ -n "$SPEC_FILTER" ]]; then
    case "$spec" in *"$SPEC_FILTER"*) ;; *) return 1 ;; esac
  fi
  [[ -n "$LIB_FILTER" && "$lib" != "$LIB_FILTER" ]] && return 1
  [[ -n "$THREADS_FILTER" && "$threads" != "$THREADS_FILTER" ]] && return 1
  [[ -n "$COOP_FILTER" && "$eff_coop" != "$COOP_FILTER" ]] && return 1
  [[ -n "$WASMLOAD_FILTER" && "$wasmload" != "$WASMLOAD_FILTER" ]] && return 1
  [[ -n "$MODULE_FILTER" && "$module" != "$MODULE_FILTER" ]] && return 1
  return 0
}

# ──────────────────────────────────────────────────────────────────────────────
# --list
# ──────────────────────────────────────────────────────────────────────────────
if [[ "$LIST" -eq 1 ]]; then
  printf "%-4s %-13s %-7s %-4s %-5s %-20s %-10s %s\n" "IDX" "SPEC" "LIB" "THR" "COOP" "WASM_LOAD" "MODULE" "DESCRIPTION"
  idx=0
  for rec in "${CELLS[@]}"; do
    idx=$((idx + 1))
    IFS='|' read -r spec lib threads coop wasmload module desc <<<"$rec"
    matches "$idx" "$spec" "$lib" "$threads" "$coop" "$wasmload" "$module" || continue
    printf "%-4s %-13s %-7s %-4s %-5s %-20s %-10s %s\n" \
      "$idx" "${spec%.spec.ts}" "${lib:--}" "${threads:--}" "${coop:-(1)}" "${wasmload:--}" "${module:--}" "$desc"
  done
  exit 0
fi

# Kill the Next dev server and wait for the port to free. Needed before every cell
# because NEXT_PUBLIC_* (lib, thread mode, wasm-load, module) is inlined at dev-server
# startup, so each cell needs a fresh server. SIGKILL + 'pkill next' because 'next dev
# --turbopack' spawns children that keep 3334 bound under a graceful signal; then WAIT
# until the port is actually free so the next webServer can't hit EADDRINUSE.
free_next() {
  local pids
  pids="$(lsof -ti tcp:3334 2>/dev/null || true)"
  if [[ -n "$pids" ]]; then
    # shellcheck disable=SC2086
    kill -9 $pids 2>/dev/null || true
  fi
  pkill -9 -f 'next(-server)? dev' 2>/dev/null || true
  for _ in $(seq 1 60); do
    lsof -ti tcp:3334 >/dev/null 2>&1 || return 0
    sleep 0.25
  done
  echo "⚠️  port 3334 still in use after free_next" >&2
}

# ──────────────────────────────────────────────────────────────────────────────
# Run
# ──────────────────────────────────────────────────────────────────────────────
fail=0
ran=0

if [[ "$NO_UP" -eq 0 ]]; then
  echo "▶ starting infra (up.sh -d)..."
  if ! "$INFRA_DIR/up.sh" -d; then
    echo "❌ infra failed to start" >&2
    exit 1
  fi
fi

cd "$BN_DIR"
echo "▶ browsers: ${PROJECT_ARGS[*]//--project=/}"

idx=0
for rec in "${CELLS[@]}"; do
  idx=$((idx + 1))
  IFS='|' read -r spec lib threads coop wasmload module desc <<<"$rec"
  matches "$idx" "$spec" "$lib" "$threads" "$coop" "$wasmload" "$module" || continue
  ran=$((ran + 1))

  echo ""
  echo "▶ [$idx/$N] $desc"
  free_next

  cell_env=()
  [[ -n "$lib" ]] && cell_env+=("FHEVM_TEST_LIB=$lib")
  [[ -n "$threads" ]] && cell_env+=("FHEVM_TEST_THREADS=$threads")
  [[ -n "$coop" ]] && cell_env+=("FHEVM_TEST_COOP=$coop")
  [[ -n "$wasmload" ]] && cell_env+=("FHEVM_TEST_WASM_LOAD=$wasmload")
  [[ -n "$module" ]] && cell_env+=("FHEVM_TEST_MODULE=$module")
  env ${cell_env[@]+"${cell_env[@]}"} "${PW[@]}" "${PROJECT_ARGS[@]}" "specs/$spec" || fail=1
done

if [[ "$ran" -eq 0 ]]; then
  echo "No cells matched the filters." >&2
  [[ "$NO_UP" -eq 0 && "$KEEP_INFRA" -eq 0 ]] && "$INFRA_DIR/down.sh"
  exit 1
fi

if [[ "$NO_UP" -eq 0 && "$KEEP_INFRA" -eq 0 ]]; then
  echo ""
  echo "▶ tearing down infra (down.sh)..."
  "$INFRA_DIR/down.sh"
fi

echo ""
if [[ "$fail" -eq 0 ]]; then
  echo "✅ browser-next DoD passed ($ran/$N cell(s))"
else
  echo "❌ browser-next DoD failed"
fi
exit "$fail"
