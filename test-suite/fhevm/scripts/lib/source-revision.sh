# shellcheck shell=bash
# One definition of a source revision for builds and consensus evidence.
# Provisioning rewrites this exact generated address file. Its actual contents
# are checked separately by suite-identity; every other source change counts.
SR_GENERATED_SOURCE=test-suite/e2e/contracts/E2ECoprocessorConfigLocal.sol

sr_status() {
  git -C "${1:?repository root required}" status --porcelain --untracked-files=all -- \
    . ":(top,exclude,literal)$SR_GENERATED_SOURCE"
}

sr_revision() {
  local root="${1:?repository root required}" revision dirty
  revision="$(git -C "$root" rev-parse HEAD)" || return 1
  dirty="$(sr_status "$root")" || return 1
  [[ -z "$dirty" ]] || revision="${revision}-dirty"
  printf '%s\n' "$revision"
}
