#!/usr/bin/env bash
# Blue/Green QA checkpoints: pass/fail lines per operator and host chain for each phase of a
# manual round, read from the operator databases (plus RPCs for block timing). Exit 1 on any FAIL.
#
#   bg-checkpoints.sh baseline       Blue live at the Blue version, no upgrade state, no Green
#                                    schema, Blue fleet ready and ingesting; schema level; traffic clean
#   bg-checkpoints.sh dry-run        one GCS row per chain in UpgradeActivated/DryRunStarted with the
#                                    same proposal on every operator, gateway dry run started, synthetic
#                                    anchors present, state hashes equal across operators, Green
#                                    shadowing work
#   bg-checkpoints.sh window-timing  real timestamps of the window blocks per chain, cross-chain skew,
#                                    skew against WINDOW_START if given, gateway anchor reachability
#   bg-checkpoints.sh cutover        versioning at the Green version, LIVE/completed everywhere, Green
#                                    schema dropped, every Green replica running at the Green version,
#                                    Blue paused, synthetic rows gone
#   bg-checkpoints.sh post           state hashes still agree, versioning
#                                    unchanged, traffic counters clean (run bg-traffic.sh verify too)
#
# Usage: NAMESPACE=<ns> bash ci/preview-env/scripts/bg/bg-checkpoints.sh <phase>
# Env: NAMESPACE (required); GCS_STACK_VERSION (default: gcs overlay); BCS_STACK_VERSION (default:
#      what the live consumer binary prints); WINDOW_START (ISO time passed to the propose task,
#      optional); WINDOW_ALIGN_SECS (12); GATEWAY_RPC_URL (optional, gateway checks);
#      GREEN_SLOT (-gcs) / LIVE_RELEASE_SUFFIX ("") / LIVE_CONSENSUS_VERSION (1) - see bg-green.sh;
#      a second round in the same env inverts the slots and starts one consensus version higher.
set -euo pipefail

phase="${1:?usage: bg-checkpoints.sh baseline|dry-run|window-timing|cutover|post}"
: "${NAMESPACE:?}"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
GCS_STACK_VERSION="${GCS_STACK_VERSION:-$(yq -r '.commonConfig.stackVersion' "${root}/ci/preview-env/coprocessor/values-coprocessor-gcs-e2e.yaml")}"
schema="gcs-${GCS_STACK_VERSION}"
# Same slot parameters as bg-green.sh: which helm slot holds the incoming fleet and which the live one.
GREEN_SLOT="${GREEN_SLOT--gcs}"
LIVE_RELEASE_SUFFIX="${LIVE_RELEASE_SUFFIX:-}"
LIVE_CONSENSUS_VERSION="${LIVE_CONSENSUS_VERSION:-1}"
WINDOW_ALIGN_SECS="${WINDOW_ALIGN_SECS:-12}"
# verify_proofs.contract_address is text holding 0x-prefixed hex, not bytea.
SYNTHETIC_INPUT_CONTRACT='0x5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a'

failed=0
pass() { echo "[PASS] $*"; }
fail() { echo "[FAIL] $*"; failed=1; }
info() { echo "[INFO] $*"; }
check() { # check <message> <test-expression...>   e.g. check "x is 1" "${x}" = 1
  local msg="$1"; shift
  if test "$@"; then pass "${msg}"; else fail "${msg}"; fi
}

psql_party() {
  local party="$1" sql="$2"
  kubectl exec -n "${NAMESPACE}" "postgres-coprocessor-${party}-0" -- \
    env PGPASSWORD=zama psql -U zama -d fhevm_e2e -v ON_ERROR_STOP=1 -tAqc "${sql}"
}
nb_parties=$(helm list -n "${NAMESPACE}" -o json | jq --arg re "^coprocessor-[0-9]+${LIVE_RELEASE_SUFFIX}\$" '[.[] | select(.name | test($re))] | length')
[[ "${nb_parties}" -gt 0 ]] || { echo "::error::no coprocessor releases match ^coprocessor-[0-9]+${LIVE_RELEASE_SUFFIX}\$ in ${NAMESPACE}; set LIVE_RELEASE_SUFFIX for a second round" >&2; exit 1; }
parties=$(seq 1 "${nb_parties}")
chains=$(psql_party 1 "SELECT chain_id FROM host_chains ORDER BY chain_id;")
nb_chains=$(wc -w <<<"${chains}" | tr -d ' ')
version_mm() { sed -E 's/^v//; s/^([0-9]+\.[0-9]+).*/\1/' <<<"$1"; }

