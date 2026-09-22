#!/usr/bin/env bash
# Reinitialize Solana state while retaining the running preview services and image pins.
set +x
set -euo pipefail
umask 077
: "${NAMESPACE:?}"
[[ "$NAMESPACE" == fhevm-ci-* && "$NAMESPACE" != fhevm-ci-solana-owner ]] || exit 2
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
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
bash "$script_dir/recover.sh" reset
while read -r release; do
  kubectl delete job "$release-deploy" -n "$NAMESPACE" --ignore-not-found --wait=true
  helm upgrade "$release" charts/contracts -n "$NAMESPACE" -f "$work/$release.yaml" \
    --set scDeploy.preventRedeployment=false --wait --wait-for-jobs --timeout=20m
done < "$work/releases"
echo 'Solana state reinitialized; preview services and image pins retained. Reseed the local demo before use.'
