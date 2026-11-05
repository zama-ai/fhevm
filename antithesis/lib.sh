# shellcheck shell=bash
# Shared by render.sh and mirror-images.sh so manifests and pushed images agree on names.

ANTITHESIS_REGISTRY="${ANTITHESIS_REGISTRY:-us-central1-docker.pkg.dev/molten-verve-216720/zama-repository}"

# Map a source image ref to its name in the Antithesis registry: drop the source registry
# host, the hub.zama.org proxy-cache prefixes and any digest (a single-platform re-push
# changes the digest, so the pushed tag is the stable reference).
antithesis_image() {
  local ref="${1%%@*}" first="${1%%/*}"
  if [[ "${ref}" == */* && ( "${first}" == *.* || "${first}" == *:* ) ]]; then
    ref="${ref#*/}"
  fi
  ref="${ref#ghcr/}"
  ref="${ref#cgr/}"
  ref="${ref#zama-ai/}"
  ref="${ref#library/}"
  echo "${ANTITHESIS_REGISTRY}/${ref}"
}
