#!/usr/bin/env bash
# PRF spike (option A): the QA gate of the coprocessor half, Run B (load). A stand-in connector
# (prf-standins standin-connector) signs as the KMS tx-senders and loads every party's
# coprocessor-proxy; the stand-in pad-worker costs E = 1 s + 1.5 ms x padLength for a label
# without a fault (1.1 s at 64 bits, 4.1 s at 2048 bits) and evaluates C = 16 rows at once. Labels
# starting with 0f cost nothing. Run B deploys without the test caps.
# Load rows gate on correctness only: no 5xx but the retryable 502 and 504, no lost label, no
# unplanned pod restart, never more than C evaluations at once, and every label that misses the
# deadline served on retry. Everything else is measured. A row that meets a capacity event on the
# PRF path is CAPACITY, never a pass; a saturated load generator makes a row NOT TESTABLE.
# Keys: 0-2 honest, 3 the rogue.
# Env: NAMESPACE, NB_COPROCESSOR, NB_KMS_CORE, HOST_CHAIN_ID, PRF_IMAGE.
set -euo pipefail

: "${NAMESPACE:?}" "${NB_COPROCESSOR:?}" "${NB_KMS_CORE:?}" "${HOST_CHAIN_ID:?}" "${PRF_IMAGE:?}"
pod=prf-endpoint-check
results=$(mktemp)
samples=$(mktemp)
failed=0
C=16
# shellcheck disable=SC2329 # called by the EXIT trap
finish() {
  kubectl delete pod "${pod}" -n "${NAMESPACE}" --wait=false >/dev/null 2>&1 || true
  if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
    { echo "### PRF endpoint QA gate (Run B)"; echo; echo "| row | expected | observed | verdict | evidence |"; echo "|---|---|---|---|---|"; cat "${results}"; } >> "${GITHUB_STEP_SUMMARY}"
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

# The check pod requests resources: a BestEffort pod is the first one throttled or evicted.
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
      "resources": {"requests": {"cpu": "1", "memory": "512Mi"}},
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
rand_label() { od -An -N31 -tx1 /dev/urandom | tr -d ' \n'; }
now() { date -u +%Y-%m-%dT%H:%M:%SZ; }
prf_pods='^(coprocessor-[0-9]+-(coprocessor-proxy|prf-service|standin-pad-worker)-|postgres-coprocessor-[0-9]+-0$|prf-endpoint-check$)'

# record <row> <expected> <observed> <true | measure | untestable | capacity | anything else = fail> [evidence]
record() {
  local verdict
  case "$4" in
    true) verdict=PASS ;;
    measure) verdict=MEASURED ;;
    untestable) verdict="NOT TESTABLE" ;;
    capacity) verdict=CAPACITY; failed=1 ;;
    *) verdict=FAIL; failed=1 ;;
  esac
  printf '| %s | %s | %s | %s | %s |\n' "$1" "$2" "$3" "${verdict}" "${5:-}" | tee -a "${results}"
}

