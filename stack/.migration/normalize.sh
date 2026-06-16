#!/usr/bin/env bash
# normalize.sh — canonicalize helm-template output for L0 golden comparison.
#
# DOCUMENT-AWARE & ROBUST (text-based; does NOT require the output to be strict,
# parseable YAML — helm can emit embedded config that trips a YAML parser).
# Replaces the earlier global line-sort, which could let an intra-document
# value swap pass undetected.
#   1. Split the stream into documents on '---' separators.
#   2. Within each document, strip volatile lines that vary between renders but
#      carry no semantic content (helm.sh/chart, app.kubernetes.io/version,
#      checksum/*, uid:, creationTimestamp:, helm log lines, '# Source:'),
#      while PRESERVING the document's internal line order.
#   3. Sort the LIST of documents (document order is irrelevant) but keep each
#      document's internal order — so a value swap WITHIN a document changes
#      that document and therefore the sha256.
#
# USAGE:  helm template ... | bash normalize.sh
#
# STABLE CONTRACT: committed alongside the goldens. Changing the rules here
# REQUIRES re-recording all goldens (run-l0.sh <case> --record).
set -euo pipefail
awk '
  function emit() { if (cur != "") { docs[n++] = cur } ; cur = "" }
  /^---[[:space:]]*$/ { emit(); next }
  {
    if ($0 ~ /helm\.sh\/chart:/)                next
    if ($0 ~ /app\.kubernetes\.io\/version:/)   next
    if ($0 ~ /checksum\//)                      next
    if ($0 ~ /^[[:space:]]*uid:[[:space:]]/)    next
    if ($0 ~ /^[[:space:]]*creationTimestamp:/) next
    if ($0 ~ /^level=INFO/)                     next
    if ($0 ~ /^time=/)                          next
    if ($0 ~ /^# Source:/)                      next
    cur = (cur == "" ? $0 : cur "\036" $0)   # \036 (RS) joins lines within one doc
  }
  END { emit(); for (i = 0; i < n; i++) print docs[i] }
' | LC_ALL=C sort | tr '\036' '\n'
