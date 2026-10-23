#!/usr/bin/env bash
# PRF spike (option A): the RFC-038 QA gate of the coprocessor half (Run A). A stand-in connector
# (prf-standins standin-connector) signs as the KMS tx-senders and retrieves labels through every
# party's coprocessor-proxy; the stand-in pad-worker injects faults by label first byte (the
# table is pad_worker_faults in scripts/deploy/deploy-coprocessor.sh). Cluster faults use kubectl.
# Prints one line per row: expected, observed, verdict (PASS, FAIL, MEASURED or NOT TESTABLE).
# Fails if a gate row fails.
# Keys: 0 general rows, 1 the pending-bits cap, 2 the in-flight cap, 3 the two-lengths row, so
# that no row meets the pending work of another.
# Env: NAMESPACE, NB_COPROCESSOR, NB_KMS_CORE, HOST_CHAIN_ID, PRF_IMAGE.
set -euo pipefail

: "${NAMESPACE:?}" "${NB_COPROCESSOR:?}" "${NB_KMS_CORE:?}" "${HOST_CHAIN_ID:?}" "${PRF_IMAGE:?}"
pod=prf-endpoint-check
results=$(mktemp)
failed=0
# shellcheck disable=SC2329 # called by the EXIT trap
finish() {
  kubectl delete pod "${pod}" -n "${NAMESPACE}" --wait=false >/dev/null 2>&1 || true
  if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
    { echo "### PRF endpoint QA gate (Run A)"; echo; echo "| row | expected | observed | verdict | evidence |"; echo "|---|---|---|---|---|"; cat "${results}"; } >> "${GITHUB_STEP_SUMMARY}"
  fi
}
trap finish EXIT

# The load pod of check-coprocessor-proxy.sh signs as tx-sender 1 and outlives a failed run: wait
# for it to go, then remove it, so that it cannot spend the keys of this check.
leftover=""
for _ in $(seq 1 60); do
  kubectl get pod coprocessor-proxy-check -n "${NAMESPACE}" >/dev/null 2>&1 || break
  sleep 2
done
if kubectl get pod coprocessor-proxy-check -n "${NAMESPACE}" >/dev/null 2>&1; then
  kubectl delete pod coprocessor-proxy-check -n "${NAMESPACE}" --wait=true >/dev/null
  leftover="deleted a leftover coprocessor-proxy-check pod"
fi

key_env='[]'
key_list=""
for i in $(seq 1 "${NB_KMS_CORE}"); do
  key_env=$(jq -c --arg i "${i}" '. + [{name: "KMS_KEY_\($i)", valueFrom: {secretKeyRef: {name: "kms-connector-tx-sender-\($i)", key: "kms-wallet"}}}]' <<<"${key_env}")
  key_list="${key_list:+${key_list},}\$(KMS_KEY_${i})"
done

kubectl delete pod "${pod}" -n "${NAMESPACE}" --ignore-not-found --wait=true
POD="${pod}" IMAGE="${PRF_IMAGE}" KEY_ENV="${key_env}" KEY_LIST="${key_list}" CHAIN="${HOST_CHAIN_ID}" yq -n '
{
  "apiVersion": "v1",
  "kind": "Pod",
  "metadata": {"name": strenv(POD)},
  "spec": {
    "restartPolicy": "Never",
    "imagePullSecrets": [{"name": "registry-credentials"}],
    "containers": [{
      "name": "connector",
      "image": strenv(IMAGE),
      "command": ["standin-connector", "idle"],
      "env": (env(KEY_ENV) + [
        {"name": "KMS_TX_SENDER_KEYS", "value": strenv(KEY_LIST)},
        {"name": "PRF_ETHEREUM_CHAIN_ID", "value": strenv(CHAIN)},
        {"name": "PRF_PROTOCOL_CONFIG_ADDRESS", "valueFrom": {"configMapKeyRef": {"name": "host-sc-addresses", "key": "protocol_config.address"}}},
        {"name": "PRF_URLS", "valueFrom": {"configMapKeyRef": {"name": "coprocessor-proxy", "key": "urls"}}},
        {"name": "PRF_SIGNERS", "valueFrom": {"configMapKeyRef": {"name": "coprocessor-proxy", "key": "signer-addresses"}}}
      ])
    }]
  }
}' | kubectl apply -n "${NAMESPACE}" -f -
kubectl wait --for=condition=Ready "pod/${pod}" -n "${NAMESPACE}" --timeout=300s

# The stand-in connector, in the check pod. rustls refuses the preview's self-signed CA:TRUE proxy
# certificate, so TLS is not verified: integrity comes from the attestation quorum.
connector() {
  kubectl exec "${pod}" -n "${NAMESPACE}" -- standin-connector --accept-invalid-certs "$@" \
    || echo '{"ok": false, "error": "connector_exited", "status": 0}'
}
# Parties are 1-based; the connector's coprocessor indexes are 0-based.
psql_party() { kubectl exec "statefulset/postgres-coprocessor-${1}" -n "${NAMESPACE}" -- psql -U zama -d fhevm_e2e -tAc "$2" || echo psql_failed; }
pad_worker_logs() { kubectl logs "deploy/coprocessor-${1}-standin-pad-worker" -n "${NAMESPACE}" --all-containers 2>/dev/null || true; }
label_hex() { jq -r '.label' <<<"$1" | sed 's/^0x//'; }
rand_label() { od -An -N31 -tx1 /dev/urandom | tr -d ' \n'; }

