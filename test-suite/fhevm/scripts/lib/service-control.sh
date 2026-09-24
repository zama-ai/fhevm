# shellcheck shell=bash
# One service-control API for both execution backends, and it fails closed.
#
# Sourced, not executed. Callers must have SCRIPT_DIR and REPO_ROOT set.
#
# Three runners used to carry their own copies of "stop this operator's worker",
# and the copies had drifted: one called a helper that did not exist
# (`unit_main_pid` rather than `gpu_unit_main_pid`), so GPU stall injection was a
# silent no-op and the cells reported PASS against a worker that had never been
# interrupted. That is the failure mode this file exists to remove, so every
# operation here:
#
#   * resolves the target to whatever is ACTUALLY serving the queue -- a compose
#     container, or a GPU host unit whose container the swap deliberately
#     stopped;
#   * refuses a target that is missing, already stopped, or not faultable,
#     rather than injecting into nothing;
#   * verifies an independent postcondition after acting -- a stopped process is
#     confirmed stopped, a killed process is confirmed replaced -- because a
#     successful signal command proves only that the signal was sent;
#   * returns non-zero on every failure, so `if ! sc_pause ...` is meaningful
#     even inside a command substitution where `set -e` does not apply.
#
# Nothing here interprets a result. A caller decides whether an interrupted
# worker is a fault or the point of the test.

# shellcheck source=gpu-session.sh
source "${SCRIPT_DIR}/lib/gpu-session.sh"
source "${SCRIPT_DIR}/lib/host-command.sh"

SC_DEFAULT_TIMEOUT="${SC_DEFAULT_TIMEOUT:-30}"

sc_log() { printf 'service-control: %s\n' "$*" >&2; }
sc_error() { printf 'service-control: ERROR %s\n' "$*" >&2; }

sc_init() {
  gpu_normalise_user_bus
  SC_RESTORE_LOG_OWNED=0
  [[ -n "${SC_RESTORE_LOG:-}" ]] || SC_RESTORE_LOG_OWNED=1
  SC_RESTORE_LOG="${SC_RESTORE_LOG:-$(mktemp)}" || return 1
  [[ ! -e "$SC_RESTORE_LOG" || -f "$SC_RESTORE_LOG" ]] || return 1
  touch "$SC_RESTORE_LOG" || return 1
  SC_CLEANUP_FAILED=0
}

# ---------------------------------------------------------------------------
# Target resolution
# ---------------------------------------------------------------------------

# `unit` when a GPU host unit serves this container's queue, else `container`.
#
# Keyed on the GPU session marker, NOT on a unit's current ActiveState: a case
# that stops a unit and then heals it would otherwise find the unit inactive,
# fall back to Docker, and start a CPU worker beside a dead CUDA unit -- which
# is how one queue ends up served by two different builds.
sc_kind() {
  local target="$1"
  if [[ -n "$(gpu_unit_for_container "$target")" ]]; then echo unit; else echo container; fi
}

sc_unit() { gpu_unit_for_container "$1"; }

# ---------------------------------------------------------------------------
# Inspection
# ---------------------------------------------------------------------------

# One vocabulary for both backends: running | stopped | paused | missing.
#
# A unit has no paused state of its own -- SIGSTOP leaves it `active` -- so the
# process state is read from /proc, which is also the only honest answer for a
# container whose main process was stopped from outside Docker.
sc_state() {
  local target="$1" kind unit pid status
  kind="$(sc_kind "$target")"
  if [[ "$kind" == unit ]]; then
    unit="$(sc_unit "$target")"
    status="$(systemctl --user show "$unit" --property=ActiveState --value 2>/dev/null)" || status=""
    case "$status" in
      active) ;;
      "") echo missing; return 0 ;;
      *) echo stopped; return 0 ;;
    esac
    pid="$(gpu_unit_main_pid "$unit")"
    if [[ -z "$pid" || "$pid" == 0 ]]; then echo stopped; return 0; fi
    if [[ "$(sc_proc_state "$pid")" == T ]]; then echo paused; else echo running; fi
    return 0
  fi
  status="$(docker inspect -f '{{.State.Status}}' "$target" 2>/dev/null)" || { echo missing; return 0; }
  case "$status" in
    running)
      pid="$(docker inspect -f '{{.State.Pid}}' "$target" 2>/dev/null)"
      if [[ -n "$pid" && "$pid" != 0 && "$(sc_proc_state "$pid")" == T ]]; then echo paused; else echo running; fi
      ;;
    paused) echo paused ;;
    "") echo missing ;;
    *) echo stopped ;;
  esac
}

