# shellcheck shell=bash
# Bound host-side Docker/systemd transport too. A case deadline must interrupt
# the runner even before its first in-container phase has been launched.
[[ "${HC_HELPERS_LOADED:-0}" == 1 ]] && return 0
readonly HC_HELPERS_LOADED=1

hc_stop_timer() {
  if [[ -n "${HC_TIMER_PID:-}" ]]; then
    kill "$HC_TIMER_PID" 2>/dev/null || true
    wait "$HC_TIMER_PID" 2>/dev/null || true
    HC_TIMER_PID=""
  fi
}
hc_arm_deadline() {
  hc_stop_timer
  unset HC_CLEANUP_DEADLINE_EPOCH
  local owner="$$" deadline="${CASE_DEADLINE_EPOCH:?}"
  command -v node >/dev/null || return 1
  node -e 'const fs=require("fs");const [pid,deadline]=process.argv.slice(1);const file=`/proc/${pid}/stat`;const identity=()=>{try{return fs.readFileSync(file,"utf8").split(") ")[1].split(" ")[19]}catch{return null}};const before=identity();setInterval(()=>{if(identity()!==before)process.exit(0)},1000).unref();setTimeout(()=>{if(before && identity()===before)process.kill(Number(pid),"SIGTERM")},Math.max(0,Number(deadline)*1000-Date.now()))' "$owner" "$deadline" </dev/null >/dev/null 2>&1 &
  HC_TIMER_PID=$!
}
# EXIT recovery is bounded by hc_run. A second INT/TERM must not interrupt
# restoration. Use caught no-op handlers, not SIG_IGN inherited by subprocesses.
hc_cleanup_signals() {
  trap - EXIT
  trap ':' INT TERM
}
hc_begin_cleanup() {
  hc_stop_timer
  if [[ -z "${HC_CLEANUP_DEADLINE_EPOCH:-}" ]]; then
    local budget="${HC_CLEANUP_TIMEOUT_SECONDS:-600}"
    [[ "$budget" =~ ^[1-9][0-9]*$ ]] || return 2
    HC_CLEANUP_DEADLINE_EPOCH=$(( $(date +%s) + budget ))
    export HC_CLEANUP_DEADLINE_EPOCH
  fi
}
hc_time_left() {
  local deadline="${HC_CLEANUP_DEADLINE_EPOCH:-${CASE_DEADLINE_EPOCH:-}}"
  [[ -z "$deadline" ]] || (( $(date +%s) < deadline ))
}
hc_run() {
  local cap="${HC_COMMAND_TIMEOUT_SECONDS:-120}" deadline remaining status=0
  [[ "$cap" =~ ^[1-9][0-9]*$ ]] || return 2
  deadline="${HC_CLEANUP_DEADLINE_EPOCH:-${CASE_DEADLINE_EPOCH:-}}"
  if [[ -n "$deadline" ]]; then
    remaining=$((deadline - $(date +%s)))
    (( remaining > 0 )) || return 124
    (( remaining >= cap )) || cap="$remaining"
  fi
  timeout --kill-after=2s "${cap}s" "$@" || status=$?
  return "$status"
}
# Explicitly timed commands (`timeout ... docker`) still execute the binary
# directly. Ordinary calls and helpers sourced into these runners use the cap.
docker() { hc_run docker "$@"; }
systemctl() { hc_run systemctl "$@"; }
journalctl() { hc_run journalctl "$@"; }
