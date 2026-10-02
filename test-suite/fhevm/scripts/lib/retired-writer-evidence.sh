#!/usr/bin/env bash
# Release workers render retirement differently; unrelated retry errors are not evidence.
rw_rejection_observed() {
  case "$1" in
    sns-worker)
      grep -Eq 'access denied \(retired stack\)|pausing into no-op mode' <<<"$2"
      ;;
    tfhe-worker)
      grep -Fq 'StaleStackError { binary: \"0.14.0\", live: \"0.15.0\" }' <<<"$2"
      ;;
    *) return 1 ;;
  esac
}
