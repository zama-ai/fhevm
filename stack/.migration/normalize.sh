#!/usr/bin/env bash
# normalize.sh — canonicalize a helm-template output stream for L0 golden comparison.
#
# USAGE
#   helm template ... | bash normalize.sh
#   normalize.sh < raw-helm-output.yaml
#
# ALGORITHM
#   1. Strip volatile lines that change between renders but carry no semantic
#      content: helm.sh/chart annotations, app.kubernetes.io/version labels,
#      checksum/* annotations, uid: fields, creationTimestamp: nulls,
#      and helm symlink-resolution INFO messages emitted on stderr.
#   2. Normalize blank lines: collapse runs of blank lines to a single blank.
#   3. Sort all remaining lines lexicographically so that insertion-order
#      differences in maps don't cause false diff positives.
#
# STABLE CONTRACT
#   This script is committed alongside the goldens.  Any change to the
#   stripping rules REQUIRES re-recording all goldens (they are keyed by
#   sha256 of the normalized stream).  The rules below are the canonical
#   filter set; do not add new rules without updating the golden sha256s.
#
# VOLATILE PATTERNS STRIPPED
#   - Lines matching:  helm.sh/chart:
#   - Lines matching:  app.kubernetes.io/version:
#   - Lines matching:  checksum/
#   - Lines matching:  uid:
#   - Lines matching:  creationTimestamp:
#   - Lines matching:  ^level=INFO  (helm log messages from symlink resolution)
#   - Lines matching:  ^time=        (helm log timestamps)
#   - Lines matching:  ^#           (helm source comments — "# Source: ...")
#                      NOTE: these are stripped to make multi-release renders
#                      comparable regardless of release name.

set -euo pipefail

grep -v \
    -e 'helm\.sh/chart:' \
    -e 'app\.kubernetes\.io/version:' \
    -e 'checksum/' \
    -e '^\s*uid:' \
    -e '^\s*creationTimestamp:' \
    -e '^level=INFO' \
    -e '^time=' \
    -e '^# Source:' \
| grep -v '^---$' \
| sort