# The kernel's own view: `T` means the process is stopped, whoever stopped it.
# This is the independent postcondition a stall case needs -- `systemctl kill
# --signal=SIGSTOP` reports success whether or not anything received it.
sc_proc_state() {
  local pid="$1" stat
  [[ -n "$pid" && "$pid" != 0 ]] || return 1
  # Field 3 of /proc/<pid>/stat, read past the parenthesised comm which may
  # itself contain spaces.
  stat="$(cat "/proc/$pid/stat" 2>/dev/null)" || return 1
  printf '%s' "${stat#*) }" | cut -d' ' -f1
}

sc_main_pid() {
  local target="$1" kind
  kind="$(sc_kind "$target")"
  if [[ "$kind" == unit ]]; then
    gpu_unit_main_pid "$(sc_unit "$target")"
  else
    docker inspect -f '{{.State.Pid}}' "$target" 2>/dev/null
  fi
}

# What identifies THIS run of the target's process.
#
# Recorded before and after a fault so a case can prove the process was
# replaced rather than merely signalled. A container keeps its id across a
# restart, so the id alone proves nothing; the start time, pid and restart
# count together do.
sc_identity() {
  local target="$1" kind unit
  kind="$(sc_kind "$target")"
  if [[ "$kind" == unit ]]; then
    unit="$(sc_unit "$target")"
    printf 'unit=%s invocation=%s pid=%s restarts=%s' \
      "$unit" \
      "$(systemctl --user show "$unit" --property=InvocationID --value 2>/dev/null)" \
      "$(gpu_unit_main_pid "$unit")" \
      "$(systemctl --user show "$unit" --property=NRestarts --value 2>/dev/null)"
    return 0
  fi
  printf 'container=%s pid=%s started=%s restarts=%s' \
    "$target" \
    "$(docker inspect -f '{{.State.Pid}}' "$target" 2>/dev/null)" \
    "$(docker inspect -f '{{.State.StartedAt}}' "$target" 2>/dev/null)" \
    "$(docker inspect -f '{{.RestartCount}}' "$target" 2>/dev/null)"
}

sc_restart_count() {
  local target="$1" kind
  kind="$(sc_kind "$target")"
  if [[ "$kind" == unit ]]; then
    systemctl --user show "$(sc_unit "$target")" --property=NRestarts --value 2>/dev/null || echo 0
  else
    docker inspect -f '{{.RestartCount}}' "$target" 2>/dev/null || echo 0
  fi
}

# Logs from whatever is actually running, scoped to a window.
#
# The GPU session stops the worker containers on purpose, so reading
# `docker logs` there returns the last CPU worker's output -- historical text
# from a process that is not the one under test. Under a session this reads the
# unit's journal instead, and deliberately not only the current invocation: a
# case that killed the worker is asking about the invocation that died.
sc_logs_since() {
  local target="$1" since="$2" kind unit
  kind="$(sc_kind "$target")"
  if [[ "$kind" == unit ]]; then
    unit="$(sc_unit "$target")"
    journalctl --user -u "$unit" --since "$since" --no-pager -o cat 2>/dev/null
    return "${PIPESTATUS[0]:-0}"
  fi
  docker logs --since "$since" "$target" 2>&1
}

# ---------------------------------------------------------------------------
# Preconditions
# ---------------------------------------------------------------------------