# Capacity events so far in the namespace, named next to any failed row: a row that ends in
# capacity is not a finding.
capacity() {
  local events oom
  events=$(kubectl get events -n "${NAMESPACE}" -o json 2>/dev/null \
    | jq -r '[.items[] | select(.reason | test("FailedScheduling|OOM|Evict|MemoryPressure")) | .reason] | group_by(.) | map("\(.[0]) x\(length)") | join(", ")' 2>/dev/null || true)
  oom=$(kubectl get pods -n "${NAMESPACE}" -o json 2>/dev/null \
    | jq -r '[.items[] | select(any(.status.containerStatuses[]?; .lastState.terminated.reason == "OOMKilled")) | .metadata.name] | join(", ")' 2>/dev/null || true)
  [[ -n "${events}${oom}" ]] && echo "capacity events so far: ${events:-none}; OOMKilled: ${oom:-none}"
  return 0
}

# record <row> <expected> <observed> <true | measure | untestable | anything else = fail> [evidence]
record() {
  local verdict evidence="${5:-}"
  case "$4" in
    true) verdict=PASS ;;
    measure) verdict=MEASURED ;;
    untestable) verdict="NOT TESTABLE" ;;
    *) verdict=FAIL; failed=1; evidence="${evidence:+${evidence}; }$(capacity)" ;;
  esac
  printf '| %s | %s | %s | %s | %s |\n' "$1" "$2" "$3" "${verdict}" "${evidence}" | tee -a "${results}"
}
summary() { jq -c '{ok, error, get_from, body_from, fallback, bytes, gets, resigns, elapsed_ms, replies: [(.replies // [])[] | "\(.coprocessor):\(.method):\(.status // "conn"):\(.code // .attestation)"]}' <<<"$1"; }
# A component that does not come back is a failed row, not the end of the matrix.
wait_ready() {
  kubectl rollout status "$1" -n "${NAMESPACE}" --timeout=300s >/dev/null \
    || record "$1 ready again" "ready within 300 s" "not ready" false
}
check_pod_load() {
  local top
  top=$(kubectl top pod "${pod}" -n "${NAMESPACE}" --no-headers 2>/dev/null || true)
  if [[ -n "${top}" ]]; then
    record "load generator after $1" "check pod CPU and memory" "${top}" measure
  else
    record "load generator after $1" "check pod CPU and memory" "kubectl top unavailable" untestable
  fi
}

echo "| row | expected | observed | verdict | evidence |"
echo "|---|---|---|---|---|"

# P0: every party's PRF path is up. A party that is not (capacity, OOM) makes later rows fail for
# a reason this row names.
not_ready=""
for p in $(seq 1 "${NB_COPROCESSOR}"); do
  for selector in "app.kubernetes.io/name=coprocessor-${p}-prf-service" "app.kubernetes.io/name=coprocessor-${p}-coprocessor-proxy" \
    "app=standin-pad-worker,fhevm.zama.ai/party=${p}" "statefulset.kubernetes.io/pod-name=postgres-coprocessor-${p}-0"; do
    ready=$(kubectl get pods -n "${NAMESPACE}" -l "${selector}" -o jsonpath='{.items[*].status.containerStatuses[0].ready}' 2>/dev/null || true)
    [[ "${ready}" == *true* ]] || not_ready+="${selector} "
  done
done
record "P0 PRF path of every party ready" "all ready" "${not_ready:-all ready}" "$([[ -z "${not_ready}" ]] && echo true)" "${leftover}"

# F1: four connectors ask for the same label at once: one evaluation per coprocessor, and the row
# is triggered by one of the tx-senders that signed (prf-service recovered the real signer).
reports=$(connector fetch --label-prefix 9a)
label=$(label_hex "$(head -1 <<<"${reports}")")
oks=$(jq -s '[.[] | select(.ok)] | length' <<<"${reports}")
senders=$(jq -rs '[.[].connector | ascii_downcase | sub("^0x"; "")] | join(",")' <<<"${reports}")
evaluations="" triggered_ok=true
for p in $(seq 1 "${NB_COPROCESSOR}"); do
  evaluations+="$(pad_worker_logs "${p}" | grep 'evaluated pad' | grep -c "label=0x${label}" || true) "
  triggered=$(psql_party "${p}" "SELECT encode(triggered_by, 'hex') FROM pads WHERE label = decode('${label}', 'hex')")
  [[ ",${senders}," == *",${triggered},"* ]] || triggered_ok=false
