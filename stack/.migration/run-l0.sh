#!/usr/bin/env bash
# run-l0.sh — L0 acceptance gate: helm-render all charts for a topology case,
#             normalize the output, assert document counts, record the golden.
#
# USAGE
#   bash run-l0.sh <case> [--record]
#
#   <case>       topology case name: "default" or "two-of-three"
#   --record     re-record the golden (overwrite stack/.migration/golden/<case>/)
#                WITHOUT --record the script diffs against the stored golden.
#
# RENDER TOPOLOGY
#   For each case the render expands into:
#     - 1× anvil-node release
#     - 1× contracts release
#     - N× coprocessor releases (coprocessor-0 … coprocessor-N-1)
#       N = _topology.numCoprocessors in the case values file
#     - 1× kms-connector release (uses kms-connector-<case>.yaml)
#     - 1× listener release
#
# DOCUMENT COUNT CONTRACT
#   The render asserts total YAML document count (number of "---" separators)
#   matches the expected count for the topology.  Expected counts are recorded
#   at golden-record time (--record) in golden/<case>/doc-count and verified
#   on every subsequent run.
#
# EXIT CODES
#   0  all assertions passed (or golden recorded successfully)
#   1  assertion failure or helm error

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CHARTS="${REPO_ROOT}/charts"
VALUES="${REPO_ROOT}/stack/values"
MIGRATION="${REPO_ROOT}/stack/.migration"
NORMALIZE="${MIGRATION}/normalize.sh"

CASE="${1:-}"
RECORD=false
for arg in "$@"; do
  [[ "${arg}" == "--record" ]] && RECORD=true
done
if [[ -z "${CASE}" ]] || [[ "${CASE}" == "--record" ]]; then
  echo "USAGE: $0 <case> [--record]   (case: default | two-of-three)" >&2
  exit 1
fi

GOLDEN_DIR="${MIGRATION}/golden/${CASE}"

# ---------------------------------------------------------------------------
# Read numCoprocessors from the case values file.
# The values file contains:
#   _topology:
#     numCoprocessors: N
# ---------------------------------------------------------------------------
VALUES_FILE="${VALUES}/${CASE}.yaml"
if [[ ! -f "${VALUES_FILE}" ]]; then
  echo "ERROR: values file not found: ${VALUES_FILE}" >&2
  exit 1
fi

NUM_COPROS=$(grep -A5 '^_topology:' "${VALUES_FILE}" | grep 'numCoprocessors:' | awk '{print $2}')
if [[ -z "${NUM_COPROS}" ]]; then
  echo "ERROR: _topology.numCoprocessors not found in ${VALUES_FILE}" >&2
  exit 1
fi
echo "Topology: case=${CASE}  numCoprocessors=${NUM_COPROS}"

# kms-connector uses a separate per-topology file (avoids commonConfig key collision)
KMS_CONNECTOR_VALUES="${VALUES}/kms-connector-${CASE}.yaml"
if [[ ! -f "${KMS_CONNECTOR_VALUES}" ]]; then
  echo "ERROR: kms-connector values file not found: ${KMS_CONNECTOR_VALUES}" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Render all charts into a single raw stream.
# ---------------------------------------------------------------------------
TMP_RAW="$(mktemp /tmp/l0-raw-XXXXXX.yaml)"
TMP_NORM="$(mktemp /tmp/l0-norm-XXXXXX.txt)"
trap "rm -f '${TMP_RAW}' '${TMP_NORM}'" EXIT

echo "--- Rendering anvil-node ---"
helm template anvil-node "${CHARTS}/anvil-node" \
  -f "${VALUES}/kind-local.yaml" \
  >> "${TMP_RAW}" 2>&1

echo "--- Rendering contracts ---"
helm template host-contracts "${CHARTS}/contracts" \
  -f "${VALUES}/kind-local.yaml" \
  -f "${VALUES_FILE}" \
  >> "${TMP_RAW}" 2>&1

