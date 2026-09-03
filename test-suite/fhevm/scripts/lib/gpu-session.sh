# Shared helpers for scripts that must work whether the workers are compose
# containers or GPU host units.
#
# Sourced, not executed. Callers must have REPO_ROOT set.
#
# Three separate scripts grew their own copies of this logic during the GPU work
# and the third one exposed why that was a mistake: the host-worker metrics URLs
# were plumbed into one runner only, so the failure matrix and the degraded-host
# suite both failed their baseline probe on a topology that was working fine --
# and the degraded-host message blamed the fleet for disagreeing.

readonly GPU_NODE_CONFIG="${FHEVM_STATE_DIR:-${REPO_ROOT}/.fhevm}/runtime/gpu-consensus-workers/node-config.env"

# systemd --user is addressed through this user's runtime directory. A detached
# or re-parented shell can inherit another user's values -- observed as
# XDG_RUNTIME_DIR=/run/user/0 while running as uid 1000 -- and then every
# systemctl call fails with "Permission denied", which reads as a unit problem
# rather than a wrong address.
gpu_normalise_user_bus() {
  local runtime="/run/user/$(id -u)"
  [[ -d "$runtime" ]] || return 0
  export XDG_RUNTIME_DIR="$runtime"
  export DBUS_SESSION_BUS_ADDRESS="unix:path=${runtime}/bus"
}

# True while GPU host units own the operators' queues.
gpu_session_active() { [[ -f "$GPU_NODE_CONFIG" ]]; }

# The unit serving a worker container's queue, or nothing for a container role.
#
# Keyed on the session, NOT on a unit's ActiveState: a cell that stops a unit and
# then heals it would otherwise find the unit inactive, fall back to Docker, and
# start a CPU worker beside a dead CUDA unit -- recreating B-1's mixed-backend
# fleet from inside the harness.
gpu_unit_for_container() {
  local container="$1" index kind
  gpu_session_active || return 0
  case "$container" in
    coprocessor-*)  index=0 ;;
    coprocessor[0-9]*-*) index="${container#coprocessor}"; index="${index%%-*}" ;;
    *) return 0 ;;
  esac
  case "$container" in
    *-tfhe-worker)    kind=tfhe ;;
    *-zkproof-worker) kind=zkproof ;;
    *-sns-worker)     kind=sns ;;
    *) return 0 ;;
  esac
  printf 'fhevm-gpu-consensus-%s-%s' "$kind" "$index"
}

gpu_unit_main_pid() { systemctl --user show "$1" --property=MainPID --value 2>/dev/null; }

# Comma-separated metrics URLs for the run-validity gate, index-ordered, or
# nothing when the workers are containers and container DNS already works.
#
# The launcher binds worker `i` on 19100 + i*10, and the host is the bridge
# gateway -- which lives on the network object, not the container: a container's
# own .Gateway is empty on user-defined networks and the test container reports
# no networks at all. Walk candidates rather than trusting one.
gpu_worker_metrics_urls() {          # gpu_worker_metrics_urls <count> [container...]
  gpu_session_active || return 0
  local count="$1"; shift
  local candidate net host_ip="" urls="" i
  for candidate in "$@"; do
    net="$(docker inspect "$candidate" \
      -f '{{range $k,$v := .NetworkSettings.Networks}}{{$k}}{{"\n"}}{{end}}' 2>/dev/null | head -1)"
    [[ -n "$net" ]] || continue
    host_ip="$(docker network inspect "$net" \
      -f '{{range .IPAM.Config}}{{.Gateway}}{{"\n"}}{{end}}' 2>/dev/null | grep -m1 -E '^[0-9]+\.')"
    [[ -n "$host_ip" ]] && break
  done
  [[ -n "$host_ip" ]] || return 0
  for (( i = 0; i < count; i++ )); do
    urls+="${urls:+,}http://${host_ip}:$(( 19100 + i * 10 ))/metrics"
  done
  printf '%s' "$urls"
}
