#!/usr/bin/env bash
# PRF spike (option A): the RFC-038 failure matrix of the coprocessor half. A stand-in connector
# (prf-standins standin-connector) signs as the KMS tx-senders and retrieves labels through every
# party's coprocessor-proxy; the stand-in pad-worker injects faults by label first byte (the
# table is pad_worker_faults in scripts/deploy/deploy-coprocessor.sh). Cluster faults use kubectl.
# Prints one line per scenario: expected, observed, PASS or FAIL. Fails if a scenario fails.
# Env: NAMESPACE, NB_COPROCESSOR, NB_KMS_CORE, HOST_CHAIN_ID, PRF_IMAGE.
set -euo pipefail

: "${NAMESPACE:?}" "${NB_COPROCESSOR:?}" "${NB_KMS_CORE:?}" "${HOST_CHAIN_ID:?}" "${PRF_IMAGE:?}"
pod=prf-endpoint-check
results=$(mktemp)
failed=0

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
psql_party() { kubectl exec "statefulset/postgres-coprocessor-${1}" -n "${NAMESPACE}" -- psql -U zama -d fhevm_e2e -tAc "$2"; }
pad_worker_logs() { kubectl logs "deploy/coprocessor-${1}-standin-pad-worker" -n "${NAMESPACE}" --all-containers 2>/dev/null || true; }
label_hex() { jq -r '.label' <<<"$1" | sed 's/^0x//'; }

# record <scenario> <expected> <observed> <pass: true|false> [evidence]
record() {
  local verdict=PASS
  if [[ "$4" != "true" ]]; then verdict=FAIL; failed=1; fi
  printf '| %s | %s | %s | %s | %s |\n' "$1" "$2" "$3" "${verdict}" "${5:-}" | tee -a "${results}"
}
summary() { jq -c '{ok, error, get_from, body_from, fallback, resigns, elapsed_ms, replies: [(.replies // [])[] | "\(.coprocessor):\(.method):\(.status // "conn"):\(.code // .attestation)"]}' <<<"$1"; }
wait_ready() {
  kubectl rollout status "$1" -n "${NAMESPACE}" --timeout=300s >/dev/null
}

echo "| scenario | expected | observed | verdict | evidence |"
echo "|---|---|---|---|---|"