echo "--- Rendering coprocessor (${NUM_COPROS} release(s)) ---"
for i in $(seq 0 $((NUM_COPROS - 1))); do
  echo "    coprocessor-${i}"
  helm template "coprocessor-${i}" "${CHARTS}/coprocessor" \
    -f "${VALUES}/kind-local.yaml" \
    -f "${VALUES_FILE}" \
    >> "${TMP_RAW}" 2>&1
done

echo "--- Rendering kms-connector ---"
helm template kms-connector "${CHARTS}/kms-connector" \
  -f "${KMS_CONNECTOR_VALUES}" \
  >> "${TMP_RAW}" 2>&1

echo "--- Rendering listener ---"
helm template listener "${CHARTS}/listener" \
  -f "${VALUES}/kind-local.yaml" \
  >> "${TMP_RAW}" 2>&1

# ---------------------------------------------------------------------------
# Count documents (lines that are exactly "---").
# ---------------------------------------------------------------------------
RAW_DOC_COUNT=$(grep -c '^---$' "${TMP_RAW}" || true)
echo ""
echo "Raw document count: ${RAW_DOC_COUNT}"

# ---------------------------------------------------------------------------
# Normalize (strip volatile fields, sort).
# ---------------------------------------------------------------------------
bash "${NORMALIZE}" < "${TMP_RAW}" > "${TMP_NORM}"
NORM_LINES=$(wc -l < "${TMP_NORM}")
echo "Normalized lines: ${NORM_LINES}"

SHA256=$(sha256sum "${TMP_NORM}" | awk '{print $1}')
echo "sha256: ${SHA256}"

# ---------------------------------------------------------------------------
# Record or compare.
# ---------------------------------------------------------------------------
if [[ "${RECORD}" == "true" ]]; then
  mkdir -p "${GOLDEN_DIR}"
  cp "${TMP_NORM}" "${GOLDEN_DIR}/manifests.norm"
  echo "${SHA256}" > "${GOLDEN_DIR}/sha256"
  echo "${RAW_DOC_COUNT}" > "${GOLDEN_DIR}/doc-count"
  echo ""
  echo "RECORDED golden for case '${CASE}':"
  echo "  ${GOLDEN_DIR}/manifests.norm  (${NORM_LINES} lines)"
  echo "  ${GOLDEN_DIR}/sha256          ${SHA256}"
  echo "  ${GOLDEN_DIR}/doc-count       ${RAW_DOC_COUNT}"
else
  FAIL=0

  if [[ ! -f "${GOLDEN_DIR}/sha256" ]]; then
    echo "ERROR: no golden found at ${GOLDEN_DIR}/sha256 — run with --record first" >&2
    exit 1
  fi
  if [[ ! -f "${GOLDEN_DIR}/doc-count" ]]; then
    echo "ERROR: no doc-count golden at ${GOLDEN_DIR}/doc-count — run with --record first" >&2
    exit 1
  fi

  GOLDEN_SHA=$(cat "${GOLDEN_DIR}/sha256")
  GOLDEN_DOCS=$(cat "${GOLDEN_DIR}/doc-count")

  # Document count assertion
  if [[ "${RAW_DOC_COUNT}" -ne "${GOLDEN_DOCS}" ]]; then
    echo "FAIL: document count mismatch: got ${RAW_DOC_COUNT}, expected ${GOLDEN_DOCS}" >&2
    FAIL=1
  else
    echo "PASS: document count = ${RAW_DOC_COUNT}"
  fi

  # sha256 assertion
  if [[ "${SHA256}" != "${GOLDEN_SHA}" ]]; then
    echo "FAIL: sha256 mismatch:" >&2
    echo "  got:      ${SHA256}" >&2
    echo "  expected: ${GOLDEN_SHA}" >&2
    FAIL=1
  else
    echo "PASS: sha256 matches golden"
  fi

  if [[ "${FAIL}" -ne 0 ]]; then
    echo ""
    echo "Diff (golden vs current normalized):"
    diff "${GOLDEN_DIR}/manifests.norm" "${TMP_NORM}" || true
    exit 1
  fi

  echo ""
  echo "PASS: case '${CASE}' — all L0 assertions green"
fi
