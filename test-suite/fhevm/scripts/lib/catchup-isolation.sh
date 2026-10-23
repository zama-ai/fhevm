# Private E2E control for the relative-flags-only replay scenario.
# The marker is shared with the host supervisor so timeout cleanup can remove
# the gate even if the child runner is killed before its own cleanup executes.
install_catchup_isolation() {
  local target="$1" workload="$2" evidence="$REPORT_DIR/FM-HOST-RELATIVE-CATCHUP"
  mkdir -p "$evidence" || return 1
  printf '%s\n' "$workload" > "$evidence/workload.json" || return 1
  jq -e --argjson until "$(($(date +%s) + 2400))" '
    .payload.detail.blockHashes as $hashes |
    if ($hashes | length) != 12 or ($hashes | unique | length) != 12
      or any($hashes[]; test("^0x[0-9a-f]{64}$") | not)
    then error("twelve exact receipt hashes required")
    else {block_hashes:$hashes, until:$until} end' <<<"$workload" > "$evidence/control.json" || return 1
  chmod 644 "$evidence/control.json" || return 1
  # Write ownership before copying into the stopped consumer's filesystem.
  printf '%s\n' "$target" > "$SP_RUNTIME_DIR/catchup-isolation-owner" || return 1
  docker cp "$evidence/control.json" "$target:/tmp/fhevm-test-catchup"
}

verify_catchup_isolation() {
  local target="$1" evidence="$REPORT_DIR/FM-HOST-RELATIVE-CATCHUP"
  docker exec "$target" host_listener_consumer --catchup-test-control receipts > "$evidence/receipts.jsonl" || return 1
  docker exec "$TEST_CONTAINER" cat "$HANDSHAKE_DIR/failure-catchup.json" > "$evidence/request.json" || return 1
  bun "$SCRIPT_DIR/verify-catchup-isolation.ts" "$evidence" | tee "$evidence/verification.json"
  return "${PIPESTATUS[0]}"
}

restore_catchup_isolation() {
  local marker="${SP_RUNTIME_DIR:-}/catchup-isolation-owner" target
  [[ -f "$marker" ]] || return 0
  target="$(cat "$marker")" || return 1
  sc_start "$target" || return 1
  docker exec -u 0 "$target" host_listener_consumer --catchup-test-control clear || return 1
  docker exec "$target" host_listener_consumer --catchup-test-control empty || return 1
  rm -f "$marker"
}