# Refuse a target that cannot carry the fault the caller is about to inject.
#
# `allow_paused` exists for a boundary that has to be FROZEN before the fault
# lands: the caller stops the process, re-verifies the state under the freeze,
# and only then signals, with the signal delivered on resume. No case needs it
# today -- the post-commit boundary is now held open by blocking the release's
# own row lock instead, which keeps the worker running and the state still --
# but the capability stays because the pattern is the honest way to fault a
# state that a running process leaves within milliseconds. Everywhere else a
# paused target means the caller is injecting into something already stopped,
# which proves nothing.
sc_require_faultable() {
  local target="$1" allow_paused="${2:-0}" state
  state="$(sc_state "$target")"
  case "$state" in
    running) return 0 ;;
    paused)
      [[ "$allow_paused" == 1 ]] && return 0
      sc_error "$target is already paused; injecting into it would prove nothing"
      return 1
      ;;
    stopped)
      sc_error "$target is stopped and nothing else serves it; a fault injected here would prove nothing"
      return 1
      ;;
    missing)
      sc_error "$target is absent from this topology"
      return 1
      ;;
  esac
  sc_error "$target is in an unrecognised state ($state)"
  return 1
}

# ---------------------------------------------------------------------------
# Waiting
# ---------------------------------------------------------------------------

# Bounded wait for a target to reach a state. Every wait here has a deadline:
# an unbounded one turns a wedged stack into a hung run rather than a failure.
sc_wait_state() {
  local target="$1" want="$2" timeout="${3:-$SC_DEFAULT_TIMEOUT}" deadline
  deadline=$((SECONDS + timeout))
  while ((SECONDS < deadline)); do
    hc_time_left || return 124
    [[ "$(sc_state "$target")" == "$want" ]] && return 0
    sleep 1
  done
  sc_error "$target did not become $want within ${timeout}s (it is $(sc_state "$target"))"
  return 1
}

# Bounded wait for the target's process to be REPLACED, which is what a crash
# case has to establish. A restart policy that did not fire, or a signal that
# never arrived, both look like an unchanged identity.
sc_wait_replaced() {
  local target="$1" before="$2" timeout="${3:-60}" deadline now confirmed
  deadline=$((SECONDS + timeout))
  while ((SECONDS < deadline)); do
    hc_time_left || return 124
    now="$(sc_identity "$target")"
    if [[ "$now" != "$before" && "$now" =~ [[:space:]]pid=[1-9][0-9]*[[:space:]] && "$(sc_state "$target")" == running ]]; then
      confirmed="$(sc_identity "$target")"
      [[ "$confirmed" == "$now" ]] || continue
      printf '%s' "$now"
      return 0
    fi
    sleep 1
  done
  sc_error "$target still reports the identity it had before the fault ($before) after ${timeout}s"
  return 1
}

# ---------------------------------------------------------------------------
# Restore registry
# ---------------------------------------------------------------------------

# Registered immediately BEFORE the first mutation, so an interrupt between the
# mutation and the trap cannot leave a stack nobody puts back. Restores run in
# reverse order.
sc_register_restore() {
  local target="$1" action="$2"
  [[ -f "$SC_RESTORE_LOG" && "$target" != *'|'* && "$target" != *$'\n'* ]] || return 1
  printf '%s|%s\n' "$target" "$action" >>"$SC_RESTORE_LOG" || {
    sc_error "cannot record restoration for $target; refusing mutation"
    return 1
  }
}

sc_clear_restore() {
  local target="$1" action="$2" tmp status=0
  [[ -f "$SC_RESTORE_LOG" ]] || return 1
  # Same-directory rename is atomic. Only grep's ordinary no-match status is
  # acceptable; an unreadable source must never become an empty ledger.
  tmp="$(mktemp "${SC_RESTORE_LOG}.XXXXXX")" || return 1
  grep -v -x -F "${target}|${action}" "$SC_RESTORE_LOG" >"$tmp" || status=$?
  if ((status > 1)) || ! mv "$tmp" "$SC_RESTORE_LOG"; then
    rm -f "$tmp"
    sc_error "could not update restore ledger; retaining owners"
    return 1
  fi
}

# Runs every outstanding restore and REPORTS what it could not do. A cleanup
# failure is a result, not a footnote: the next case on the same stack inherits
# whatever was left behind, so callers stop rather than continue.
sc_recovery_allowed() {
  if [[ -f "${FHEVM_STATE_DIR:-$REPO_ROOT/.fhevm}/runtime/failure-matrix/uncancelled-phase" ]]; then
    sc_error "test-process shutdown is unverified; refusing to heal faults beneath it"
    return 1
  fi
}

