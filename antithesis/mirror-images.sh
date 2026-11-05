#!/usr/bin/env bash
# Copy linux/amd64 images into the Antithesis registry under the names render.sh writes.
# Usage: mirror-images.sh < antithesis/images.txt   (written by render.sh; one source ref per line)
# Needs `docker login` to the source registries and to us-central1-docker.pkg.dev.
set -uo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

failed=()
while read -r src; do
  [[ -z "${src}" || "${src}" == \#* ]] && continue
  dst=$(antithesis_image "${src}")
  echo "==> ${src} -> ${dst}"
  # Registry-to-registry copy of the amd64 manifest; nothing goes through the local store.
  if docker buildx imagetools create --platform linux/amd64 --tag "${dst}" "${src}"; then
    continue
  fi
  failed+=("${src}")
done

if ((${#failed[@]})); then
  printf 'FAILED: %s\n' "${failed[@]}" >&2
  exit 1
fi
echo "all images mirrored"
