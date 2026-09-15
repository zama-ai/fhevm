# shellcheck shell=bash
# Own every in-container suite, including executions launched in substitutions
# or background shells. Register before launch so an abort cannot miss a phase.
[[ "${SP_HELPERS_LOADED:-0}" == 1 ]] && return 0
readonly SP_HELPERS_LOADED=1
source "${SCRIPT_DIR}/lib/case-deadline.sh"
source "${SCRIPT_DIR}/lib/host-command.sh"

sp_init() {
  SP_OWNS_RUNTIME=0
  if [[ -z "${SP_RUNTIME_DIR:-}" ]]; then SP_RUNTIME_DIR="$(mktemp -d)"; SP_OWNS_RUNTIME=1; fi
  SP_INHERITED_DEADLINE_EPOCH="${CASE_DEADLINE_EPOCH:-}"
  mkdir -p "$SP_RUNTIME_DIR" || return 1
  SP_PHASE_REGISTRY="${SP_PHASE_REGISTRY:-$SP_RUNTIME_DIR/phases}"
  touch "$SP_PHASE_REGISTRY" || return 1
  SP_OWNER_PID="$$"
  SP_FORCED_STOP=0
  SP_CONTAMINATION="${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}/runtime/failure-matrix/uncancelled-phase"
}

# A new case is the only admission boundary. First finish every recovery owner
# from the prior case; a failed cleanup leaves admission closed for retry/EXIT.
sp_case_ready() {
  [[ ! -f "$SP_CONTAMINATION" ]] || return 1
  if [[ -f "$SP_RUNTIME_DIR/case-active" ]]; then
    hc_begin_cleanup || return 1
    sp_cancel_all && sp_recover_suite_state || return 1
    if declare -F sp_case_cleanup >/dev/null; then sp_case_cleanup || return 1; fi
    sc_run_restores || return 1
    if [[ -n "${SC_CASE_BASELINE:-}" && -f "$SC_CASE_BASELINE" ]]; then
      sc_restore_running "$SC_CASE_BASELINE" || return 1
    fi
    [[ ! -s "${SC_RESTORE_LOG:-/dev/null}" ]] || return 1
    # Only the owning runner can advance its cases. A delegated child cannot
    # reopen admission which its parent closed to enforce an outer deadline.
    [[ "${SP_OWNS_RUNTIME:-0}" == 1 ]] || return 1
    : > "$SP_PHASE_REGISTRY" || return 1
    rm -f "$SP_RUNTIME_DIR/cancelling" "$SP_RUNTIME_DIR/forced-stop"
  elif [[ -f "$SP_RUNTIME_DIR/cancelling" ]]; then
    return 1
  fi
  touch "$SP_RUNTIME_DIR/case-active"
}

sp_case_start() {
  sp_case_ready || { echo "suite-process: prior case recovery incomplete; refusing another case" >&2; return 1; }
  local budget total=0 case_id
  for case_id in "$@"; do
    budget="$(bun "$SCRIPT_DIR/consensus-inventory.ts" show "$case_id" | sed -n 's/^timeout: *\([0-9]*\)s$/\1/p')" || return 1
    [[ "$budget" =~ ^[1-9][0-9]*$ ]] || return 1
    total=$((total + budget))
  done
  case_deadline_start "$total" || return 1
  if [[ -n "$SP_INHERITED_DEADLINE_EPOCH" && "$SP_INHERITED_DEADLINE_EPOCH" -lt "$CASE_DEADLINE_EPOCH" ]]; then
    CASE_DEADLINE_EPOCH="$SP_INHERITED_DEADLINE_EPOCH"
  fi
  hc_arm_deadline
}

