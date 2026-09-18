# shellcheck shell=bash
# One absolute deadline across arming, fault observation and recovery.
case_deadline_start() {
  local budget="$1"
  [[ "$budget" =~ ^[1-9][0-9]*$ ]] || return 2
  unset HC_CLEANUP_DEADLINE_EPOCH
  CASE_DEADLINE_EPOCH=$(( $(date +%s) + budget ))
  export CASE_DEADLINE_EPOCH
}

case_seconds_left() {
  local remaining=$(( ${CASE_DEADLINE_EPOCH:?case deadline not set} - $(date +%s) ))
  (( remaining > 0 )) || return 124
  printf '%s\n' "$remaining"
}

case_phase_deadline_ms() {
  local cap="$1" remaining
  remaining="$(case_seconds_left)" || return $?
  (( cap < remaining )) && remaining="$cap"
  printf '%s\n' "$(( ($(date +%s) + remaining) * 1000 ))"
}