sc_run_restores() {
  sc_recovery_allowed || return 1
  local target action failures=0 entries
  [[ -f "$SC_RESTORE_LOG" ]] || return 1
  entries="$(tac "$SC_RESTORE_LOG")" || return 1
  while IFS='|' read -r target action; do
    [[ -n "$target" ]] || continue
    sc_log "restoring $target ($action)"
    case "$action" in
      resume)
        sc_resume "$target" || failures=$((failures + 1))
        ;;
      start)
        sc_start "$target" || failures=$((failures + 1))
        ;;
      stop-container)
        # Only a deliberately introduced Docker owner is removed. Do not use
        # sc_stop, whose logical target may resolve to the displaced GPU unit.
        if docker stop "$target" >/dev/null 2>&1 &&
           [[ "$(docker inspect -f '{{.State.Running}} {{.State.Pid}}' "$target" 2>/dev/null)" == 'false 0' ]]; then
          sc_clear_restore "$target" stop-container || failures=$((failures + 1))
        else failures=$((failures + 1)); fi
        ;;
      *)
        sc_error "unknown restore action $action for $target"
        failures=$((failures + 1))
        ;;
    esac
  done <<<"$entries"
  # Successful heals deregister themselves. Keep every failed entry for retry.
  if ((failures > 0)); then
    SC_CLEANUP_FAILED=1
    sc_error "$failures restore(s) failed; this stack is contaminated and later cases cannot be trusted"
    return 1
  fi
  return 0
}

# ---------------------------------------------------------------------------
# Faults and heals
# ---------------------------------------------------------------------------

# SIGSTOP the target's main process, then prove it is stopped.
#
# `docker pause` freezes the whole cgroup and is the right tool for a container;
# a unit has no equivalent, so its main process is stopped directly. Either way
# the postcondition is read from the kernel rather than from the command's exit
# status.
sc_pause() {
  local target="$1" kind pid
  sc_require_faultable "$target" || return 1
  kind="$(sc_kind "$target")"
  sc_register_restore "$target" resume || return 1
  if [[ "$kind" == unit ]]; then
    pid="$(sc_main_pid "$target")"
    if [[ -z "$pid" || "$pid" == 0 ]]; then
      sc_error "$(sc_unit "$target") has no main PID, so it cannot be stalled"
      return 1
    fi
    if ! kill -STOP "$pid" 2>/dev/null; then
      sc_error "could not SIGSTOP pid $pid of $(sc_unit "$target")"
      return 1
    fi
  elif ! docker pause "$target" >/dev/null 2>&1; then
    sc_error "docker pause $target failed"
    return 1
  fi
  sc_wait_state "$target" paused 15 || return 1
  # "frozen", not "stopped": a stopped service has exited and released whatever
  # it held, a frozen one still holds it. The crash-retry cases turn on exactly
  # that difference, and a log line calling one the other cost real debugging
  # time.
  sc_log "$target is frozen (verified)"
  return 0
}

sc_resume() {
  sc_recovery_allowed || return 1
  local target="$1" kind pid state
  state="$(sc_state "$target")"
  if [[ "$state" == running ]]; then
    sc_clear_restore "$target" resume || return 1
    return 0
  fi
  kind="$(sc_kind "$target")"
  if [[ "$kind" == unit ]]; then
    pid="$(sc_main_pid "$target")"
    if [[ -z "$pid" || "$pid" == 0 ]]; then
      # A unit that was stalled and then died has nothing to resume; bring it
      # back through the launcher instead, which owns the invocation.
      sc_start "$target" || return 1
      sc_clear_restore "$target" resume || return 1
      return 0
    fi
    kill -CONT "$pid" 2>/dev/null || { sc_error "could not SIGCONT pid $pid of $(sc_unit "$target")"; return 1; }
  else
    if [[ "$state" == paused ]]; then
      docker unpause "$target" >/dev/null 2>&1 || {
        # A container whose main process was SIGSTOPped from the host is
        # `running` to Docker, so `unpause` refuses; signal it instead.
        pid="$(sc_main_pid "$target")"
        [[ -n "$pid" && "$pid" != 0 ]] && kill -CONT "$pid" 2>/dev/null
      }
    fi
  fi
  sc_wait_state "$target" running 30 || return 1
  sc_clear_restore "$target" resume || return 1
  sc_log "$target is running again (verified)"
  return 0
}