# PRF path pods by uid, with their restart counts.
pods_snapshot() {
  kubectl get pods -n "${NAMESPACE}" -o json | jq -c --arg re "${prf_pods}" \
    '[.items[] | select(.metadata.name | test($re)) | {key: .metadata.uid, value: {name: .metadata.name, restarts: ([.status.containerStatuses[]?.restartCount] | add // 0)}}] | from_entries'
}
# unplanned_restarts <before> <after> [pod name regex that may be replaced]
unplanned_restarts() {
  jq -rn --argjson b "$1" --argjson a "$2" --arg planned "${3:-^$}" \
    '[$b | to_entries[] | select(.value.name | test($planned) | not)
      | if $a[.key] == null then "replaced \(.value.name)" elif $a[.key].restarts > .value.restarts then "restarted \(.value.name)" else empty end]
     | join(", ")'
}
# Capacity events on the PRF path since <time>: scheduling, OOM and eviction events, OOMKilled
# containers, and MemoryPressure on the nodes that host the PRF path.
capacity_since() {
  local since="$1" events oom nodes pressure=""
  events=$(kubectl get events -n "${NAMESPACE}" -o json | jq -r --arg re "${prf_pods}" --arg t "${since}" \
    '[.items[] | select((.lastTimestamp // .eventTime // "") >= $t and (.reason | test("FailedScheduling|OOM|Evict")) and (.involvedObject.name | test($re))) | "\(.reason) \(.involvedObject.name)"] | unique | join(", ")')
  oom=$(kubectl get pods -n "${NAMESPACE}" -o json | jq -r --arg re "${prf_pods}" --arg t "${since}" \
    '[.items[] | select(.metadata.name | test($re)) | select(any(.status.containerStatuses[]?; .lastState.terminated.reason == "OOMKilled" and .lastState.terminated.finishedAt >= $t)) | "OOMKilled \(.metadata.name)"] | join(", ")')
  nodes=$(kubectl get pods -n "${NAMESPACE}" -o json | jq -r --arg re "${prf_pods}" '[.items[] | select(.metadata.name | test($re)) | .spec.nodeName // empty] | unique | .[]')
  for node in ${nodes}; do
    if kubectl get node "${node}" -o jsonpath='{.status.conditions[?(@.type=="MemoryPressure")].status}' 2>/dev/null | grep -q True; then
      pressure+="MemoryPressure ${node} "
    fi
  done
  echo "${events}${oom:+; ${oom}}${pressure:+; ${pressure}}" | sed 's/^; //'
}
# The most evaluations at once at each party since <time>, from the pad-worker claim and finish lines.
concurrency_since() {
  local out=""
  for p in $(seq 1 "${NB_COPROCESSOR}"); do
    out+="$(kubectl logs "deploy/coprocessor-${p}-standin-pad-worker" -n "${NAMESPACE}" --since-time="$1" 2>/dev/null \
      | awk '/claimed pad/ {n++; if (n > m) m = n} /evaluated pad|marked pad failed/ {if (n > 0) n--} END {print m + 0}') "
  done
  echo "${out% }"
}
# kubectl top every 10 s while a row runs; memory_max prints the highest memory of each component.
sample_start() {
  : > "${samples}"
  ( while true; do kubectl top pods -n "${NAMESPACE}" --no-headers 2>/dev/null >> "${samples}" || true; sleep 10; done ) &
  sampler=$!
}
sample_stop() {
  kill "${sampler}" 2>/dev/null || true
  wait "${sampler}" 2>/dev/null || true
  awk '$1 ~ /coprocessor-[0-9]+-(coprocessor-proxy|prf-service)-|postgres-coprocessor-[0-9]+-0|prf-endpoint-check/ {
         name = $1; if (name ~ /^coprocessor-/) sub(/-[a-z0-9]+-[a-z0-9]+$/, "", name); cpu = $2 + 0; mem = $3 + 0
         if (mem > m[name]) m[name] = mem; if (cpu > c[name]) c[name] = cpu }
       END { for (n in m) printf "%s %dm/%dMi; ", n, c[n], m[n] }' "${samples}" | sed 's/; $//'
}
# Waits until no row is pending at any party; prints the seconds it took.
wait_drained() {
  local started=${SECONDS} pending
  while (( SECONDS - started < 600 )); do
    pending=0
    for p in $(seq 1 "${NB_COPROCESSOR}"); do
      n=$(psql_party "${p}" "SELECT count(*) FROM pads WHERE signature IS NULL AND NOT is_error")
      [[ "${n}" =~ ^[0-9]+$ ]] || n=1
      pending=$((pending + n))
    done
    if (( pending == 0 )); then echo "$((SECONDS - started))"; return; fi
    sleep 3
  done
  echo "not drained after 600"
}

