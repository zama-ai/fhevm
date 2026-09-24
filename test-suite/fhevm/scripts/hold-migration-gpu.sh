#!/usr/bin/env bash
# Parent stdin owns this temporary backend swap. Restore exact original images
# even on parent death; keep private snapshots if any restoration fails.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/lib/source-revision.sh"
: "${SC_RESTORE_LOG:?}"
: "${RFC029_GPU_IMAGES:?package-migration receipt required}"
root="$(dirname "$SC_RESTORE_LOG")"
umask 077
receipt="$root/gpu-images.json"
cp "$RFC029_GPU_IMAGES" "$receipt"
revision="$(sr_revision "$REPO_ROOT")"
[[ "$(jq -r .revision "$receipt")" == "$revision" ]] || { echo 'GPU image source differs from checkout' >&2; exit 1; }
device="$(jq -r .device "$receipt")"
[[ "$device" =~ ^GPU-[a-f0-9-]+$ ]] || exit 2
[[ "$(nvidia-smi --id="$device" --query-gpu=compute_cap --format=csv,noheader | tr -d '. ')" == "$(jq -r .capability "$receipt")" ]] || exit 1
compose=()
changed=()
cleanup() {
  local status=$? target
  trap - EXIT
  trap '' INT TERM
  for target in "${changed[@]}"; do
    if ! docker "${compose[@]}" -f "$root/original-images.json" up -d --no-deps --force-recreate "$target" >&2; then status=1; continue; fi
    docker inspect "$target" > "$root/$target.restored.json" || { status=1; continue; }
    jq -e -s '.[0][0].Image == .[1][0].Image and .[0][0].Config.Cmd == .[1][0].Config.Cmd and (.[0][0].Config.Env|sort)==(.[1][0].Config.Env|sort) and .[1][0].State.Running' \
      "$root/$target.json" "$root/$target.restored.json" >/dev/null || status=1
  done
  [[ "$status" == 0 ]] || echo "GPU migration restoration failed; retain $root and repair before stack reuse" >&2
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
# Preflight every participant and image before stopping a single worker.
printf '{"services":{}}\n' > "$root/original-images.json"
printf '{"services":{}}\n' > "$root/gpu.json"
for operator in '' 1; do
  for role in tfhe-worker zkproof-worker sns-worker; do
    target="coprocessor${operator}-gcs-$role"
    docker inspect "$target" > "$root/$target.json"
    jq -e '.[0].State.Running and (.[0].Config.Env|index("FORCE_LEGACY_SERVER_KEY=true") != null)' "$root/$target.json" >/dev/null
    project="$(jq -r '.[0].Config.Labels["com.docker.compose.project"]' "$root/$target.json")"
    files="$(jq -r '.[0].Config.Labels["com.docker.compose.project.config_files"]' "$root/$target.json")"
    [[ -n "$project" && "$project" != null && -n "$files" && "$files" != null ]] || exit 1
    if [[ ${#compose[@]} == 0 ]]; then
      original_project="$project"; original_files="$files"
      compose=(compose -p "$project")
      IFS=',' read -r -a configs <<< "$files"
      for config in "${configs[@]}"; do [[ -f "$config" ]] || exit 1; compose+=(-f "$config"); done
      versions="${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}/runtime/env/versions.env"
      [[ ! -f "$versions" ]] || compose+=(--env-file "$versions")
    fi
    [[ "$project" == "$original_project" && "$files" == "$original_files" ]] || exit 1
    image="$(jq -r --arg role "$role" '.images[$role]' "$receipt")"
    [[ "$image" =~ ^sha256:[a-f0-9]{64}$ ]] || exit 1
    docker image inspect "$image" > "$root/$role.image.json"
    jq -e --arg revision "$revision" '.[0].Config.Labels | .["ai.zama.fhevm.gpu"]=="true" and .["org.opencontainers.image.revision"]==$revision' "$root/$role.image.json" >/dev/null
    jq --arg target "$target" --arg image "$(jq -r '.[0].Image' "$root/$target.json")" '.services[$target]={image:$image}' "$root/original-images.json" > "$root/next"
    mv "$root/next" "$root/original-images.json"
    jq --arg target "$target" --arg image "$image" --arg device "$device" \
      '.services[$target]={image:$image, environment:{FORCE_LEGACY_SERVER_KEY:"false",NVIDIA_VISIBLE_DEVICES:$device,NVIDIA_DRIVER_CAPABILITIES:"compute,utility"},deploy:{resources:{reservations:{devices:[{driver:"nvidia",device_ids:[$device],capabilities:["gpu"]}]}}}}' "$root/gpu.json" > "$root/next"
    mv "$root/next" "$root/gpu.json"
  done
done
bun "$SCRIPT_DIR/gpu-key-readiness.ts" >&2
mapfile -t targets < <(jq -r '.services|keys[]' "$root/gpu.json")
changed=("${targets[@]}")
docker stop "${targets[@]}" >&2
docker "${compose[@]}" -f "$root/gpu.json" up -d --no-deps --force-recreate "${targets[@]}" >&2
for target in "${targets[@]}"; do
  docker inspect "$target" > "$root/$target.gpu.json"
  jq -e '.[0].State.Running and (.[0].Config.Env|index("FORCE_LEGACY_SERVER_KEY=false") != null)' "$root/$target.gpu.json" >/dev/null
done
printf 'ROLLOUT_HOLD_READY\n'
IFS= read -r command || exit 1
[[ "$command" == release ]] || exit 2