# Can this target's supervisor still restart it?
#
# Docker's `restart: on-failure:N` stops restarting after N attempts, and the
# count is cumulative until someone starts the container by hand. A stack that
# has hosted several sessions of crash cases exhausts it: the poller reported
# `restarts=10` and was never replaced, and the case read that as the service
# failing to recover when it was the supervisor having given up. A manual
# stop+start resets the count to zero, which restores the precondition the case
# needs -- a supervised service with restarts available, which is what
# production has.
sc_restart_budget() {
  local target="$1" policy count max
  [[ "$(sc_kind "$target")" == container ]] || { echo ok; return 0; }
  policy="$(docker inspect -f '{{.HostConfig.RestartPolicy.Name}}' "$target" 2>/dev/null)"
  max="$(docker inspect -f '{{.HostConfig.RestartPolicy.MaximumRetryCount}}' "$target" 2>/dev/null)"
  count="$(docker inspect -f '{{.RestartCount}}' "$target" 2>/dev/null)"
  [[ "$policy" == on-failure && "${max:-0}" -gt 0 ]] || { echo ok; return 0; }
  if [[ "${count:-0}" -ge "$max" ]]; then echo exhausted; else echo ok; fi
}

# Give it its budget back, by the only means Docker offers.
sc_reset_restart_budget() {
  local target="$1"
  sc_register_restore "$target" start || return 1
  docker stop "$target" >/dev/null 2>&1 || return 1
  docker start "$target" >/dev/null 2>&1 || return 1
  sc_wait_state "$target" running 60 || return 1
  sc_log "$target restart budget reset (it had used all of its on-failure retries)"
  return 0
}

# Is every operator running the SAME implementation for a given worker kind?
#
# `exclusivity` asks whether each queue has exactly one worker, which a fleet
# split across backends satisfies perfectly: operator 0 on a CUDA unit and
# operator 2 on a CPU container is one worker per queue and two different
# squash implementations. That is B-1 -- identical compute digests and
# different SNS digests, which is exactly what a consensus comparison then
# reports as a divergence in the system.
sc_fleet_homogeneous() {
  local kind="${1:-sns}" count="${2:-3}" index prefix target seen="" impl detail=""
  for ((index = 0; index < count; index++)); do
    [[ "$index" == 0 ]] && prefix=coprocessor || prefix="coprocessor${index}"
    target="${prefix}-${kind}-worker"
    docker inspect "$target" >/dev/null 2>&1 || continue
    impl="$(sc_kind "$target")"
    detail="${detail}${detail:+ }${target}=${impl}"
    [[ -z "$seen" ]] && seen="$impl"
    if [[ "$impl" != "$seen" ]]; then
      sc_error "the fleet is split across implementations for ${kind}: $detail"
      return 1
    fi
  done
  sc_log "every operator runs the same ${kind} implementation ($detail)"
  return 0
}

# Kill the target so its supervisor sees a crash, and prove the process died.
#
# The signal must come from OUTSIDE Docker's API for a container: `docker kill`
# is recorded as a manual intervention and suppresses the restart policy, which
# turns a test of automatic recovery into a test of manual recovery. Signalling
# the container's main process from the host is indistinguishable from the
# process dying on its own, so the policy applies exactly as it would for a real
# crash. `sc_kill` reports which path it used, because that changes what the run
# proves.
#
# KILL cannot be caught and yields a nonzero process exit, allowing the
# on-failure restart policy to replace the victim. Catchable signals may be
# handled or lead to a clean exit, which would not exercise automatic recovery.
# Callers still have to prove the process identity changed after the signal.
#
# Callers wanting a different signal must pass it and say why.
#
# Prints the pre-fault identity on success, for a caller to pass to
# sc_wait_replaced.
sc_kill() {
  local target="$1" signal="${2:-KILL}" allow_paused="${3:-0}" kind unit pid before
  sc_require_faultable "$target" "$allow_paused" || return 1
  before="$(sc_identity "$target")"
  kind="$(sc_kind "$target")"
  pid="$(sc_main_pid "$target")"
  if [[ -z "$pid" || "$pid" == 0 ]]; then
    sc_error "$target has no main PID to signal"
    return 1
  fi

  sc_register_restore "$target" start || return 1
  if [[ "$kind" == unit ]]; then
    # Our own user's unit: no privilege escalation needed.
    if ! kill "-$signal" "$pid" 2>/dev/null; then
      sc_error "could not send SIG$signal to pid $pid of $(sc_unit "$target")"
      return 1
    fi
    SC_KILL_PATH=host-signal
  elif kill -0 "$pid" 2>/dev/null && kill "-$signal" "$pid" 2>/dev/null; then
    # A stopped process keeps the signal pending until it is resumed, which is
    # what the frozen-boundary case relies on.
    SC_KILL_PATH=host-signal
  elif sudo -n kill "-$signal" "$pid" 2>/dev/null; then
    SC_KILL_PATH=host-signal-sudo
  else
    sc_error "could not signal pid $pid of $target from the host; a Docker-API kill would suppress the restart policy and the run would measure manual recovery instead"
    return 1
  fi
  printf '%s' "$before"
  return 0
}