row_begin() {
  row_t=$(now)
  row_pods=$(pods_snapshot)
  sample_start
}
# row_end <row> <expected> <observed> <correct: true|false> [saturated: true|false] [planned replacements regex]
row_end() {
  local name="$1" expected="$2" observed="$3" correct="$4" saturated="${5:-false}" planned="${6:-^$}"
  local usage restarts capacity concurrency drained verdict over
  usage=$(sample_stop)
  drained=$(wait_drained)
  restarts=$(unplanned_restarts "${row_pods}" "$(pods_snapshot)" "${planned}")
  capacity=$(capacity_since "${row_t}")
  concurrency=$(concurrency_since "${row_t}")
  over=$(tr ' ' '\n' <<<"${concurrency}" | awk -v c="${C}" '$1 > c' | head -1)
  if [[ -n "${capacity}" ]]; then verdict=capacity
  elif [[ "${saturated}" == true ]]; then verdict=untestable
  elif [[ "${correct}" == true && -z "${restarts}" && -z "${over}" ]]; then verdict=true
  else verdict=false
  fi
  record "${name}" "${expected}" "${observed}; most evaluations at once per party ${concurrency}; queue drained after ${drained} s; top ${usage}" \
    "${verdict}" "${restarts:+unplanned: ${restarts}; }${capacity:+capacity: ${capacity}}"
  last_verdict="${verdict}"
}
# Correctness of a burst report: no 5xx but 502 and 504, and every failed retrieval served on retry.
# Only the retryable 5xx (502, 504) are allowed: a 503 is the A1 finding.
allowed_5xx='all((test("^5") | not) or test("^50[24]"))'
burst_correct() { jq "(.statuses | keys | ${allowed_5xx}) and .requests > 0 and (if .retry then .retry.ok == .retry.retried else .ok == .requests end)" <<<"$1"; }
burst_saturated() { jq '.lag_ms.p99 > 500' <<<"$1"; }
burst_summary() { jq -c '{users, requests, ok, failed: (.failed | length), failed_labels, latency_ms, lag_ms, statuses, fallback_rate, bytes_per_label, drain_ms, max_fds, retry: (.retry | if . then {retried, ok, failed: (.failed | length), latency_ms} else null end)}' <<<"$1"; }
# run_burst <row> <expected> <connector args...>: one burst row, with the retry of what missed the deadline.
run_burst() {
  local name="$1" expected="$2" out; shift 2
  row_begin
  out=$(connector "$@")
  row_end "${name}" "${expected}" "$(burst_summary "${out}")" "$(burst_correct "${out}")" "$(burst_saturated "${out}")"
  last_burst="${out}"
}

echo "| row | expected | observed | verdict | evidence |"
echo "|---|---|---|---|---|"

# P0: every party's PRF path is up, and nothing is pending.
not_ready=""
for p in $(seq 1 "${NB_COPROCESSOR}"); do
  for selector in "app.kubernetes.io/name=coprocessor-${p}-prf-service" "app.kubernetes.io/name=coprocessor-${p}-coprocessor-proxy" \
    "app=standin-pad-worker,fhevm.zama.ai/party=${p}" "statefulset.kubernetes.io/pod-name=postgres-coprocessor-${p}-0"; do
    ready=$(kubectl get pods -n "${NAMESPACE}" -l "${selector}" -o jsonpath='{.items[*].status.containerStatuses[0].ready}' 2>/dev/null || true)
    [[ "${ready}" == *true* ]] || not_ready+="${selector} "
  done
done
record "P0 PRF path of every party ready" "all ready" "${not_ready:-all ready}" "$([[ -z "${not_ready}" ]] && echo true)" "${leftover}"

# S1 rerun: one label at 64 and 2048 bits from one process (party 1, key 3, 20 s evaluations).
# While both rows are pending, the sum admission uses for key 3 is 64 + 2048.
x="6d$(rand_label)"
row_begin
connector --key-index 3 ask --target 0 --label "${x}" --pad-lengths 64,2048 > /tmp/prf-s1.jsonl &
s1_pid=$!
sleep 6
rows=$(psql_party 1 "SELECT count(*) FROM pads WHERE label = decode('${x}', 'hex')")
pending=$(psql_party 1 "SELECT COALESCE(SUM(pad_length), 0) FROM pads WHERE triggered_by = (SELECT triggered_by FROM pads WHERE label = decode('${x}', 'hex') LIMIT 1) AND signature IS NULL AND NOT is_error")
triggerers=$(psql_party 1 "SELECT count(DISTINCT triggered_by) FROM pads WHERE label = decode('${x}', 'hex')")
wait "${s1_pid}" || true
asks=$(jq -sc '[.[] | {status, digest, path}]' /tmp/prf-s1.jsonl)
row_end "S1 same label at 64 and 2048 bits, one process (party 1, key 3)" "2 rows, both by key 3; key 3's pending sum 2112 while both are pending; both 200 with different digests" \
  "rows ${rows}; triggered by ${triggerers} key(s); pending sum ${pending}; asks ${asks}" \
  "$(jq --arg rows "${rows}" --arg sum "${pending}" --arg t "${triggerers}" \
    'length == 2 and all(.status == 200) and .[0].digest != .[1].digest and $rows == "2" and $sum == "2112" and $t == "1"' <<<"${asks}")"

