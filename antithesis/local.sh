#!/usr/bin/env bash
# Deploy the rendered manifests to a local single-node k3s (k3d) with kapp, as Antithesis
# does, to iterate on setup before using the tenant. Run render.sh first.
# Usage: antithesis/local.sh [up|down]     Needs k3d, kapp, kubectl, docker, yq, jq, and
# `docker login us-central1-docker.pkg.dev` (the node pulls from the Antithesis registry).
# Watch progress: kubectl -n fhevm logs -f job/setup-milestones
# It cannot reproduce what only Antithesis does: deterministic CPU and clock, no internet,
# a 15 min kapp timeout, fault injection.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cluster=fhevm-antithesis
registry=us-central1-docker.pkg.dev
work="$(mktemp -d)"
trap 'rm -rf "${work}"' EXIT

if [[ "${1:-up}" == down ]]; then
  k3d cluster delete "${cluster}"
  exit 0
fi

if ! k3d cluster get "${cluster}" >/dev/null 2>&1; then
  # Docker Desktop's credential store holds the registry key; it never enters the repo.
  echo "${registry}" | docker-credential-desktop get | jq -r --arg r "${registry}" \
    '"configs:\n  \"\($r)\":\n    auth:\n      username: \(.Username | @json)\n      password: \(.Secret | @json)"' \
    >"${work}/registries.yaml"
  k3d cluster create "${cluster}" --agents 0 --no-lb --registry-config "${work}/registries.yaml" \
    --k3s-arg "--disable=traefik@server:0" --wait
fi

# The node is the host's arch: amd64 k3s under Rosetta has no seccomp, so on Apple
# silicon it is arm64. containerd there rejects an index with no arm64 entry, so pin
# each index to its amd64 manifest (same bytes as on Antithesis), and drop memory
# limits that emulation overshoots (add-host-chains OOMs at 512Mi under Rosetta).
cp -R "${here}/config/manifests" "${work}/manifests"
containers='(select(.spec.template.spec != null)
  | (.spec.template.spec.containers[], .spec.template.spec.initContainers[]))'
for ref in $(yq -N '.spec.template.spec | (.containers[], .initContainers[]) | .image' \
  "${work}"/manifests/*.yaml | grep -v -e '^---$' -e '^null$' | sort -u); do
  digest=$(docker buildx imagetools inspect "${ref}" --raw |
    jq -r '.manifests[]? | select(.platform.architecture == "amd64") | .digest' | head -1)
  [[ -n "${digest}" ]] || continue
  while IFS= read -r f; do
    REF="${ref}" PIN="${ref}@${digest}" yq -i \
      "${containers} |= select(.image == strenv(REF)).image = strenv(PIN)" "${f}"
  done < <(grep -l -F "${ref}" "${work}"/manifests/*.yaml)
done
for f in "${work}"/manifests/*.yaml; do
  yq -i "${containers} |= del(.resources.limits.memory)" "${f}"
done

# Jobs are immutable: replace what changed when redeploying onto an existing cluster.
kapp deploy -a fhevm -f "${work}/manifests" -y --wait-timeout 60m \
  --kubeconfig-context "k3d-${cluster}" --apply-default-update-strategy fallback-on-replace
