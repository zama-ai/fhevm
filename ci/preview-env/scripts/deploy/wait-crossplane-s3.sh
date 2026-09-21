#!/usr/bin/env bash
# Poll until exactly NB_COPROCESSOR Crossplane S3 claims matching NAME_PREFIX
# (default coprocessor-infra-) are Ready in NAMESPACE, else fail after a timeout.
#
# Filter by name prefix so unrelated claims (e.g. the KMS party bucket) don't
# count; the explicit count also rejects the "not created yet" empty-list case
# (which would otherwise look ready). `|| true` guards grep's exit 1 on 0 matches.
set -euo pipefail

: "${NAMESPACE:?NAMESPACE is required}"
: "${NB_COPROCESSOR:?NB_COPROCESSOR is required}"
name_prefix="${NAME_PREFIX:-coprocessor-infra-}"

max_wait=420; elapsed=0; interval=10
ready=false
expected="${NB_COPROCESSOR}"
while [[ $elapsed -lt $max_wait ]]; do
  rows=$(kubectl get s3 -n "${NAMESPACE}" --no-headers \
      -o custom-columns=NAME:.metadata.name,READY:'.status.conditions[?(@.type=="Ready")].status' 2>/dev/null \
      | grep "^${name_prefix}" || true)
  total=$(printf '%s' "${rows}" | grep -c . || true)
  ready_count=$(printf '%s\n' "${rows}" | awk '$2=="True"{c++} END{print c+0}')
  if [[ "${total}" -eq "${expected}" && "${ready_count}" -eq "${expected}" ]]; then
    ready=true; echo "coprocessor-infra buckets ready (${ready_count}/${expected})"; break
  fi
  echo "Still waiting for coprocessor-infra buckets... (${ready_count}/${expected} Ready, ${total} present, ${elapsed}s/${max_wait}s)"
  sleep "$interval"; elapsed=$((elapsed + interval))
done
kubectl get s3 -n "${NAMESPACE}" -o wide || true
if [[ "${ready}" != "true" ]]; then
  echo "::error::coprocessor-infra buckets not all Ready (${expected} expected) after ${max_wait}s - see conditions/events above (or below via kubectl describe)"
  kubectl describe s3 -n "${NAMESPACE}" || true
  exit 1
fi