# Same arguments as `docker exec`. The supervised command must be foreground;
# callers may background sp_exec itself when a handshake coordinates the fault.
sp_exec() {
  local -a options=()
  while [[ $# -gt 0 ]]; do
    case "$1" in
      -e|--env|--env-file|-u|--user|-w|--workdir) options+=("$1" "${2:?missing docker exec option value}"); shift 2 ;;
      -i|-t|-it|--interactive|--tty|--privileged) options+=("$1"); shift ;;
      --) shift; break ;;
      -*) echo "suite-process: unsupported docker exec option $1" >&2; return 2 ;;
      *) break ;;
    esac
  done
  local container="${1:?missing test container}"; shift
  # Tests and recovery-only fixtures may omit a run ID; live E2E emitters refuse
  # that omission instead of creating anonymous evidence.
  [[ -z "${CR_RUN_ID:-${CONSENSUS_RUN_ID:-}}" ]] || options+=(-e "CONSENSUS_RUN_ID=${CR_RUN_ID:-$CONSENSUS_RUN_ID}")
  [[ $# -gt 0 ]] || return 2
  [[ "$container" =~ ^[a-zA-Z0-9_.-]+$ ]] || return 2
  [[ ! -f "$SP_RUNTIME_DIR/cancelling" ]] || return 143
  local token remaining deadline status=0 client
  token="phase_${BASHPID}_${RANDOM}_$(date +%s%N)"
  remaining="${SP_PHASE_TIMEOUT_SECONDS:-2400}"
  [[ "$remaining" =~ ^[1-9][0-9]*$ ]] || return 2
  if [[ -n "${CASE_DEADLINE_EPOCH:-}" ]]; then
    local case_remaining
    case_remaining="$(case_seconds_left)" || return 124
    if [[ -z "${SP_PHASE_TIMEOUT_SECONDS:-}" || "$remaining" -gt "$case_remaining" ]]; then remaining="$case_remaining"; fi
  fi
  deadline="$(( ($(date +%s) + remaining) * 1000 ))"
  printf '%s|%s\n' "$container" "$token" >> "$SP_PHASE_REGISTRY" || return 1
  # The parent closes admission before reading the registry. A phase registered
  # concurrently with cancellation may never start work afterwards.
  [[ ! -f "$SP_RUNTIME_DIR/cancelling" ]] || return 143
  HC_COMMAND_TIMEOUT_SECONDS="$((remaining + 8))" hc_run node -e "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" run "${token}_client" "$((deadline + 8000))" \
    bash -c 'set -o pipefail; evidence="$1"; capture="$2"; shift 2; "$@" | node "$capture" "$evidence"' _ \
    "$SP_RUNTIME_DIR/assertion-evidence.jsonl" "$SCRIPT_DIR/lib/capture-assertions.cjs" \
    timeout --kill-after=2s "$((remaining + 8))s" docker exec "${options[@]}" "$container" \
    node -e "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" run "$token" "$deadline" "$@" &
  client=$!
  if [[ -f "$SP_RUNTIME_DIR/cancelling" ]]; then sp_stop_client "$token" || return 1; fi
  wait "$client" || status=$?
  if [[ "$status" != 0 && ! -f "$SP_RUNTIME_DIR/$token.verified" ]] && ! sp_cancel_one "$container" "$token"; then
    sp_mark_unquiesced
    # The caller may otherwise enter a fault-healing error branch. Its EXIT
    # handler must prove quiescence before allowing any restoration.
    kill -TERM "$SP_OWNER_PID" 2>/dev/null || true
    return 125
  fi
  if [[ "$status" != 0 && ! -f "$SP_RUNTIME_DIR/cancelling" ]] && { ! sp_cancel_all || ! sp_recover_suite_state; }; then
    kill -TERM "$SP_OWNER_PID" 2>/dev/null || true
    return 125
  fi
  return "$status"
}

sp_stop_client() {
  local token="$1"
  # The background PID belongs to the Bash hc_run wrapper, whose argv does not
  # contain the phase token. Own the actual transport process group separately;
  # a prelaunch cancellation tombstone also closes the spawn/publication race.
  hc_run timeout --kill-after=2s 20s node -e \
    "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" cancel "${token}_client"
}

sp_mark_unquiesced() {
  mkdir -p "$(dirname "$SP_CONTAMINATION")"
  printf 'phase_registry=%s\nrestore_log=%s\n' "$SP_PHASE_REGISTRY" "${SC_RESTORE_LOG:-}" > "$SP_CONTAMINATION"
  echo "suite-process: cancellation unverified; refusing recovery while the test may still run ($SP_CONTAMINATION)" >&2
}

sp_cancel_one() {
  local container="$1" token="$2"
  if hc_run timeout --kill-after=2s 20s docker exec "$container" node -e \
    "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" cancel "$token"; then touch "$SP_RUNTIME_DIR/$token.verified"; return 0; fi
  echo "suite-process: cannot prove phase $token stopped; stopping $container before recovery" >&2
  SP_FORCED_STOP=1
  touch "$SP_RUNTIME_DIR/forced-stop"
  hc_run timeout --kill-after=2s 20s docker stop -t 2 "$container" >/dev/null 2>&1 || true
  [[ "$(hc_run timeout --kill-after=2s 10s docker inspect -f '{{.State.Running}} {{.State.Pid}}' "$container" 2>/dev/null)" == 'false 0' ]] || return 1
  touch "$SP_RUNTIME_DIR/$token.verified"
}

# Return zero only when no owned suite can still act on the stack. A verified
# container stop is a safe fallback, but SP_FORCED_STOP makes the run fail.
sp_cancel_all() {
  [[ -n "${SP_PHASE_REGISTRY:-}" ]] || return 0
  touch "$SP_RUNTIME_DIR/cancelling" || return 1
  local container token status=0 entries
  if [[ ! -f "$SP_PHASE_REGISTRY" ]] || ! entries="$(cat "$SP_PHASE_REGISTRY")"; then
    sp_mark_unquiesced; return 1
  fi
  while IFS='|' read -r container token; do
    [[ -n "$container" || -n "$token" ]] || continue
    if [[ ! "$container" =~ ^[a-zA-Z0-9_.-]+$ || ! "$token" =~ ^[a-zA-Z0-9_.-]+$ ]]; then
      status=1; continue
    fi
    sp_cancel_one "$container" "$token" || status=1
    sp_stop_client "$token" || status=1
  done <<<"$entries"
  [[ ! -f "$SP_RUNTIME_DIR/forced-stop" ]] || SP_FORCED_STOP=1
  if [[ "$status" != 0 ]]; then sp_mark_unquiesced; return 1; fi
  # Only remove our own marker; another invocation's unresolved work stays fatal.
  if [[ -f "$SP_CONTAMINATION" ]] && grep -Fxq "phase_registry=$SP_PHASE_REGISTRY" "$SP_CONTAMINATION"; then
    rm -f "$SP_CONTAMINATION"
  fi
  if [[ -n "${CR_SUITE_PID:-}" ]]; then
    wait "$CR_SUITE_PID" 2>/dev/null || true
    CR_SUITE_PID=""
  fi
  return 0
}

# A recovery operation may start after workload admission closes. It remains
# registered, bounded and cancellable; failed recovery never permits healing.
sp_recovery_exec() {
  local container="${1:?missing recovery container}"; shift
  local token deadline status=0
  token="recovery_${BASHPID}_${RANDOM}_$(date +%s%N)"
  deadline="$(( ($(date +%s) + 120) * 1000 ))"
  printf '%s|%s\n' "$container" "$token" >> "$SP_PHASE_REGISTRY" || return 1
  HC_COMMAND_TIMEOUT_SECONDS=128 hc_run timeout --kill-after=2s 128s docker exec "$container" node -e \
    "$(cat "$SCRIPT_DIR/lib/container-phase.cjs")" run "$token" "$deadline" "$@" || status=$?
  if [[ "$status" != 0 ]]; then
    sp_cancel_one "$container" "$token" || true
    sp_mark_unquiesced
    echo "suite-process: suite-state recovery failed; retaining journal and fault owners" >&2
    return 1
  fi
}

# JS finally/after hooks do not run after a hard kill. Restore the private
# durable canary/mining journal while the test is gone and workers remain held.
sp_recover_suite_state() {
  [[ -n "${SP_PHASE_REGISTRY:-}" ]] || return 0
  local entries
  if [[ ! -f "$SP_PHASE_REGISTRY" ]] || ! entries="$(cat "$SP_PHASE_REGISTRY")"; then
    sp_mark_unquiesced; return 1
  fi
  local -a containers=()
  mapfile -t containers < <(cut -d'|' -f1 <<<"$entries" | sort -u)
  local container
  for container in "${containers[@]}"; do
    [[ -n "$container" ]] || continue
    if [[ -f "$SP_RUNTIME_DIR/forced-stop" ]]; then
      # Reuse the same idle harness filesystem: its private recovery journal
      # must not be discarded by replacing the container after cancellation.
      hc_run timeout --kill-after=2s 30s docker start "$container" >/dev/null || { sp_mark_unquiesced; return 1; }
    fi
    sp_restore_recreated "$container" || { sp_mark_unquiesced; return 1; }
    sp_recovery_exec "$container" node -r ts-node/register/transpile-only -e \
      'require("./test/consensus/abortRecovery").recoverAbortedSuite().catch(e=>{console.error(e.message);process.exitCode=1})' || return 1
  done
}

# Only after cancellation and fault restoration both succeeded.
sp_dispose() {
  hc_stop_timer
  [[ -f "${SP_RUNTIME_DIR:-}/cancelling" ]] || return 1
  [[ "${SP_OWNS_RUNTIME:-0}" == 1 ]] || return 0
  rm -rf "$SP_RUNTIME_DIR"
}

# A replacement destroys the old harness filesystem. Quiesce its owned work,
# then snapshot both recovery journals and handshakes before Compose can do so.
# The private backup stays outside artifacts until copying it back is verified.
sp_snapshot_recreate() {
  local container="$1" handshake="$2" saved="$SP_RUNTIME_DIR/recreate-$1" entries target token journal
  [[ "$container" =~ ^[a-zA-Z0-9_.-]+$ && "$handshake" == /* ]] || return 1
  entries="$(cat "$SP_PHASE_REGISTRY")" || return 1
  while IFS='|' read -r target token; do
    [[ "$target" == "$container" ]] || continue
    sp_cancel_one "$target" "$token" && sp_stop_client "$token" || return 1
  done <<<"$entries"
  [[ ! -e "$saved/ready" || -e "$saved/restored" ]] || { echo "unrestored harness snapshot: $saved" >&2; return 1; }
  mkdir -p "$saved" || return 1
  rm -f "$saved/handshake.tar" "$saved/journal.tar" || return 1
  chmod 700 "$saved" || return 1
  rm -f "$saved/ready" "$saved/restored"
  # mkdir makes absence explicit, so a failed docker cp can never mean an
  # optional/missing journal. Docker exec remains usable during the DNS outage.
  journal="$(docker exec "$container" sh -c 'printf "%s" "${CONSENSUS_RECOVERY_DIR:-/tmp/fhevm-consensus-abort-recovery}"')" || return 1
  [[ "$journal" == /* && "$journal" != *$'\n'* ]] || return 1
  printf '%s\n' "$journal" > "$saved/journal-path" || return 1
  printf '%s|recreate_%s_%s\n' "$container" "$BASHPID" "$RANDOM" >> "$SP_PHASE_REGISTRY" || return 1
  docker exec "$container" mkdir -p "$handshake" "$journal" || return 1
  docker stop -t 2 "$container" >/dev/null || return 1
  [[ "$(docker inspect -f '{{.State.Running}} {{.State.Pid}}' "$container")" == 'false 0' ]] || return 1
  printf '%s\n' "$handshake" > "$saved/handshake-path" || return 1
  # Keep uid/gid/mode in the archive without requiring host chown privileges.
  # Copying through extracted host files would restore root-owned mode600
  # journals, unreadable by the configured non-root E2E container user.
  (umask 177; docker cp "$container:$handshake/." - > "$saved/handshake.tar") || return 1
  (umask 177; docker cp "$container:$journal/." - > "$saved/journal.tar") || return 1
  touch "$saved/ready"
}

sp_restore_recreated() {
  local container="$1" saved="$SP_RUNTIME_DIR/recreate-$1" handshake journal
  [[ -f "$saved/ready" ]] || return 0
  [[ ! -f "$saved/restored" ]] || return 0
  handshake="$(cat "$saved/handshake-path")" || return 1
  journal="$(cat "$saved/journal-path")" || return 1
  [[ "$handshake" == /* && "$journal" == /* ]] || return 1
  docker exec "$container" mkdir -p "$handshake" "$journal" || return 1
  docker cp -a - "$container:$handshake" < "$saved/handshake.tar" || return 1
  docker cp -a - "$container:$journal" < "$saved/journal.tar" || return 1
  # Copying directory contents preserves each entry's owner/mode; the existing
  # destination directory was created by Config.User and needs its private mode.
  docker exec "$container" chmod 700 "$handshake" "$journal" || return 1
  touch "$saved/restored"
}