# Stop the target for as long as the caller wants it gone, and prove it stopped.
sc_stop() {
  local target="$1" kind
  sc_require_faultable "$target" || return 1
  kind="$(sc_kind "$target")"
  sc_register_restore "$target" start || return 1
  if [[ "$kind" == unit ]]; then
    systemctl --user stop "$(sc_unit "$target")" >/dev/null 2>&1 || {
      sc_error "systemctl stop $(sc_unit "$target") failed"
      return 1
    }
  elif ! docker stop "$target" >/dev/null 2>&1; then
    sc_error "docker stop $target failed"
    return 1
  fi
  sc_wait_state "$target" stopped 60 || return 1
  sc_log "$target is stopped (verified)"
  return 0
}

# Bring the target back.
#
# A GPU unit cannot be restarted with `systemctl start`: the units are transient
# (`systemd-run --collect`), so stopping one garbage-collects it and the name
# stops resolving. Only the launcher holds the invocation -- the generated
# environment file, the device, the stream count and the per-node tuning -- so
# restoration goes through it. Starting the container instead would put a CPU
# worker on a queue a CUDA unit owns.
sc_start() {
  sc_recovery_allowed || return 1
  local target="$1" kind unit kindname index
  kind="$(sc_kind "$target")"
  if [[ "$kind" == unit ]]; then
    unit="$(sc_unit "$target")"
    if [[ "$(sc_state "$target")" == running ]]; then
      sc_clear_restore "$target" start || return 1
      return 0
    fi
    kindname="${unit#fhevm-gpu-consensus-}"; kindname="${kindname%-*}"
    index="${unit##*-}"
    if ! hc_run "$SCRIPT_DIR/gpu-consensus-workers.sh" restart-unit "$kindname" "$index" >/dev/null 2>&1; then
      sc_error "gpu-consensus-workers.sh restart-unit $kindname $index failed; $unit is still down"
      return 1
    fi
  else
    docker start "$target" >/dev/null 2>&1 || {
      sc_error "docker start $target failed"
      return 1
    }
  fi
  sc_wait_state "$target" running 90 || return 1
  sc_clear_restore "$target" start || return 1
  sc_log "$target is running (verified)"
  return 0
}

# Restart in place, recording the identity either side so a caller can prove the
# process really was replaced.
sc_restart() {
  local target="$1" before after
  before="$(sc_identity "$target")"
  sc_stop "$target" || return 1
  sc_start "$target" || return 1
  after="$(sc_identity "$target")"
  if [[ "$after" == "$before" ]]; then
    sc_error "$target reports the same process identity after a restart ($after); it did not restart"
    return 1
  fi
  printf '%s' "$after"
  return 0
}

# ---------------------------------------------------------------------------
# Fleet view
# ---------------------------------------------------------------------------