# B3: 3C new labels at once with one connector: never more than C evaluations at once (true by
# construction of the stand-in: it runs C workers), FIFO drain in about 48 x 1.1 / 16 = 3.3 s.
run_burst "B3 global cap: 48 new 64-bit labels at once, 1 connector (E 1.1 s, C 16)" "all ok; at most 16 at once" \
  burst --connectors 1 --users 48 --window 0s --retry-deadline 120s

# L1: 100 users in 10 s, every connector (model drain 100 x 1.1 / 16 = 6.9 s).
run_burst "L1 100 users in 10 s, 64-bit (E 1.1 s, C 16)" "all ok; model drain 6.9 s" \
  burst --users 100 --window 10s --retry-deadline 300s
l1_p50=$(jq '.latency_ms.p50' <<<"${last_burst}")
ladder="${last_verdict}"

# F3: decryptor fan-out: each connector asks for each label 2 or 3 times at once. The duplicates
# get 429 concurrency_limited (one request per connector and label) and join after Retry-After.
for copies in 2 3; do
  run_burst "F3 decryptor fan-out x${copies}: 100 users in 10 s, 64-bit (E 1.1 s)" "all ok; 429s and added latency measured (L1 p50 ${l1_p50} ms)" \
    burst --users 100 --window 10s --copies "${copies}" --retry-deadline 300s
done

# B5: the fastest connector absorbs its budget. Party 1 evaluates nothing while 600 new 2048-bit
# labels arrive with key 0 200 ms ahead: key 0 triggers 512 (1,048,576 bits), then gets 429, and
# the others trigger the rest. The other parties evaluate 0f labels at once.
kubectl scale deploy/coprocessor-1-standin-pad-worker -n "${NAMESPACE}" --replicas=0 >/dev/null
b5_t=$(now)
row_begin
out=$(connector burst --users 600 --window 10s --pad-length 2048 --label-prefix 0f --lead 200ms --retry-deadline 300s)
kubectl scale deploy/coprocessor-1-standin-pad-worker -n "${NAMESPACE}" --replicas=1 >/dev/null
kubectl rollout status deploy/coprocessor-1-standin-pad-worker -n "${NAMESPACE}" --timeout=300s >/dev/null || true
distribution=$(psql_party 1 "SELECT string_agg(n::text, ',' ORDER BY n DESC) FROM (SELECT count(*) AS n FROM pads WHERE get_byte(label, 0) = 15 AND created_at >= '${b5_t}' GROUP BY triggered_by) t")
row_end "B5 fastest connector absorbs its budget (party 1 not evaluating, 600 labels of 2048 bits, key 0 leads 200 ms)" \
  "all ok; key 0 triggers 512 at party 1, the rest by other keys" \
  "$(burst_summary "${out}"); rows triggered per key at party 1: ${distribution}" \
  "$(burst_correct "${out}")" "$(burst_saturated "${out}")" '^coprocessor-1-standin-pad-worker-'

# B4: the queue under a rising rate, 30 s per step (throughput model C / E = 14.5 labels/s).
for rate in 4 8 16 32; do
  run_burst "B4 ${rate} new labels/s for 30 s, 64-bit (E 1.1 s, C 16)" "all ok, misses served on retry; p99 against the 30 s deadline measured" \
    burst --users $((rate * 30)) --window 30s --retry-deadline 300s
done

# L2, then L4 (2048-bit) only if L2 left headroom (p99 under 15 s). The ladder stops at the first
# capacity event.
if [[ "${ladder}" != capacity ]]; then
  run_burst "L2 250 users in 10 s, 64-bit (E 1.1 s, C 16)" "all ok; model drain 17 s" \
    burst --users 250 --window 10s --retry-deadline 300s
  ladder="${last_verdict}"
else
  record "L2 250 users in 10 s" "run unless the ladder met capacity" "skipped: capacity in L1" untestable