# 1. Four connectors ask for the same label at once: one evaluation per coprocessor, and the row is
# triggered by one of the tx-senders that signed (prf-service recovered the real signer).
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
record "4 connectors, same label" "4 ok; 1 evaluation per coprocessor; triggered_by is a signer" \
  "${oks} ok; evaluations ${evaluations% }; triggered_by ok=${triggered_ok}" \
  "$([[ "${oks}" == "${NB_KMS_CORE}" && "${evaluations// /}" == "$(printf '1%.0s' $(seq 1 "${NB_COPROCESSOR}"))" && "${triggered_ok}" == true ]] && echo true)" "label 0x${label}"

# How often a healthy stack still needs the fallback GET: RFC-038 falls back when quorum forms
# before the GET target answered, and three HEADs are lighter than one GET.
for _ in $(seq 1 20); do connector fetch --connectors 1 --label-prefix 00; done > /tmp/prf-healthy.jsonl
out=$(jq -sc '{retrievals: length, ok: [.[] | select(.ok)] | length, fallback: [.[] | select(.fallback)] | length}' /tmp/prf-healthy.jsonl)
record "fallback rate, healthy stack, 64-bit pads" "body from the GET target in most retrievals (<= 20% fallback)" "${out}" \
  "$(jq '.ok == .retrievals and .fallback * 5 <= .retrievals' <<<"${out}")"

# fetch_case <scenario> <expected> <jq predicate on the report> <connector args...>
fetch_case() {
  local name="$1" expected="$2" predicate="$3"; shift 3
  local report
  report=$(connector fetch --connectors 1 "$@")
  record "${name}" "${expected}" "$(summary "${report}")" "$(jq "${predicate}" <<<"${report}")" "label $(jq -r .label <<<"${report}")"
}

fetch_case "slow pad-worker on the GET target (party 1, 10 s)" "ok via fallback GET" \
  '.ok and .fallback and .body_from != 0' --label-prefix d1 --get-from 0
fetch_case "1 wrong digest (party 1, GET target)" "outvoted, ok via fallback" \
  '.ok and .fallback and .body_from != 0' --label-prefix b1 --get-from 0
fetch_case "2 wrong digests (parties 1-2)" "outvoted, ok" '.ok' --label-prefix b2 --get-from 4
fetch_case "3 wrong digests (parties 1-3)" "no quorum, failed" '(.ok | not) and .error == "no_quorum"' --label-prefix b3
# Quorum can form before party 1's HEAD answers, and then the report has no reply from party 1:
# retry on a new label until it does.
for _ in 1 2 3 4 5; do
  report=$(connector fetch --connectors 1 --label-prefix a1 --get-from 1)
  jq -e '[(.replies // [])[] | select(.coprocessor == 0)] | length > 0' <<<"${report}" >/dev/null && break
done
record "1 unsigned attestation (party 1)" "rejected, ok" "$(summary "${report}")" \
  "$(jq '.ok and ([(.replies // [])[] | select(.coprocessor == 0 and .method == "HEAD") | .attestation] == ["rejected"])' <<<"${report}")" \
  "label $(jq -r .label <<<"${report}")"
fetch_case "3 unsigned attestations (parties 1-3)" "rejected, no quorum" '(.ok | not)' --label-prefix a5
fetch_case "expired FhevmSig headers" "401 auth_expired, re-sign, ok" '.ok and .resigns >= 3' --label-prefix 00 --stale-headers
fetch_case "permanent failure on the GET target (party 1)" "ok via fallback" '.ok and .fallback' --label-prefix e2 --get-from 0
fetch_case "permanent failure everywhere" "500 prf_evaluation_failed, not retried; label fails before the deadline" \
  '(.ok | not) and .error == "no_quorum" and ([(.replies // [])[] | select(.method == "GET") | .code] == ["prf_evaluation_failed"]) and ([(.replies // [])[] | .attempts] | max) == 1' --label-prefix e1 --get-from 0

# probe_case <scenario> <expected status> <expected code> <target> <path> [method]
probe_case() {
  local out
  out=$(connector probe --target "$4" --path "$5" --method "${6:-GET}")
  record "$1" "$2 $3" "$(jq -c . <<<"${out}")" \
    "$(jq --argjson s "$2" --arg c "$3" '.status == $s and ((.error.code // "") == $c)' <<<"${out}")"
}
rand_label() { od -An -N31 -tx1 /dev/urandom | tr -d ' \n'; }
l="00$(rand_label)"
probe_case "permanent failure, GET answer" 500 prf_evaluation_failed 1 "/v1/prf/0/labels/e1$(rand_label)/64"
probe_case "permanent failure, HEAD answer" 500 "" 1 "/v1/prf/0/labels/e1$(rand_label)/64" HEAD
# ask_case <scenario> <expected> <jq predicate on the reply> <connector ask args...>
ask_case() {
  local name="$1" expected="$2" predicate="$3"; shift 3
  local reply
  reply=$(connector ask "$@")
  record "${name}" "${expected}" "$(jq -c . <<<"${reply}")" "$(jq "${predicate}" <<<"${reply}")"
}
for method in GET HEAD; do
  ask_case "retry rule: 500 on ${method} stops" "1 attempt, nothing retried" \
    '.status == 500 and .attempts == 1 and .retried == []' --target 1 --label-prefix e1 --method "${method}"
done
probe_case "non-canonical path: uppercase label" 400 non_canonical_path 1 "/v1/prf/0/labels/AB$(rand_label | tr a-f A-F)/64"
probe_case "non-canonical path: leading zero" 400 non_canonical_path 1 "/v1/prf/00/labels/${l}/64"
probe_case "query string" 401 auth_query_string 1 "/v1/prf/0/labels/${l}/64?x=1"
probe_case "unknown prf" 404 unknown_prf 1 "/v1/prf/1/labels/${l}/64"
probe_case "padLength 0" 400 non_canonical_path 1 "/v1/prf/0/labels/${l}/0"
probe_case "padLength 2049" 400 non_canonical_path 1 "/v1/prf/0/labels/${l}/2049"
probe_case "padLength 2048" 200 "" 1 "/v1/prf/0/labels/${l}/2048"

# The cap of party 1: at most two pending 2048-bit pads per connector (4096 bits).
out=$(connector flood --target 0 --count 6 --label-prefix 4a --pad-length 2048)
record "per-connector cap (party 1)" '2x 200, 4x 429 concurrency_limited, Retry-After' "$(jq -c . <<<"${out}")" \
  "$(jq '.statuses == {"200": 2, "429 concurrency_limited": 4} and .retry_after != null' <<<"${out}")"
out=$(connector flood --target 1 --count 3 --label-prefix 4a --pad-length 64)
record "PRF_MAX_IN_FLIGHT_PER_CONNECTOR (party 2, 2 requests)" '2x 200, 1x 429 concurrency_limited' "$(jq -c . <<<"${out}")" \
  "$(jq '.statuses == {"200": 2, "429 concurrency_limited": 1}' <<<"${out}")"

# The pad-worker of parties 1-3 exits once with the row claimed; the reaper gives it back.
restarts_before=$(for p in 1 2 3; do kubectl get pods -n "${NAMESPACE}" -l "app=standin-pad-worker,fhevm.zama.ai/party=${p}" -o jsonpath='{.items[0].status.containerStatuses[0].restartCount}'; echo -n " "; done)
report=$(connector fetch --connectors 1 --label-prefix c1 --deadline 120s)
restarts_after=$(for p in 1 2 3; do kubectl get pods -n "${NAMESPACE}" -l "app=standin-pad-worker,fhevm.zama.ai/party=${p}" -o jsonpath='{.items[0].status.containerStatuses[0].restartCount}'; echo -n " "; done)
record "pad-worker killed mid-evaluation (parties 1-3)" "ok after restart and reap" "$(summary "${report}"); restarts ${restarts_before}-> ${restarts_after}" \
  "$(jq '.ok' <<<"${report}")"

# An evaluation longer than the connector's 30 s deadline, then the stored output.
report=$(connector fetch --connectors 1 --label-prefix 7f)
label=$(jq -r .label <<<"${report}")
sleep 15
again=$(connector fetch --connectors 1 --label "${label}")
record "evaluation of 40 s, connector deadline 30 s" "first fails at the deadline; the stored output is served after" \
  "first $(summary "${report}"); then $(summary "${again}")" \
  "$(jq -n --argjson a "${report}" --argjson b "${again}" '($a.error == "deadline") and $b.ok and $b.elapsed_ms < 5000')"

# prf-service of parties 1-3 is killed while it holds requests for a 20 s evaluation.
connector fetch --connectors 1 --label-prefix 6d --deadline 120s > /tmp/prf-held.json &
fetch_pid=$!
sleep 5
for p in 1 2 3; do
  kubectl delete pod -n "${NAMESPACE}" -l "app.kubernetes.io/name=coprocessor-${p}-prf-service" --wait=false
done
wait "${fetch_pid}" || true
report=$(cat /tmp/prf-held.json)
record "prf-service killed while holding requests (parties 1-3)" "ok: retries reach the new pods" "$(summary "${report}")" "$(jq '.ok' <<<"${report}")"
for p in 1 2 3; do wait_ready "deploy/coprocessor-${p}-prf-service"; done

# The proxy answers 502 upstream_transient while party 1 has no prf-service pod.
kubectl scale deploy/coprocessor-1-prf-service -n "${NAMESPACE}" --replicas=0 >/dev/null
kubectl wait --for=delete pod -l app.kubernetes.io/name=coprocessor-1-prf-service -n "${NAMESPACE}" --timeout=120s >/dev/null || true
connector ask --target 0 --label-prefix 00 --deadline 120s > /tmp/prf-502.json &
ask_pid=$!
sleep 10
kubectl scale deploy/coprocessor-1-prf-service -n "${NAMESPACE}" --replicas=1 >/dev/null
wait "${ask_pid}" || true
wait_ready deploy/coprocessor-1-prf-service
reply=$(cat /tmp/prf-502.json)
record "retry rule: proxy 502 is retried (party 1 prf-service down 10 s)" "502 upstream_transient retried, then 200" \
  "$(jq -c . <<<"${reply}")" "$(jq '.status == 200 and any((.retried // [])[]; . == "502 upstream_transient")' <<<"${reply}")"

# Proxy rolling restart under load: sequential retrievals until every proxy has restarted.
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
record "proxy rolling restart under load" "every retrieval ok" "${out}" "$(jq '.ok == .retrievals' <<<"${out}")"

# Coprocessors down: their proxies scaled to 0.
proxy_down() { kubectl scale "deploy/coprocessor-${1}-coprocessor-proxy" -n "${NAMESPACE}" --replicas=0 >/dev/null; }
replicas=$(kubectl get deploy/coprocessor-5-coprocessor-proxy -n "${NAMESPACE}" -o jsonpath='{.spec.replicas}')
proxy_down 5
fetch_case "1 coprocessor down (party 5, GET target)" "ok via fallback" '.ok and .fallback' --label-prefix 00 --get-from 4
proxy_down 4
fetch_case "2 coprocessors down (parties 4-5)" "ok via fallback" '.ok and .fallback' --label-prefix 00 --get-from 3
proxy_down 3
fetch_case "3 coprocessors down (parties 3-5)" "no quorum before the deadline: retryable failure" '(.ok | not) and .error == "deadline"' --label-prefix 00
for p in 3 4 5; do
  kubectl scale "deploy/coprocessor-${p}-coprocessor-proxy" -n "${NAMESPACE}" --replicas="${replicas}" >/dev/null
  wait_ready "deploy/coprocessor-${p}-coprocessor-proxy"
done

# The Postgres of party 5 is down.
kubectl scale statefulset/postgres-coprocessor-5 -n "${NAMESPACE}" --replicas=0 >/dev/null
kubectl wait --for=delete pod/postgres-coprocessor-5-0 -n "${NAMESPACE}" --timeout=120s >/dev/null || true
fetch_case "Postgres of one coprocessor down (party 5, GET target)" "ok via fallback" '.ok and .fallback' --label-prefix 00 --get-from 4
kubectl scale statefulset/postgres-coprocessor-5 -n "${NAMESPACE}" --replicas=1 >/dev/null
kubectl rollout status statefulset/postgres-coprocessor-5 -n "${NAMESPACE}" --timeout=300s >/dev/null
wait_ready deploy/coprocessor-5-prf-service

# An evaluation of 130 s outlives the proxy's 120 s endpoint_response_timeout: the proxy answers
# 504 timeout, and the retry joins the same evaluation.
ask_case "retry rule: proxy 504 is retried (evaluation 130 s, proxy timeout 120 s)" "504 timeout retried, then 200" \
  '.status == 200 and .retried == ["504 timeout"]' --target 1 --label-prefix f0 --deadline 200s

kubectl delete pod "${pod}" -n "${NAMESPACE}" --wait=false >/dev/null
if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
  { echo "### PRF endpoint failure matrix"; echo; echo "| scenario | expected | observed | verdict | evidence |"; echo "|---|---|---|---|---|"; cat "${results}"; } >> "${GITHUB_STEP_SUMMARY}"
fi
exit "${failed}"
