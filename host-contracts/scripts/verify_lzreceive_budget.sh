#!/usr/bin/env bash
#
# Exhaustively verify the fitted lzReceive gas budget (base + perHandle*n + perByte*len) is an
# upper bound on real measured gas across the full (nHandles, payloadLen) domain.
#
# It runs HandlesReceiverBudgetVerifierExample once PER PAYLOAD SUB-RANGE, in a SEPARATE forge
# process. A single step=1 run (32 x 10001 ~= 320k real lzReceive calls) accumulates enough
# host-side RAM (fork journaling, state snapshots, buffers) to get OOM-killed ("zsh: killed").
# Splitting into short-lived processes releases that memory between batches.
#
# Required env (same wiring as the profiler/verifier):
#   PROFILE_RPC_URL PROFILE_RECEIVER PROFILE_SENDER PROFILE_SRC_EID PROFILE_DST_EID
#   VERIFY_BASE_GAS VERIFY_PER_HANDLE_GAS VERIFY_PER_PAYLOAD_BYTE_GAS
#
# Optional env (defaults in parens):
#   VERIFY_MIN_PAYLOAD (0)   VERIFY_MAX_PAYLOAD (10000)   BATCH_WIDTH (50)
#   VERIFY_MIN_HANDLES (1)   VERIFY_MAX_HANDLES (32)
#   PROFILE_ENDPOINT (script default)   PROFILE_RUNS (1)
#   GAS_LIMIT (9223372036854775807)     FORGE_VERBOSITY (-vv)
#
# Each batch covers ALL handle counts x BATCH_WIDTH payload lengths at step=1. The script stops
# at the first batch that reports an under-budget cell (forge exits non-zero -> the verifier
# reverted), so a clean finish means every cell in [MIN_PAYLOAD, MAX_PAYLOAD] passed.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

require() {
  local name="$1"
  if [ -z "${!name:-}" ]; then
    echo "ERROR: required env var $name is not set." >&2
    exit 2
  fi
}

for v in PROFILE_RPC_URL PROFILE_RECEIVER PROFILE_SENDER PROFILE_SRC_EID PROFILE_DST_EID \
         VERIFY_BASE_GAS VERIFY_PER_HANDLE_GAS VERIFY_PER_PAYLOAD_BYTE_GAS; do
  require "$v"
done

MIN_PAYLOAD="${VERIFY_MIN_PAYLOAD:-0}"
MAX_PAYLOAD="${VERIFY_MAX_PAYLOAD:-10000}"
BATCH_WIDTH="${BATCH_WIDTH:-50}"
MIN_BATCH_WIDTH="${MIN_BATCH_WIDTH:-25}"
GAS_LIMIT="${GAS_LIMIT:-9223372036854775807}"
FORGE_VERBOSITY="${FORGE_VERBOSITY:--vv}"
TARGET="scripts/HandlesReceiverBudgetVerifier.s.sol:HandlesReceiverBudgetVerifierExample"

if [ "$BATCH_WIDTH" -lt 1 ] || [ "$MIN_BATCH_WIDTH" -lt 1 ]; then
  echo "ERROR: BATCH_WIDTH and MIN_BATCH_WIDTH must be >= 1." >&2
  exit 2
fi

echo "========================================================="
echo "Exhaustive (step=1) lzReceive budget verification"
echo "  payload range : [$MIN_PAYLOAD, $MAX_PAYLOAD]"
echo "  batch width   : $BATCH_WIDTH (auto-subdivides to >= $MIN_BATCH_WIDTH on OOM)"
echo "  handles       : ${VERIFY_MIN_HANDLES:-1}..${VERIFY_MAX_HANDLES:-32}"
echo "  formula       : $VERIFY_BASE_GAS + $VERIFY_PER_HANDLE_GAS*n + $VERIFY_PER_PAYLOAD_BYTE_GAS*len"
echo "========================================================="

# Runs one payload sub-range in its own forge process. Returns:
#   0  -> all cells in [lo,hi] within budget
#   1  -> verifier reverted (genuine under-budget cell) OR another forge error -> abort
#   2  -> OOM-killed even at MIN_BATCH_WIDTH -> abort
# On an OOM kill (forge killed by a signal, exit >= 128) for a range wider than
# MIN_BATCH_WIDTH, the range is split in half and each half retried in a fresh process.
process_range() {
  local lo="$1" hi="$2"
  local width=$(( hi - lo + 1 ))

  echo ""
  echo "----- payloadLen in [$lo, $hi] (step=1, width=$width) -----"

  set +e
  VERIFY_MIN_PAYLOAD="$lo" \
  VERIFY_MAX_PAYLOAD="$hi" \
  VERIFY_PAYLOAD_STEP=1 \
  forge script "$TARGET" "$FORGE_VERBOSITY" --gas-limit "$GAS_LIMIT"
  local code=$?
  set -e

  if [ "$code" -eq 0 ]; then
    return 0
  fi

  if [ "$code" -ge 128 ]; then
    # Killed by a signal (137 = 128 + SIGKILL = OOM). NOT a budget failure.
    echo "WARN: forge was killed (exit $code, almost certainly OOM) on [$lo, $hi] width=$width." >&2
    if [ "$width" -le "$MIN_BATCH_WIDTH" ]; then
      echo "ERROR: still OOM at minimum width $MIN_BATCH_WIDTH on [$lo, $hi]." >&2
      echo "       Lower MIN_BATCH_WIDTH, close other apps, or run on a higher-RAM machine." >&2
      return 2
    fi
    local mid=$(( lo + width / 2 - 1 ))
    echo "       OOM is not a budget violation; subdividing into [$lo, $mid] and [$((mid + 1)), $hi]." >&2
    process_range "$lo" "$mid" || return $?
    process_range "$((mid + 1))" "$hi" || return $?
    return 0
  fi

  # Non-zero, not a signal: the verifier reverted (under-budget cell) or a config/RPC error.
  echo "FAIL: forge exited $code on payloadLen in [$lo, $hi] -- verifier reverted (under-budget)" >&2
  echo "      or a config/RPC error. See UNDER-BUDGET / RESULT: FAIL lines above for (n, len)." >&2
  return 1
}

lo="$MIN_PAYLOAD"
while [ "$lo" -le "$MAX_PAYLOAD" ]; do
  hi=$(( lo + BATCH_WIDTH - 1 ))
  if [ "$hi" -gt "$MAX_PAYLOAD" ]; then
    hi="$MAX_PAYLOAD"
  fi

  set +e
  process_range "$lo" "$hi"
  rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    exit "$rc"
  fi

  if [ "$hi" -ge "$MAX_PAYLOAD" ]; then
    break
  fi
  lo=$(( hi + 1 ))
done

echo ""
echo "========================================================="
echo "PASS: every cell with payloadLen in [$MIN_PAYLOAD, $MAX_PAYLOAD]"
echo "      (all handle counts, step=1) is within budget."
echo "========================================================="