fi
if [[ "${ladder}" == true ]] && jq -e '.latency_ms.p99 < 15000' <<<"${last_burst}" >/dev/null; then
  run_burst "L4 250 users in 10 s, 2048-bit (E 4.1 s, C 16)" "all ok, misses served on retry; model drain 64 s" \
    burst --users 250 --window 10s --pad-length 2048 --retry-deadline 300s
  ladder="${last_verdict}"
else
  record "L4 250 users in 10 s, 2048-bit" "only with headroom after L2" "skipped: L2 verdict ${ladder}, p99 $(jq '.latency_ms.p99' <<<"${last_burst}") ms" untestable
fi

# G1: a rogue (key 3) floods 600 new 2048-bit labels at party 2. It is bounded by its budget:
# 512 admitted (1,048,576 bits), 88 refused. Honest keys 0-2 keep retrieving, and party 2's own
# answers to key 0 queue behind the rogue's: about 512 / 16 x 4.1 = 131 s.
row_begin
connector --key-index 3 flood --target 1 --count 600 --label-prefix 00 --pad-length 2048 > /tmp/prf-g1-rogue.json &
rogue_pid=$!
sleep 5
connector --deadline 300s ask --target 1 --label-prefix 00 --repeat 4 --interval 20s > /tmp/prf-g1-party2.jsonl &
party2_pid=$!
honest=$(connector burst --connectors 3 --users 60 --window 60s --retry-deadline 300s)
wait "${rogue_pid}" "${party2_pid}" || true
rogue=$(cat /tmp/prf-g1-rogue.json)
party2=$(jq -sc '[.[] | {status, elapsed_ms, retried: (.retried | length)}]' /tmp/prf-g1-party2.jsonl)
row_end "G1 rogue floods 600 new 2048-bit labels at party 2 (key 3; E 4.1 s, C 16)" \
  "rogue: 88x 429, the rest 200 or 504; honest retrievals all ok; party 2's own answers measured (model 131 s)" \
  "rogue $(jq -c . <<<"${rogue}"); honest $(burst_summary "${honest}"); party 2 to key 0 ${party2}" \
  "$(jq -n --argjson r "${rogue}" --argjson h "$(burst_correct "${honest}")" --argjson p "${party2}" \
    '$h and ($r.statuses["429 concurrency_limited"] == 88) and ($r.statuses | keys | all(. == "200" or . == "504 timeout" or . == "429 concurrency_limited")) and ($p | length == 4 and all(.status == 200))')" \
  "$(burst_saturated "${honest}")"

# G3: a rogue (key 3) repeats stored 100 KB outputs as fast as it can at party 3, 128 loops for
# 60 s, GET then HEAD. Party 3's own answers to key 0 are measured meanwhile.
for method in GET HEAD; do
  row_begin
  connector --deadline 120s ask --target 2 --label-prefix 00 --repeat 10 --interval 5s > /tmp/prf-g3-party3.jsonl &
  party3_pid=$!
  rogue=$(connector --key-index 3 hammer --target 2 --label-prefix 0f --pad-length 2048 --method "${method}" --concurrency 128 --duration 60s)
  wait "${party3_pid}" || true
  party3=$(jq -sc '{asks: length, ok: ([.[] | select(.status == 200)] | length), elapsed_ms: ([.[].elapsed_ms] | sort), statuses: ([.[] | "\(.status // .error)"] | group_by(.) | map({key: .[0], value: length}) | from_entries)}' /tmp/prf-g3-party3.jsonl)
  row_end "G3 rogue repeats stored 100 KB outputs at party 3, ${method} (key 3, 128 loops, 60 s)" \
    "no 5xx; rate, bandwidth and party 3's own latency measured (no rate bound expected)" \
    "rogue $(jq -c . <<<"${rogue}"); party 3 to key 0 ${party3}" \
    "$(jq -n --argjson r "${rogue}" --argjson p "${party3}" "(\$r.statuses | keys | ${allowed_5xx}) and \$p.ok == \$p.asks")"
done

# L3: 1000 users in 10 s (model drain 1000 x 1.1 / 16 = 69 s: labels past 30 s miss and are
# served on retry). Only if the ladder has not met capacity.
if [[ "${ladder}" != capacity ]]; then
  run_burst "L3 1000 users in 10 s, 64-bit (E 1.1 s, C 16)" "all ok or served on retry; model drain 69 s" \
    burst --users 1000 --window 10s --retry-deadline 300s