blue_version() {
  kubectl exec -n "${NAMESPACE}" "deploy/coprocessor-1${LIVE_RELEASE_SUFFIX}-host-listener-consumer" -- host_listener --stack-version 2>/dev/null || echo "${BCS_STACK_VERSION:-0.14.0}"
}
# Deployments of a fleet, anchored on the component name so an empty slot suffix cannot swallow the
# other fleet's releases (coprocessor-1- is a prefix of coprocessor-1-gcs-).
fleet_ready() { # fleet_ready <blue|green> <party>
  local fleet="$1" party="$2" sfx pattern
  sfx="${LIVE_RELEASE_SUFFIX}"; [[ "${fleet}" == green ]] && sfx="${GREEN_SLOT}"
  pattern="^coprocessor-(${party}|polygon-${party})${sfx}-(gw|host|sns|tfhe|tx|zk|upgrade|consensus)|^coprocessor-poller-(polygon-)?${party}${sfx}-host"
  kubectl get deploy -n "${NAMESPACE}" -o json \
    | jq -r --arg re "${pattern}" '[.items[] | select(.metadata.name | test($re))] | "\(map(select(.status.readyReplicas == .spec.replicas and .spec.replicas > 0)) | length)/\(length)"'
}
rpc_for_chain() { # host RPC URL from the env's rpc Secret
  local key
  case "$1" in 11155111|1) key=ethereum-rpc-url ;; 80002|137) key=polygon-rpc-url ;; *) return 1 ;; esac
  kubectl get secret -n "${NAMESPACE}" rpc -o jsonpath="{.data.${key}}" 2>/dev/null | base64 -d
}
block_ts() { cast block "$2" --field timestamp --rpc-url "$1" 2>/dev/null || true; }
iso() { date -u -r "$1" +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date -u -d "@$1" +%Y-%m-%dT%H:%M:%SZ; }

traffic_counters() { # from the bg-traffic state ConfigMaps, if any
  for cm in $(kubectl get configmap -n "${NAMESPACE}" -o name | grep "bg-traffic-state-" | sed 's#configmap/##'); do
    kubectl get configmap -n "${NAMESPACE}" "${cm}" -o json | jq -r --arg cm "${cm}" '.data["state.json"] | fromjson
      | "\($cm): iterations \(.counters.iterations) transfers \(.counters.transfers) mints \(.counters.mints) decrypts \(.counters.decrypts) mismatches \(.counters.decryptMismatches) failures \(.counters.failures)|\(.counters.decryptMismatches + .counters.failures)"'
  done
}

# Shared checks ------------------------------------------------------------------------------
# State hashes must agree across operators for the newest block every operator has hashed.
check_state_hash_agreement() { # <schema>
  local sch="$1"
  for chain in ${chains}; do
    local common="" hashes=""
    for i in ${parties}; do
      local m
      m=$(psql_party "${i}" "SELECT COALESCE(max(block_number), 0) FROM \"${sch}\".state_hash WHERE chain_id = ${chain};")
      [[ -z "${common}" || "${m}" -lt "${common}" ]] && common="${m}"
    done
    if [[ "${common}" == "0" ]]; then fail "chain ${chain}: no state hash in ${sch} on every operator yet"; continue; fi
    for i in ${parties}; do
      hashes+=" $(psql_party "${i}" "SELECT COALESCE(min(state_hash), '-') FROM \"${sch}\".state_hash WHERE chain_id = ${chain} AND block_number = ${common};")"
    done
    local distinct
    distinct=$(tr ' ' '\n' <<<"${hashes}" | sed '/^$/d' | sort -u | wc -l | tr -d ' ')
    check "chain ${chain}: state hash at block ${common} identical on ${nb_parties} operators (${distinct} distinct)" "${distinct}" = "1"
  done
}

echo "== bg-checkpoints ${phase}: ${NAMESPACE}, ${nb_parties} operators, chains ${chains//$'\n'/ }, Green ${GCS_STACK_VERSION}"

