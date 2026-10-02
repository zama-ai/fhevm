# shellcheck shell=bash
# GPU02 alone wraps the CLI Docker boundary. All other CLI invocations retain
# their normal transport, and no production module imports harness code.
gpu02_transport_init() {
  GPU02_REAL_DOCKER="$(type -P docker)" || return 1
  [[ -n "$GPU02_REAL_DOCKER" ]] || return 1
  GPU02_MIGRATION_REGISTRY="$SP_RUNTIME_DIR/gpu02-migrations"
  GPU02_PHASE_HELPER="$SCRIPT_DIR/lib/container-phase.cjs"
  GPU02_SHIM_DIR="$SP_RUNTIME_DIR/gpu02-bin"
  mkdir -p "$GPU02_SHIM_DIR" || return 1
  : > "$GPU02_MIGRATION_REGISTRY" || return 1
  ln -s "$SCRIPT_DIR/lib/gpu02-docker.cjs" "$GPU02_SHIM_DIR/docker" || return 1
  export GPU02_REAL_DOCKER GPU02_MIGRATION_REGISTRY GPU02_PHASE_HELPER
  export SP_RUNTIME_DIR SP_PHASE_REGISTRY SP_CONTAMINATION SC_RESTORE_LOG
}

gpu02_stop_remote() {
  [[ -n "${GPU02_MIGRATION_REGISTRY:-}" ]] || return 0
  touch "$SP_RUNTIME_DIR/cancelling" || return 1
  local names name state owner
  names="$(cat "$GPU02_MIGRATION_REGISTRY")" || { sp_mark_unquiesced; return 1; }
  while IFS= read -r name; do
    [[ -n "$name" ]] || continue
    [[ "$name" =~ ^gpu02-revert-[0-9]+-[0-9]+$ ]] || { sp_mark_unquiesced; return 1; }
    owner="$(hc_run "$GPU02_REAL_DOCKER" inspect -f '{{index .Config.Labels "fhevm.gpu02-owner"}}' "$name")" || { sp_mark_unquiesced; return 1; }
    [[ "$owner" == "$SP_RUNTIME_DIR" ]] || { sp_mark_unquiesced; return 1; }
    state="$(hc_run "$GPU02_REAL_DOCKER" inspect -f '{{.State.Status}} {{.State.Running}} {{.State.Pid}}' "$name")" || { sp_mark_unquiesced; return 1; }
    if [[ "$state" == 'created false 0' && ! -f "${GPU02_MIGRATION_REGISTRY}.${name}.start-requested" ]]; then continue; fi
    # A created container with an unacknowledged start may still be launched by
    # the daemon. It is not terminal evidence, even though its PID is zero.
    if [[ ! "$state" =~ ^(exited|dead)\ false\ 0$ ]]; then
      hc_run "$GPU02_REAL_DOCKER" stop -t 2 "$name" >/dev/null || true
      state="$(hc_run "$GPU02_REAL_DOCKER" inspect -f '{{.State.Status}} {{.State.Running}} {{.State.Pid}}' "$name")" || { sp_mark_unquiesced; return 1; }
      [[ "$state" =~ ^(exited|dead)\ false\ 0$ ]] || { sp_mark_unquiesced; return 1; }
    fi
    hc_run node "$SCRIPT_DIR/lib/gpu02-postgres.cjs" "$name" || { sp_mark_unquiesced; return 1; }
  done <<<"$names"
  sp_cancel_all && sp_recover_suite_state || return 1
}

gpu02_dispose_remote() {
  [[ -n "${GPU02_MIGRATION_REGISTRY:-}" ]] || return 0
  local names name
  names="$(cat "$GPU02_MIGRATION_REGISTRY")" || return 1
  while IFS= read -r name; do
    [[ -n "$name" ]] || continue
    hc_run "$GPU02_REAL_DOCKER" rm "$name" >/dev/null || return 1
  done <<<"$names"
  : > "$GPU02_MIGRATION_REGISTRY"
}