else
  record "L3 1000 users in 10 s" "run unless the ladder met capacity" "skipped: capacity earlier in the ladder" untestable
fi

# O4: 500 slow-header and 2000 idle TLS connections at party 2's proxy for 120 s, while key 0
# keeps asking party 2. The proxy does not shed slow clients (that is the front layer's job).
row_begin
connector slowloris --target 1 --slow 500 --idle 2000 --duration 120s --byte-every 10s > /tmp/prf-o4.json &
slow_pid=$!
sleep 10
connector ask --target 1 --label-prefix 00 --repeat 20 --interval 5s > /tmp/prf-o4-party2.jsonl
wait "${slow_pid}" || true
slow=$(cat /tmp/prf-o4.json)
party2=$(jq -sc '{asks: length, ok: ([.[] | select(.status == 200)] | length), elapsed_ms: ([.[].elapsed_ms] | sort)}' /tmp/prf-o4-party2.jsonl)
row_end "O4 2500 slow or idle connections at party 2's proxy (120 s)" \
  "honest asks all 200; held connections, proxy memory and honest latency measured" \
  "slowloris $(jq -c . <<<"${slow}"); party 2 to key 0 ${party2}" \
  "$(jq '.ok == .asks and .asks == 20' <<<"${party2}")"

# K10: the proxy's own counters, read by the check pod (no NetworkPolicy covers the proxy pods).
k10=""
k10_ok=true
for p in $(seq 1 "${NB_COPROCESSOR}"); do
  ip=$(kubectl get pods -n "${NAMESPACE}" -l "app.kubernetes.io/name=coprocessor-${p}-coprocessor-proxy" -o jsonpath='{.items[0].status.podIP}')
  lines=$(connector metrics --url "http://${ip}:9100/metrics" --prefix coprocessor_proxy_requests_total 2>/dev/null | grep 'route="prf"' || true)
  if [[ -z "${lines}" ]]; then k10_ok=false; k10+="party ${p}: unreadable; "; continue; fi
  k10+="party ${p}: $(awk '{match($0, /status="[0-9]+"/); s = substr($0, RSTART + 8, 3); n[s] += $NF} END {for (s in n) printf "%s=%d ", s, n[s]}' <<<"${lines}"); "
done
if [[ "${k10_ok}" == true ]]; then
  record "K10 proxy PRF answers by status over Run B (since each proxy started)" "no 500 or 503" "${k10% ; }" \
    "$([[ "${k10}" != *500=* && "${k10}" != *503=* ]] && echo true)"
else
  record "K10 proxy PRF answers by status over Run B" "metrics readable from the check pod" "${k10% ; }" untestable
fi

# O2 and O3, last: copro expects prf-service to run out of memory. Party 3 evaluates nothing while
# 4 keys hold 1000 new 2048-bit labels each (4000 held requests), then evaluates them all at once
# (0f: no cost): 4000 waiters read 100 KB each.
kubectl scale deploy/coprocessor-3-standin-pad-worker -n "${NAMESPACE}" --replicas=0 >/dev/null
row_begin
connector --deadline 300s burst --only 2 --users 1000 --window 5s --pad-length 2048 --label-prefix 0f > /tmp/prf-o2.json &
o2_pid=$!
sleep 60
kubectl scale deploy/coprocessor-3-standin-pad-worker -n "${NAMESPACE}" --replicas=1 >/dev/null
wait "${o2_pid}" || true
o2=$(cat /tmp/prf-o2.json)
pool=$(kubectl logs deploy/coprocessor-3-prf-service -n "${NAMESPACE}" --since-time="${row_t}" 2>/dev/null | grep -c -i 'pool timed out\|PoolTimedOut' || true)
row_end "O2 4000 requests held at party 3, then served at once (2048-bit, 4 keys)" \
  "all 200 within 300 s; prf-service memory measured (capacity expected)" \
  "$(burst_summary "${o2}")" "$(burst_correct "${o2}")" "$(burst_saturated "${o2}")" '^coprocessor-3-standin-pad-worker-'
record "O3 DB pool pressure during O2 (pool of 16)" "503 count and pool timeouts measured" \
  "503s $(jq '[.statuses | to_entries[] | select(.key | test("^503")) | .value] | add // 0' <<<"${o2}"); pool timeout log lines ${pool}" measure

exit "${failed}"