# Container roles that make up one operator, from the generated compose rather
# than a hardcoded list: the six names one runner assumed are not the whole
# operator on every bundle, and an operator declared offline while an
# unlisted ingestion path kept running is not offline.
sc_operator_services() {
  local index="$1" prefix
  [[ "$index" == 0 ]] && prefix="coprocessor" || prefix="coprocessor${index}"
  docker ps -a --format '{{.Names}}' \
    | grep -E "^${prefix}-(host-listener(-(poller|consumer))?(-[a-zA-Z0-9_-]+)?|gw-listener|tfhe-worker|zkproof-worker|sns-worker|transaction-sender|consensus-detector|upgrade-controller)$" \
    | sort
}

# Every coprocessor service across the fleet.
sc_fleet_services() {
  docker ps -a --format '{{.Names}}' \
    | grep -E '^coprocessor[0-9]*-(host-listener(-(poller|consumer))?(-[a-zA-Z0-9_-]+)?|gw-listener|tfhe-worker|zkproof-worker|sns-worker|transaction-sender|consensus-detector|upgrade-controller)$' \
    | sort
}

# Names anything that is not running, judged on whatever serves it. Prints
# nothing when the fleet is intact.
sc_fleet_down() {
  local exclude="${1:-}" name state down=""
  while IFS= read -r name; do
    [[ -n "$name" ]] || continue
    [[ -n "$exclude" && "$name" == "$exclude" ]] && continue
    state="$(sc_state "$name")"
    [[ "$state" == running ]] || down="$down $name($state)"
  done < <(sc_fleet_services)
  printf '%s' "${down# }"
}

# Names anything a supervisor restarted, which is recovery working rather than a
# fault -- but a reader cannot tell the two apart afterwards without the record.
sc_fleet_restarted() {
  local name count out=""
  while IFS= read -r name; do
    [[ -n "$name" ]] || continue
    count="$(sc_restart_count "$name")"
    [[ "${count:-0}" -gt 0 ]] && out="$out $name(x$count)"
  done < <(sc_fleet_services)
  printf '%s' "${out# }"
}


# Snapshot logical service owners, including GPU units whose Docker containers are stopped.
sc_snapshot_running() {
  local names target state
  names="$(docker ps -a --format '{{.Names}}')" || return 1
  while IFS= read -r target; do
    [[ "$target" =~ ^(coprocessor|kms-connector|fhevm|gateway|host|listener) ]] || continue
    [[ "$target" =~ migration|-sc-|setup|deploy|trigger|pausers|add-network|payment|oft ]] && continue
    state="$(sc_state "$target")"
    [[ "$state" != missing ]] || { sc_error "cannot inspect $target for recovery snapshot"; return 1; }
    [[ "$state" == running ]] && printf '%s\n' "$target"
  done <<<"$names"
  return 0
}

# Restore only prior owners. Never sweep exited Docker containers: some are displaced CPU workers.
sc_restore_running() {
  local snapshot="$1" leave_faulted="${2:-0}" automatic_operators="${3:-0}" target state failures=0 retained="" owners
  [[ -f "$snapshot" ]] || return 1
  owners="$(cat "$snapshot")" || return 1
  if [[ "$leave_faulted" == 1 ]]; then retained="$(cat "$SC_RESTORE_LOG")" || return 1; fi
  while IFS= read -r target; do
    [[ -n "$target" ]] || continue
    # Database cases must establish operator recovery before cleanup can repair it.
    if [[ "$automatic_operators" == 1 && "$target" =~ ^coprocessor[0-9]*- ]]; then continue; fi
    if [[ "$leave_faulted" == 1 && $'\n'"$retained"$'\n' == *$'\n'"$target|"* ]]; then continue; fi
    state="$(sc_state "$target")"
    case "$state" in
      running) ;;
      stopped) sc_start "$target" || failures=$((failures + 1)) ;;
      paused) sc_resume "$target" || failures=$((failures + 1)) ;;
      *) sc_error "cannot restore prior owner $target ($state)"; failures=$((failures + 1)) ;;
    esac
  done <<<"$owners"
  [[ "$failures" == 0 ]]
}

# A component name in healthy startup/configuration output is not a fault reaction.
sc_logs_show_failure() {
  local logs="$1" subject="$2"
  grep -iE "$subject" <<<"$logs" \
    | grep -qiE '"level"[[:space:]]*:[[:space:]]*"(error|warn)"|(^|[[:space:]])(ERROR|WARN)([[:space:]]|:)'
}