done
record "F1 4 connectors, same label" "4 ok; 1 evaluation per coprocessor; triggered_by is a signer" \
  "${oks} ok; evaluations ${evaluations% }; triggered_by ok=${triggered_ok}" \
  "$([[ "${oks}" == "${NB_KMS_CORE}" && "${evaluations// /}" == "$(printf '1%.0s' $(seq 1 "${NB_COPROCESSOR}"))" && "${triggered_ok}" == true ]] && echo true)" "label 0x${label}"

# F2: how often a healthy stack needs the fallback GET, and what it costs, under the two connector
# policies. Policy 1 is RFC-038 step 5 as written; policy 2 waits briefly for the GET target's body
# when its attestation is in the quorum group. One client each, so connections are pooled.
f2_stats='{retrievals: length, ok: ([.[] | select(.ok)] | length), fallback_rate: (([.[] | select(.fallback)] | length) / length), bytes_per_label: (([.[].bytes // 0] | add) / length | floor), gets_per_label: (([.[].gets // 0] | add) / length), p50_ms: ([.[].elapsed_ms // 0] | sort | .[length / 2 | floor]), max_ms: ([.[].elapsed_ms // 0] | max)}'
for policy in 1 2; do
  policy_args=()
  if (( policy == 2 )); then policy_args=(--body-wait 2s); fi
  for bits in 64 2048; do
    connector ${policy_args[@]+"${policy_args[@]}"} fetch --connectors 1 --repeat 100 --label-prefix 00 --pad-length "${bits}" > "/tmp/prf-f2-${policy}-${bits}.jsonl"
    out=$(jq -sc "${f2_stats}" "/tmp/prf-f2-${policy}-${bits}.jsonl")
    record "F2 policy ${policy}, ${bits}-bit pads (100 retrievals, pooled)" "measured; every retrieval ok" "${out}" \
      "$(jq -r 'if .ok == .retrievals then "measure" else "fail" end' <<<"${out}")"
  done
done
check_pod_load F2
# The run 4 form: a new process (cold TLS to every proxy) for each retrieval, policy 1.
for _ in $(seq 1 20); do connector fetch --connectors 1 --label-prefix 00; done > /tmp/prf-healthy.jsonl
out=$(jq -sc "${f2_stats}" /tmp/prf-healthy.jsonl)
record "F2 policy 1, 64-bit pads (20 retrievals, one process each)" "measured; every retrieval ok" "${out}" \
  "$(jq -r 'if .ok == .retrievals then "measure" else "fail" end' <<<"${out}")"

# fetch_case <row> <expected> <jq predicate on the report> <connector args...>
fetch_case() {
  local name="$1" expected="$2" predicate="$3"; shift 3
  local report
  report=$(connector "$@")
  record "${name}" "${expected}" "$(summary "${report}")" "$(jq "${predicate}" <<<"${report}")" "label $(jq -r .label <<<"${report}")"
}

fetch_case "slow pad-worker on the GET target (party 1, 10 s)" "ok via fallback GET" \
  '.ok and .fallback and .body_from != 0' fetch --connectors 1 --label-prefix d1 --get-from 0
fetch_case "1 wrong digest (party 1, GET target)" "outvoted, ok via fallback" \
  '.ok and .fallback and .body_from != 0' fetch --connectors 1 --label-prefix b1 --get-from 0
fetch_case "2 wrong digests (parties 1-2)" "outvoted, ok" '.ok' fetch --connectors 1 --label-prefix b2 --get-from 4
fetch_case "3 wrong digests (parties 1-3)" "no quorum, failed" '(.ok | not) and .error == "no_quorum"' fetch --connectors 1 --label-prefix b3
# attestation_case <row> <label prefix> <get-from>: quorum can form before party 1 answers, and
# then the report has no reply from party 1: retry on a new label until it does.
attestation_case() {
  local report
  for _ in 1 2 3 4 5; do
    report=$(connector fetch --connectors 1 --label-prefix "$2" --get-from "$3")
    jq -e '[(.replies // [])[] | select(.coprocessor == 0)] | length > 0' <<<"${report}" >/dev/null && break
  done
  record "$1" "rejected, ok" "$(summary "${report}")" \
    "$(jq '.ok and ([(.replies // [])[] | select(.coprocessor == 0) | .attestation] == ["rejected"])' <<<"${report}")" \
    "label $(jq -r .label <<<"${report}")"
}
attestation_case "1 unsigned attestation (party 1)" a1 1
attestation_case "S2 attestation signed by a key that is not party 1's (party 1, GET target)" a2 0
fetch_case "3 unsigned attestations (parties 1-3)" "rejected, no quorum" '(.ok | not)' fetch --connectors 1 --label-prefix a5
fetch_case "expired FhevmSig headers" "401 auth_expired, re-sign, ok" '.ok and .resigns >= 3' fetch --connectors 1 --label-prefix 00 --stale-headers
fetch_case "permanent failure on the GET target (party 1)" "ok via fallback" '.ok and .fallback' fetch --connectors 1 --label-prefix e2 --get-from 0
# The round can close on three failures before the GET reply arrives: check the replies present.
fetch_case "permanent failure everywhere" "500, not retried; a GET reply carries prf_evaluation_failed; fails before the deadline" \
  '(.ok | not) and .error == "no_quorum" and .elapsed_ms < 30000 and all((.replies // [])[]; .status == 500 and .attempts == 1) and all((.replies // [])[] | select(.method == "GET"); .code == "prf_evaluation_failed")' \
  fetch --connectors 1 --label-prefix e1 --get-from 0
fetch_case "R7 the first fallback GET fails (party 1 slow)" "the next quorum member serves" \
  '.ok and .fallback and any((.replies // [])[]; .code == "simulated_failure")' \
  --fail-first-fallback fetch --connectors 1 --label-prefix d1 --get-from 0

# probe_case <row> <expected status> <expected code> <target> <path> [connector args...]
probe_case() {
  local name="$1" status="$2" code="$3" target="$4" path="$5" out; shift 5
  out=$(connector probe --target "${target}" --path "${path}" "$@")
  record "${name}" "${status} ${code}" "$(jq -c . <<<"${out}")" \
    "$(jq --argjson s "${status}" --arg c "${code}" '.status == $s and ((.error.code // "") == $c)' <<<"${out}")"
}
l="00$(rand_label)"
probe_case "permanent failure, GET answer" 500 prf_evaluation_failed 1 "/v1/prf/0/labels/e1$(rand_label)/64"
probe_case "permanent failure, HEAD answer" 500 "" 1 "/v1/prf/0/labels/e1$(rand_label)/64" --method HEAD
# ask_case <row> <expected> <jq predicate on the reply> <connector args...>
ask_case() {
  local name="$1" expected="$2" predicate="$3"; shift 3
  local reply
  reply=$(connector "$@")
  record "${name}" "${expected}" "$(jq -c . <<<"${reply}")" "$(jq "${predicate}" <<<"${reply}")"
}
for method in GET HEAD; do
  ask_case "retry rule: 500 on ${method} stops" "1 attempt, nothing retried" \
    '.status == 500 and .attempts == 1 and .retried == []' ask --target 1 --label-prefix e1 --method "${method}"
done
probe_case "non-canonical path: uppercase label" 400 non_canonical_path 1 "/v1/prf/0/labels/AB$(rand_label | tr a-f A-F)/64"
probe_case "non-canonical path: leading zero" 400 non_canonical_path 1 "/v1/prf/00/labels/${l}/64"
probe_case "query string" 401 auth_query_string 1 "/v1/prf/0/labels/${l}/64?x=1"
probe_case "unknown prf" 404 unknown_prf 1 "/v1/prf/1/labels/${l}/64"
probe_case "padLength 0" 400 non_canonical_path 1 "/v1/prf/0/labels/${l}/0"
probe_case "padLength 2049" 400 non_canonical_path 1 "/v1/prf/0/labels/${l}/2049"
probe_case "padLength 2048" 200 "" 1 "/v1/prf/0/labels/${l}/2048"

# C2: an unknown signer. GET tells it apart, HEAD cannot; the connector's GET keeps retrying it
# (RFC-038 step 6) until the deadline, its HEADs stop at once.
probe_case "C2 unknown signer, GET (party 2)" 401 auth_unknown_signer 1 "/v1/prf/0/labels/00$(rand_label)/64" --random-key
probe_case "C2 unknown signer, HEAD (party 2)" 401 "" 1 "/v1/prf/0/labels/00$(rand_label)/64" --random-key --method HEAD
fetch_case "C2 unknown signer, retrieval (deadline 10 s)" "not ok; every HEAD reply 1 attempt" \
  '(.ok | not) and all((.replies // [])[] | select(.method == "HEAD"); .attempts == 1)' \
  --random-keys --deadline 10s fetch --connectors 1 --label-prefix 00
ask_case "C2 unknown signer, GET request loop (deadline 10 s)" "retried until the deadline" \
  '.error == "deadline" and .elapsed_ms >= 9000' --random-keys --deadline 10s ask --target 1 --label-prefix 00

# G5: a header signed for party 1 sent to party 2, and a random key: 401 after the registry wait.
signers=$(kubectl get configmap coprocessor-proxy -n "${NAMESPACE}" -o jsonpath='{.data.signer-addresses}')
probe_case "G5 header signed for party 1, sent to party 2" 401 auth_unknown_signer 1 "/v1/prf/0/labels/00$(rand_label)/64" --audience "${signers%%,*}"
waits=""
for _ in 1 2 3; do
  waits+="$(connector probe --target 1 --path "/v1/prf/0/labels/00$(rand_label)/64" --random-key | jq -r '"\(.status):\(.elapsed_ms)ms"') "
done
warns=$(kubectl logs deploy/coprocessor-2-coprocessor-proxy -n "${NAMESPACE}" --since=2m 2>/dev/null | grep -c 'Unknown signer after waiting' || true)
record "G5 unknown-signer wait at party 2" "about 1 s each, at most about 3.5 s; one WARN line per request" \
  "3 random-key probes: ${waits% }; ${warns} WARN lines in party 2's proxy log over 2 min (7 unknown-signer requests in C2 and G5, plus the C2 retrieval's retries)" measure

# B1: the cap of party 1: at most two pending 2048-bit pads per connector (4096 bits). Key 1.
out=$(connector --key-index 1 flood --target 0 --count 6 --label-prefix 4a --pad-length 2048)
record "B1 per-connector cap (party 1, key 1)" '2x 200, 4x 429 concurrency_limited, Retry-After' "$(jq -c . <<<"${out}")" \
  "$(jq '.statuses == {"200": 2, "429 concurrency_limited": 4} and .retry_after != null' <<<"${out}")"
# G2: party 2 holds at most two requests of one connector. Key 2.
out=$(connector --key-index 2 flood --target 1 --count 3 --label-prefix 4a --pad-length 64)
record "G2 PRF_MAX_IN_FLIGHT_PER_CONNECTOR (party 2, 2 requests, key 2)" '2x 200, 1x 429 concurrency_limited' "$(jq -c . <<<"${out}")" \
  "$(jq '.statuses == {"200": 2, "429 concurrency_limited": 1}' <<<"${out}")"

# S1: one label at 64 and 2048 bits at once (party 1, key 3, 5 s evaluations). Both rows count
# against the 4096-bit cap: 64 + 2048 + another 2048 is over it.
x="4a$(rand_label)"
connector --key-index 3 ask --target 0 --label "${x}" --pad-length 64 > /tmp/prf-s1-64.json &
s1_short=$!
connector --key-index 3 ask --target 0 --label "${x}" --pad-length 2048 > /tmp/prf-s1-2048.json &
s1_long=$!
sleep 3
third=$(connector --key-index 3 probe --target 0 --path "/v1/prf/0/labels/4a$(rand_label)/2048")
wait "${s1_short}" "${s1_long}" || true
rows=$(psql_party 1 "SELECT count(*) FROM pads WHERE label = decode('${x}', 'hex')")
short=$(cat /tmp/prf-s1-64.json) long=$(cat /tmp/prf-s1-2048.json)
record "S1 same label at 64 and 2048 bits (party 1, key 3)" "both 200, different digests; 2 rows; a third 2048-bit label gets 429" \
  "64: $(jq -c '{status, digest}' <<<"${short}"); 2048: $(jq -c '{status, digest}' <<<"${long}"); rows ${rows}; third $(jq -c '{status, code: .error.code}' <<<"${third}")" \
  "$(jq -n --argjson a "${short}" --argjson b "${long}" --argjson t "${third}" --arg rows "${rows}" \
    '$a.status == 200 and $b.status == 200 and $a.digest != $b.digest and $rows == "2" and $t.status == 429 and $t.error.code == "concurrency_limited"')" "label 0x${x}"

# D1: the connector drops while its request is held (party 3, 20 s evaluation), then asks again.
d="6d$(rand_label)"
aborted=$(connector ask --target 2 --label "${d}" --abort-after 2s)
pending=$(psql_party 3 "SELECT count(*) FROM pads WHERE label = decode('${d}', 'hex') AND signature IS NULL")
retry=$(connector ask --target 2 --label "${d}" --deadline 60s)
claims=$(pad_worker_logs 3 | grep 'claimed pad' | grep -c "label=0x${d}" || true)
evals=$(pad_worker_logs 3 | grep 'evaluated pad' | grep -c "label=0x${d}" || true)
record "D1 drop while held, retry later (party 3)" "row still pending after the drop; retry 200; 1 claim, 1 evaluation; at most one 429" \
  "aborted $(jq -c . <<<"${aborted}"); pending ${pending}; retry $(jq -c '{status, attempts, retried, elapsed_ms}' <<<"${retry}"); claims ${claims}, evaluations ${evals}" \
  "$(jq --arg p "${pending}" --arg c "${claims}" --arg e "${evals}" --argjson a "${aborted}" \
    '$a.aborted_after_ms != null and $p == "1" and .status == 200 and $c == "1" and $e == "1" and ([(.retried // [])[] | select(startswith("429"))] | length) <= 1' <<<"${retry}")" "label 0x${d}"
# D2: the same, with the new request at once: the dropped request must free its place.
d="6d$(rand_label)"
aborted=$(connector ask --target 2 --label "${d}" --abort-after 2s)
retry=$(connector ask --target 2 --label "${d}" --deadline 60s)
record "D2 drop while held, immediate retry (party 3)" "200; at most one 429 concurrency_limited" \
  "aborted $(jq -c . <<<"${aborted}"); retry $(jq -c '{status, attempts, retried, elapsed_ms}' <<<"${retry}")" \
  "$(jq --argjson a "${aborted}" '$a.aborted_after_ms != null and .status == 200 and ([(.retried // [])[] | select(startswith("429"))] | length) <= 1' <<<"${retry}")" "label 0x${d}"

# R3: the pad-worker of parties 1-3 exits once with the row claimed; the reaper gives it back.
restarts_before=$(for p in 1 2 3; do kubectl get pods -n "${NAMESPACE}" -l "app=standin-pad-worker,fhevm.zama.ai/party=${p}" -o jsonpath='{.items[0].status.containerStatuses[0].restartCount}'; echo -n " "; done)
report=$(connector fetch --connectors 1 --label-prefix c1 --deadline 120s)
restarts_after=$(for p in 1 2 3; do kubectl get pods -n "${NAMESPACE}" -l "app=standin-pad-worker,fhevm.zama.ai/party=${p}" -o jsonpath='{.items[0].status.containerStatuses[0].restartCount}'; echo -n " "; done)
record "R3 pad-worker killed mid-evaluation (parties 1-3)" "ok after restart and reap" "$(summary "${report}"); restarts ${restarts_before}-> ${restarts_after}" \
  "$(jq '.ok' <<<"${report}")"

# An evaluation longer than the connector's 30 s deadline, then the stored output.
report=$(connector fetch --connectors 1 --label-prefix 7f)
label=$(jq -r .label <<<"${report}")
sleep 15
again=$(connector fetch --connectors 1 --label "${label}")
record "evaluation of 40 s, connector deadline 30 s" "first fails at the deadline; the stored output is served after" \
  "first $(summary "${report}"); then $(summary "${again}")" \
  "$(jq -n --argjson a "${report}" --argjson b "${again}" '($a.error == "deadline") and $b.ok and $b.elapsed_ms < 5000')"

# R2: prf-service of parties 1-3 is killed while it holds requests for a 20 s evaluation.
connector fetch --connectors 1 --label-prefix 6d --deadline 120s > /tmp/prf-held.json &
fetch_pid=$!
sleep 5
for p in 1 2 3; do
  kubectl delete pod -n "${NAMESPACE}" -l "app.kubernetes.io/name=coprocessor-${p}-prf-service" --wait=false
done
wait "${fetch_pid}" || true
report=$(cat /tmp/prf-held.json)
record "R2 prf-service killed while holding requests (parties 1-3)" "ok: retries reach the new pods" "$(summary "${report}")" "$(jq '.ok' <<<"${report}")"
for p in 1 2 3; do wait_ready "deploy/coprocessor-${p}-prf-service"; done

# The proxy answers 502 upstream_transient while party 1 has no prf-service pod. O6: the proxy
# stays Ready, so only its 502 counter shows that the party is down.
proxy_pod() { kubectl get pods -n "${NAMESPACE}" -l "app.kubernetes.io/name=coprocessor-${1}-coprocessor-proxy" -o jsonpath='{.items[0].metadata.name}'; }
prf_502() {
  local metrics
  if ! metrics=$(kubectl get --raw "/api/v1/namespaces/${NAMESPACE}/pods/$(proxy_pod 1):9100/proxy/metrics" 2>/dev/null); then
    echo unavailable
    return
  fi
  awk '/^coprocessor_proxy_requests_total/ && /route="prf"/ && /status="502"/ {s += $NF} END {printf "%d\n", s}' <<<"${metrics}"
}
before_502=$(prf_502)
kubectl scale deploy/coprocessor-1-prf-service -n "${NAMESPACE}" --replicas=0 >/dev/null
kubectl wait --for=delete pod -l app.kubernetes.io/name=coprocessor-1-prf-service -n "${NAMESPACE}" --timeout=120s >/dev/null || true
connector --deadline 120s ask --target 0 --label-prefix 00 > /tmp/prf-502.json &
ask_pid=$!
sleep 8
proxy_ready=$(kubectl get pods -n "${NAMESPACE}" -l app.kubernetes.io/name=coprocessor-1-coprocessor-proxy -o jsonpath='{.items[*].status.containerStatuses[0].ready}')
after_502=$(prf_502)
sleep 2
kubectl scale deploy/coprocessor-1-prf-service -n "${NAMESPACE}" --replicas=1 >/dev/null
wait "${ask_pid}" || true
wait_ready deploy/coprocessor-1-prf-service
reply=$(cat /tmp/prf-502.json)
record "retry rule: proxy 502 is retried (party 1 prf-service down 10 s)" "502 upstream_transient retried, then 200" \
  "$(jq -c . <<<"${reply}")" "$(jq '.status == 200 and any((.retried // [])[]; . == "502 upstream_transient")' <<<"${reply}")"
if [[ "${before_502}" == unavailable || "${after_502}" == unavailable || -z "${before_502}" || -z "${after_502}" ]]; then
  record "O6 proxy Ready while prf-service is down (party 1)" "the 502 counter rises" "metrics read refused; proxy ready=${proxy_ready}" untestable
else
  record "O6 proxy Ready while prf-service is down (party 1)" "the 502 counter rises" "502 count ${before_502} -> ${after_502}; proxy ready=${proxy_ready}" \
    "$([[ "${after_502%.*}" -gt "${before_502%.*}" ]] && echo true)"
fi

# R1: proxy rolling restart under load: sequential retrievals until every proxy has restarted.
rm -f /tmp/prf-restart.stop
(
  until [[ -f /tmp/prf-restart.stop ]]; do connector fetch --connectors 1 --label-prefix 00; done
) > /tmp/prf-restart.jsonl &
load_pid=$!
sleep 3
for p in $(seq 1 "${NB_COPROCESSOR}"); do kubectl rollout restart "deploy/coprocessor-${p}-coprocessor-proxy" -n "${NAMESPACE}"; done
for p in $(seq 1 "${NB_COPROCESSOR}"); do wait_ready "deploy/coprocessor-${p}-coprocessor-proxy"; done
touch /tmp/prf-restart.stop
wait "${load_pid}" || true
out=$(jq -sc '{retrievals: length, ok: [.[] | select(.ok)] | length, fallback: [.[] | select(.fallback)] | length, max_ms: ([.[].elapsed_ms // 0] | max)}' /tmp/prf-restart.jsonl)
record "R1 proxy rolling restart under load" "every retrieval ok" "${out}" "$(jq '.ok == .retrievals' <<<"${out}")"
check_pod_load R1

# R9: one client asks party 2 every 15 s while its proxy restarts: a pooled connection that idled
# through the drain may fail once, and the connector retries it.
connector --deadline 60s ask --target 1 --label-prefix 00 --repeat 16 --interval 15s > /tmp/prf-r9.jsonl &
r9_pid=$!
sleep 5
kubectl rollout restart deploy/coprocessor-2-coprocessor-proxy -n "${NAMESPACE}"
wait_ready deploy/coprocessor-2-coprocessor-proxy
wait "${r9_pid}" || true
out=$(jq -sc '{asks: length, ok: ([.[] | select(.status == 200)] | length), connection_errors: ([.[].retried // [] | .[] | select(. == "connection_error")] | length), max_ms: ([.[].elapsed_ms // 0] | max)}' /tmp/prf-r9.jsonl)
record "R9 pooled connection across a proxy restart (party 2)" "every ask 200; connection errors retried" "${out}" \
  "$(jq '.asks == 16 and .ok == 16' <<<"${out}")"

# R5: staggered restarts under a continuous load, one component of each party 10 s apart. Each
# failed label is retried once.
rm -f /tmp/prf-r5.stop
(
  until [[ -f /tmp/prf-r5.stop ]]; do
    first=$(connector fetch --connectors 1 --label-prefix 00)
    jq -c '. + {try: 1}' <<<"${first}"
    retry_label=$(jq -r 'if .ok then empty else (.label // empty) end' <<<"${first}")
    if [[ -n "${retry_label}" ]]; then
      connector fetch --connectors 1 --label "${retry_label}" | jq -c '. + {try: 2}'
    fi
  done
) > /tmp/prf-r5.jsonl &
load_pid=$!
sleep 3
t0=${SECONDS}
kubectl rollout restart deploy/coprocessor-1-coprocessor-proxy -n "${NAMESPACE}"
sleep 10
kubectl delete pod -n "${NAMESPACE}" -l app.kubernetes.io/name=coprocessor-2-prf-service --wait=false
sleep 10
kubectl delete pod -n "${NAMESPACE}" -l "app=standin-pad-worker,fhevm.zama.ai/party=3" --wait=false
sleep 10
kubectl delete pod postgres-coprocessor-4-0 -n "${NAMESPACE}" --wait=false
sleep 10
kubectl rollout restart deploy/coprocessor-5-coprocessor-proxy -n "${NAMESPACE}"
ready_by=""
for target in deploy/coprocessor-1-coprocessor-proxy deploy/coprocessor-2-prf-service deploy/coprocessor-3-standin-pad-worker \
  statefulset/postgres-coprocessor-4 deploy/coprocessor-4-prf-service deploy/coprocessor-4-standin-pad-worker deploy/coprocessor-5-coprocessor-proxy; do
  wait_ready "${target}"
  ready_by+="${target#*/coprocessor-}:$((SECONDS - t0))s "
done
touch /tmp/prf-r5.stop
wait "${load_pid}" || true
restarts4=$(kubectl get pods -n "${NAMESPACE}" -l app.kubernetes.io/name=coprocessor-4-prf-service -o jsonpath='{.items[0].status.containerStatuses[0].restartCount}' 2>/dev/null || true)
out=$(jq -sc '{labels: (group_by(.label) | length), first_try_failures: ([.[] | select(.try == 1 and (.ok | not))] | length), failed_labels: (group_by(.label) | map(select(all(.[]; .ok | not))) | length), max_ms: ([.[].elapsed_ms // 0] | max)}' /tmp/prf-r5.jsonl)
record "R5 staggered restarts under load (parties 1-5, 10 s apart)" "every label ok on its first try or its one retry" \
  "${out}; ready by (from t0) ${ready_by% }; party 4 prf-service restarts ${restarts4}" \
  "$(jq '.failed_labels == 0' <<<"${out}")"

# R8 and coprocessors down: proxies scaled to 0, one after the other. Each step waits until a new
# connection to that party is refused (the old pod accepts for its 10 s shutdownDelaySeconds).
proxy_down() { kubectl scale "deploy/coprocessor-${1}-coprocessor-proxy" -n "${NAMESPACE}" --replicas=0 >/dev/null; }
# wait_refused <party>: prints the seconds until a probe to it gets a transport error.
wait_refused() {
  local started=${SECONDS}
  for _ in $(seq 1 30); do
    if connector probe --target "$(( $1 - 1 ))" --path "/v1/prf/0/labels/00$(rand_label)/64" | jq -e '.transport_error != null' >/dev/null; then
      echo "$((SECONDS - started))"
      return
    fi
    sleep 2
  done
  echo "not refused after $((SECONDS - started))"
}
replicas=$(kubectl get deploy/coprocessor-5-coprocessor-proxy -n "${NAMESPACE}" -o jsonpath='{.spec.replicas}')
connector --deadline 120s ask --target 4 --label-prefix 6d > /tmp/prf-r8.json &
r8_pid=$!
sleep 3
proxy_down 5
refused5=$(wait_refused 5)
wait "${r8_pid}" || true
reply=$(cat /tmp/prf-r8.json)
record "R8 scale-down drains a held request (party 5)" "the held request returns 200 on attempt 1" \
  "$(jq -c '{status, attempts, retried, elapsed_ms}' <<<"${reply}"); new connections refused after ${refused5} s" \
  "$(jq '.status == 200 and .attempts == 1' <<<"${reply}")" "$(jq -r .path <<<"${reply}")"
fetch_case "1 coprocessor down (party 5, GET target)" "ok via fallback" '.ok and .fallback' fetch --connectors 1 --label-prefix 00 --get-from 4
proxy_down 4
refused4=$(wait_refused 4)
fetch_case "2 coprocessors down (parties 4-5)" "ok via fallback" '.ok and .fallback' fetch --connectors 1 --label-prefix 00 --get-from 3
proxy_down 3
refused3=$(wait_refused 3)
fetch_case "3 coprocessors down (parties 3-5)" "no quorum before the deadline: retryable failure" '(.ok | not) and .error == "deadline"' fetch --connectors 1 --label-prefix 00
record "seconds until a scaled-down proxy refuses connections" "about 10-15 s (shutdownDelaySeconds 10 s)" "party 5: ${refused5}; party 4: ${refused4}; party 3: ${refused3}" measure
for p in 3 4 5; do
  kubectl scale "deploy/coprocessor-${p}-coprocessor-proxy" -n "${NAMESPACE}" --replicas="${replicas}" >/dev/null
  wait_ready "deploy/coprocessor-${p}-coprocessor-proxy"
done

# O1: the proxy holds a request just under its 120 s endpoint_response_timeout (110 s), and an
# evaluation of 130 s gets 504 timeout, whose retry joins the same evaluation. In parallel: party 2
# holds two requests of one connector.
connector --deadline 200s ask --target 1 --label-prefix f1 > /tmp/prf-o1-f1.json &
f1_pid=$!
connector --deadline 200s ask --target 1 --label-prefix f0 > /tmp/prf-o1-f0.json &
f0_pid=$!
wait "${f1_pid}" "${f0_pid}" || true
reply=$(cat /tmp/prf-o1-f1.json)
record "O1 evaluation of 110 s, proxy timeout 120 s" "200 on attempt 1, no 504" "$(jq -c . <<<"${reply}")" \
  "$(jq '.status == 200 and .attempts == 1 and all((.retried // [])[]; . != "504 timeout")' <<<"${reply}")"
reply=$(cat /tmp/prf-o1-f0.json)
record "O1 retry rule: proxy 504 is retried (evaluation 130 s, proxy timeout 120 s)" "504 timeout retried, then 200" \
  "$(jq -c . <<<"${reply}")" "$(jq '.status == 200 and (.retried // [])[0] == "504 timeout"' <<<"${reply}")"

# R4 and A1, last, since prf-service and the pad-worker of party 5 exit without their database and
# may then wait out a crash-loop backoff: the Postgres of party 5 is down.
kubectl scale statefulset/postgres-coprocessor-5 -n "${NAMESPACE}" --replicas=0 >/dev/null
kubectl wait --for=delete pod/postgres-coprocessor-5-0 -n "${NAMESPACE}" --timeout=120s >/dev/null || true
fetch_case "R4 Postgres of one coprocessor down (party 5, GET target)" "ok via fallback" '.ok and .fallback' fetch --connectors 1 --label-prefix 00 --get-from 4
class=$(connector --deadline 15s ask --target 4 --label-prefix 00)
record "A1 what party 5 answers without its Postgres" "503 unavailable (not retried) or 502 / connection error (retried)" \
  "$(jq -c . <<<"${class}")" measure
kubectl scale statefulset/postgres-coprocessor-5 -n "${NAMESPACE}" --replicas=1 >/dev/null
started=${SECONDS}
recovered=true
{ kubectl rollout status statefulset/postgres-coprocessor-5 -n "${NAMESPACE}" --timeout=300s >/dev/null \
  && wait_ready deploy/coprocessor-5-prf-service && wait_ready deploy/coprocessor-5-standin-pad-worker; } || recovered=false
restarts=$(kubectl get pods -n "${NAMESPACE}" -l app.kubernetes.io/name=coprocessor-5-prf-service -o jsonpath='{.items[0].status.containerStatuses[0].restartCount}' 2>/dev/null || true)
reply=$(connector ask --target 4 --label-prefix 00)
record "R4 party 5 recovers after its Postgres returns" "prf-service ready; party 5 answers 200" \
  "ready=${recovered} after $((SECONDS - started)) s, prf-service restarts ${restarts}; $(jq -c '{status, attempts, retried, elapsed_ms}' <<<"${reply}")" \
  "$(jq --argjson r "${recovered}" '$r and .status == 200' <<<"${reply}")"

exit "${failed}"