case "${phase}" in
baseline)
  bv=$(blue_version)
  for i in ${parties}; do
    v=$(psql_party "${i}" "SELECT stack_version||'/'||COALESCE(to_jsonb(v)->>'consensus_version','1') FROM versioning v;")
    check "party ${i}: versioning ${v} (live binary ${bv}, consensus ${LIVE_CONSENSUS_VERSION})" "$(version_mm "${v%/*}")" = "$(version_mm "${bv}")" -a "${v#*/}" = "${LIVE_CONSENSUS_VERSION}"
    n=$(psql_party "${i}" "SELECT count(*) FROM upgrade_state;")
    check "party ${i}: upgrade_state rows = ${n}" "${n}" = "0"
    g=$(psql_party "${i}" "SELECT count(*) FROM pg_namespace WHERE nspname LIKE 'gcs%';")
    # On the production path Green is migrated mid-round, so a Green schema is only a fault
    # while the Green release is still absent.
    if helm status "coprocessor-${i}${GREEN_SLOT}" -n "${NAMESPACE}" >/dev/null 2>&1; then
      info "party ${i}: Green schemas = ${g} (Green already installed)"
    else
      check "party ${i}: Green schemas = ${g}" "${g}" = "0"
    fi
    r=$(fleet_ready blue "${i}")
    check "party ${i}: Blue deployments ready ${r}" "${r%/*}" = "${r#*/}"
    c=$(psql_party "${i}" "SELECT count(DISTINCT chain_id) FROM host_chain_blocks_valid WHERE created_at > now() - interval '120 seconds';")
    check "party ${i}: chains ingested in the last 2 min = ${c}/${nb_chains}" "${c}" -ge "${nb_chains}"
    m=$(psql_party "${i}" "SELECT max(version) FROM _sqlx_migrations;")
    info "party ${i}: schema at migration ${m}"
  done
  while IFS='|' read -r line bad; do
    [[ -n "${line}" ]] || continue
    check "traffic ${line}" "${bad}" = "0"
  done <<<"$(traffic_counters)"
  ;;

dry-run)
  ref=""
  for i in ${parties}; do
    rows=$(psql_party "${i}" "SELECT host_chain_id||'|'||state||'|'||status||'|'||proposal_id||'|'||COALESCE(proposal_block::text,'-')||'|'||start_block||'|'||end_block||'|'||gw_start_block||'|'||gw_dry_run_started||'|'||host_consensus_reached||'|'||gw_consensus_reached||'|'||(octet_length(synthetic_txn_hashes)/32) FROM upgrade_state WHERE stack_role='GCS' ORDER BY host_chain_id;")
    n=$(grep -c . <<<"${rows}" || true)
    check "party ${i}: GCS upgrade_state rows = ${n}/${nb_chains}" "${n}" = "${nb_chains}"
    while IFS='|' read -r chain state status pid pblock sb eb gwsb gwdry hcons gcons nsyn; do
      [[ -n "${chain}" ]] || continue
      check "party ${i} chain ${chain}: state ${state}/${status} proposal ${pid} @${pblock} window ${sb}-${eb} gw ${gwsb}" "${state}" = "DryRunStarted" -o "${state}" = "UpgradeActivated"
      # `||` in the query renders booleans as true/false, not psql's bare t/f.
      [[ "${state}" == "DryRunStarted" ]] && check "party ${i} chain ${chain}: gateway dry run started = ${gwdry}" "${gwdry}" = "true"
      check "party ${i} chain ${chain}: synthetic host anchors injected = ${nsyn} (consensus latches host=${hcons} gw=${gcons})" "${nsyn}" -ge 1
      key="${chain}:${pid}:${pblock}:${sb}:${eb}:${gwsb}"
      if [[ -z "${ref}" ]]; then ref="${key}"; else
        [[ "${ref%%:*}" == "${chain}" ]] && check "chain ${chain}: same proposal/window on every operator" "${ref}" = "${key}"
      fi
    done <<<"${rows}"
    # Fail loudly on a query error; only a genuinely absent schema reports zero.
    if [[ "$(psql_party "${i}" "SELECT count(*) FROM pg_namespace WHERE nspname = '${schema}';")" == "0" ]]; then
      gsyn=0
    else
      gsyn=$(psql_party "${i}" "SELECT count(*) FROM \"${schema}\".verify_proofs WHERE lower(contract_address) = lower('${SYNTHETIC_INPUT_CONTRACT}');")
    fi
    check "party ${i}: synthetic gateway input in ${schema}.verify_proofs = ${gsyn} (0 also when the schema is missing)" "${gsyn}" -ge 1
    for chain in ${chains}; do
      cnt=$(psql_party "${i}" "SELECT count(*)||' ('||count(*) FILTER (WHERE is_completed)||' completed)' FROM \"${schema}\".computations WHERE host_chain_id = ${chain};" 2>/dev/null || echo "0")
      check "party ${i} chain ${chain}: Green shadow computations = ${cnt}" "${cnt%% *}" -ge 1
    done
    r=$(fleet_ready green "${i}")
    check "party ${i}: Green deployments ready ${r}" "${r%/*}" = "${r#*/}" -a "${r#*/}" != "0"
  done
  check_state_hash_agreement "${schema}"
  ;;

