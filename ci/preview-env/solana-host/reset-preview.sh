#!/usr/bin/env bash
# Reinitialize Solana state while retaining the running preview services and image pins.
set +x
set -euo pipefail
umask 077
: "${NAMESPACE:?}"
[[ "$NAMESPACE" == fhevm-ci-* && "$NAMESPACE" != fhevm-ci-solana-owner ]] || exit 2
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=ci/preview-env/scripts/lib.sh
source "${script_dir}/../scripts/lib.sh"
# shellcheck source=ci/preview-env/solana-host/ownership.sh
source "$script_dir/ownership.sh"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
solana_acquire
trap 'solana_release_operation; rm -rf "$work"' EXIT
helm list -n "$NAMESPACE" -o json | python3 -c '
import json,re,sys
names=[r["name"] for r in json.load(sys.stdin)]
registrations=sorted(n for n in names if re.fullmatch(r"solana-register-coprocessor-[0-9]+", n))
if "solana-host" not in names or not registrations:
    sys.exit("An initialized Solana preview is required before reset")
print("\n".join(["solana-host"]+registrations+(["solana-demos"] if "solana-demos" in names else [])))
' > "$work/releases"
while read -r release; do
  helm get values "$release" -n "$NAMESPACE" -o yaml > "$work/$release.yaml"
  kubectl get job "$release-deploy" -n "$NAMESPACE" --ignore-not-found -o json | python3 -c '
import json,sys
data=sys.stdin.read().strip()
if not data: sys.exit(0)
job=json.loads(data)
if not any(c["type"] in ("Complete", "Failed") and c["status"] == "True" for c in job.get("status",{}).get("conditions",[])):
    sys.exit("A Solana bootstrap Job is still active; refusing reset")
'
done < "$work/releases"
parties=$(sed -n 's/^solana-register-coprocessor-//p' "$work/releases")
# Checked before the wipe, so a preview without an indexer is refused instead of left wiped.
for party in $parties; do
  kubectl get "deployment/coprocessor-$party-solana-merkle-indexer" -n "$NAMESPACE" -o name >/dev/null
done
bash "$script_dir/recover.sh" reset
for party in $parties; do recreate_solana_merkle_record "$party"; done
merkle_start_slot=$(finalized_solana_slot)
while read -r release; do
  kubectl delete job "$release-deploy" -n "$NAMESPACE" --ignore-not-found --wait=true
  helm upgrade "$release" charts/contracts -n "$NAMESPACE" -f "$work/$release.yaml" \
    --set scDeploy.preventRedeployment=false --wait --wait-for-jobs --timeout=20m
done < "$work/releases"
# The indexers start only now: they read HostConfig first, which the wipe closed and the
# solana-host upgrade recreates. The chart reads the start slot from this env var, so the reset
# needs no coprocessor chart; the next deploy-preview.sh sets it again through Helm.
for party in $parties; do
  indexer="deployment/coprocessor-$party-solana-merkle-indexer"
  kubectl set env "$indexer" -n "$NAMESPACE" "SOLANA_MERKLE_START_SLOT=$merkle_start_slot" >/dev/null
  kubectl scale "$indexer" -n "$NAMESPACE" --replicas=1 >/dev/null
  kubectl rollout status "$indexer" -n "$NAMESPACE" --timeout=10m
done
for party in $parties; do wait_solana_merkle_recorded "$party" "$merkle_start_slot"; done
echo 'Solana state reinitialized; preview services and image pins retained. Reseed the local demo before use.'