window-timing)
  starts=()
  for chain in ${chains}; do
    row=$(psql_party 1 "SELECT start_block||' '||end_block||' '||COALESCE(proposal_block::text,'0') FROM upgrade_state WHERE stack_role='GCS' AND host_chain_id=${chain};")
    [[ -n "${row}" ]] || { fail "chain ${chain}: no upgrade_state row"; continue; }
    read -r sb eb pb <<<"${row}"
    rpc=$(rpc_for_chain "${chain}") || { info "chain ${chain}: no RPC in the rpc Secret, skipping"; continue; }
    tip=$(cast block-number --rpc-url "${rpc}" 2>/dev/null || echo 0)
    ts_s=$(block_ts "${rpc}" "${sb}"); ts_e=$(block_ts "${rpc}" "${eb}"); ts_p=$(block_ts "${rpc}" "${pb}")
    info "chain ${chain}: proposal @${pb} ($( [[ -n "${ts_p}" ]] && iso "${ts_p}" || echo '-')), start ${sb} ($( [[ -n "${ts_s}" ]] && iso "${ts_s}" || echo "not mined, tip ${tip}")), end ${eb} ($( [[ -n "${ts_e}" ]] && iso "${ts_e}" || echo "not mined, tip ${tip}"))"
    [[ -n "${ts_s}" ]] && starts+=("${chain}:${ts_s}")
    if [[ -n "${WINDOW_START:-}" && -n "${ts_s}" ]]; then
      req=$(date -u -j -f %Y-%m-%dT%H:%M:%SZ "${WINDOW_START}" +%s 2>/dev/null || date -u -d "${WINDOW_START}" +%s)
      skew=$(( ts_s - req ))
      check "chain ${chain}: start block is ${skew}s from the requested ${WINDOW_START} (tolerance ${WINDOW_ALIGN_SECS}s)" "${skew#-}" -le "${WINDOW_ALIGN_SECS}"
    fi
  done
  if [[ ${#starts[@]} -ge 2 ]]; then
    min=""; max=""
    for s in "${starts[@]}"; do t=${s#*:}; [[ -z "${min}" || ${t} -lt ${min} ]] && min=${t}; [[ -z "${max}" || ${t} -gt ${max} ]] && max=${t}; done
    check "host windows open within $((max - min))s of each other (tolerance ${WINDOW_ALIGN_SECS}s)" "$((max - min))" -le "${WINDOW_ALIGN_SECS}"
  fi
  gwsb=$(psql_party 1 "SELECT min(gw_start_block) FROM upgrade_state WHERE stack_role='GCS';")
  if [[ -z "${gwsb}" ]]; then
    fail "gateway: no upgrade_state row, no gw_start_block"
  elif [[ -n "${GATEWAY_RPC_URL:-}" ]]; then
    gtip=$(cast block-number --rpc-url "${GATEWAY_RPC_URL}")
    check "gateway: tip ${gtip} vs gw_start_block ${gwsb} (anchor block ${gwsb}+1 $([[ "${gtip}" -ge $((gwsb + 1)) ]] && echo reached || echo "needs $((gwsb + 1 - gtip)) more gateway tx"))" "${gtip}" -ge $((gwsb + 1))
  else
    info "gateway: gw_start_block ${gwsb} (set GATEWAY_RPC_URL to compare with the gateway tip)"
  fi
  ;;

cutover)
  for i in ${parties}; do
    v=$(psql_party "${i}" "SELECT stack_version||'/'||COALESCE(to_jsonb(v)->>'consensus_version','1') FROM versioning v;")
    check "party ${i}: versioning ${v} (Green ${GCS_STACK_VERSION}, consensus >= 2)" "$(version_mm "${v%/*}")" = "$(version_mm "${GCS_STACK_VERSION}")" -a "${v#*/}" -ge 2
    rows=$(psql_party "${i}" "SELECT host_chain_id||'|'||state||'|'||status FROM upgrade_state WHERE stack_role='GCS' ORDER BY host_chain_id;")
    n=$(grep -c . <<<"${rows}" || true)
    check "party ${i}: GCS upgrade_state rows = ${n}/${nb_chains}" "${n}" = "${nb_chains}"
    while IFS='|' read -r chain state status; do
      [[ -n "${chain}" ]] || continue
      check "party ${i} chain ${chain}: ${state}/${status}" "${state}" = "LIVE" -a "${status}" = "completed"
    done <<<"${rows}"
    g=$(psql_party "${i}" "SELECT count(*) FROM pg_namespace WHERE nspname LIKE 'gcs%';")
    check "party ${i}: Green schema dropped (gcs schemas = ${g})" "${g}" = "0"
    r=$(fleet_ready green "${i}")
    check "party ${i}: Green deployments ready ${r}" "${r%/*}" = "${r#*/}" -a "${r#*/}" != "0"
    bad=$(kubectl get pods -n "${NAMESPACE}" -o json | jq -r --arg re "^coprocessor-(${i}|polygon-${i})${GREEN_SLOT}-" --arg v "${GCS_STACK_VERSION}" '[.items[] | select(.metadata.name | test($re)) | select(.status.phase == "Running") | select(.metadata.labels["app.kubernetes.io/version"] != $v)] | length')
    check "party ${i}: every running Green replica carries version label ${GCS_STACK_VERSION} (${bad} without)" "${bad}" = "0"
    paused=$(kubectl logs -n "${NAMESPACE}" "deploy/coprocessor-${i}${LIVE_RELEASE_SUFFIX}-tx-sender" --tail=500 2>/dev/null | grep -c "pausing into no-op mode" || true)
    check "party ${i}: retired tx-sender paused into no-op mode (${paused} log line)" "${paused}" -ge 1
    syn=$(psql_party "${i}" "WITH h AS (SELECT substring(u.synthetic_txn_hashes FROM g.pos FOR 32) AS tx FROM upgrade_state u, generate_series(1, GREATEST(octet_length(u.synthetic_txn_hashes), 1), 32) AS g(pos) WHERE u.stack_role='GCS' AND octet_length(u.synthetic_txn_hashes) > 0) SELECT (SELECT count(*) FROM computations c JOIN h ON c.transaction_id = h.tx) + (SELECT count(*) FROM verify_proofs WHERE lower(contract_address) = lower('${SYNTHETIC_INPUT_CONTRACT}'));")
    check "party ${i}: synthetic rows left in public = ${syn}" "${syn}" = "0"
  done
  ;;

post)
  for i in ${parties}; do
    v=$(psql_party "${i}" "SELECT stack_version||'/'||COALESCE(to_jsonb(v)->>'consensus_version','1') FROM versioning v;")
    check "party ${i}: versioning still ${v}" "$(version_mm "${v%/*}")" = "$(version_mm "${GCS_STACK_VERSION}")"
    c=$(psql_party "${i}" "SELECT count(DISTINCT chain_id) FROM host_chain_blocks_valid WHERE created_at > now() - interval '120 seconds';")
    check "party ${i}: chains ingested by Green in the last 2 min = ${c}/${nb_chains}" "${c}" -ge "${nb_chains}"
    p=$(psql_party "${i}" "SELECT count(*) FILTER (WHERE NOT is_completed AND NOT is_error)||' pending, '||count(*) FILTER (WHERE is_error)||' errors' FROM computations WHERE created_at > now() - interval '30 minutes';")
    check "party ${i}: computations in the last 30 min: ${p}" "${p}" = "0 pending, 0 errors"
  done
  check_state_hash_agreement public
  while IFS='|' read -r line bad; do
    [[ -n "${line}" ]] || continue
    check "traffic ${line}" "${bad}" = "0"
  done <<<"$(traffic_counters)"
  info "run 'bg-traffic.sh verify' for the balance decryptions across the flip"
  ;;
*)
  echo "::error::unknown phase '${phase}'" >&2; exit 1 ;;
esac

if [[ "${failed}" == "0" ]]; then echo "== ${phase}: all checks passed"; else echo "== ${phase}: FAILED checks above"; exit 1; fi
