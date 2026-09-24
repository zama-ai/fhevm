import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

const scripts = path.resolve(import.meta.dir, "../scripts");
const launcher = readFileSync(path.join(scripts, "gpu-consensus-workers.sh"), "utf8");
const definitions = launcher.slice(0, launcher.lastIndexOf('case "${1:-}" in'))
  .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${scripts}'`);

for (const override of ["", "true", "false"]) {
  test(`GPU unit environment and scheduling evidence agree across restart (override=${override || "unset"})`, () => {
    const dir = mkdtempSync(path.join(tmpdir(), "gpu-unit-env-"));
    try {
      const runtime = path.join(dir, "runtime");
      mkdirSync(path.join(runtime, "env"), { recursive: true });
      const source = path.join(runtime, "env/coprocessor.env");
      // Opposite values exercise both override directions; duplicate entries
      // and a missing trailing newline exercise the actual env-file handoff.
      const inherited = override === "true" ? "false" : "true";
      writeFileSync(source, `DATABASE_URL=postgres://fixture/coprocessor\nCUDA_VISIBLE_DEVICES=99\nFHEVM_GPU_STREAMS_PER_DEVICE=99\nFHEVM_DCID_ADAPTIVE_BATCH_EXECUTION=false\nFHEVM_DCID_ADAPTIVE_BATCH_EXECUTION=${inherited}\nFHEVM_DCID_BATCH_EXECUTION=${inherited}`);
      const harness = path.join(dir, "probe.sh");
      writeFileSync(harness, definitions + `
binary_sha() { echo fixture; }
stop_transient_unit() { :; }
# Capture the file supplied to the actual launch call. No Docker, database or
# real worker is reachable from this fixture.
systemd-run() {
  local arg file=''
  for arg in "$@"; do
    case "$arg" in
      --property=EnvironmentFile=*) file="\${arg#--property=EnvironmentFile=}" ;;
      --setenv=*) return 99 ;;
    esac
  done
  [[ -n "$file" ]] || return 98
  ( source "$file"; printf '%s %s %s %s\\n' "$CUDA_VISIBLE_DEVICES" "$FHEVM_GPU_STREAMS_PER_DEVICE" "$FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION" "$FHEVM_DCID_BATCH_EXECUTION" ) >> "$FHEVM_STATE_DIR/executed"
}
host_env="$GPU_RUNTIME_DIR/coprocessor.0.env"
mkdir -p "$GPU_RUNTIME_DIR"
cp "$ENV_DIR/coprocessor.env" "$host_env"
record_unit_tuning tfhe 0
load_unit_tuning tfhe 0
start_unit tfhe 0 "$host_env"
write_node_config
scheduling_classes > "$FHEVM_STATE_DIR/reported-before"
# Reporting and restoration must not re-read ambient overrides or the mutable
# generated environment after the operator's host environment was captured.
printf 'FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION=garbage\\n' > "$ENV_DIR/coprocessor.env"
export GPU_CONSENSUS_ADAPTIVE_BATCH_EXECUTION_0=garbage
export GPU_CONSENSUS_BATCH_EXECUTION_0=garbage
load_unit_tuning tfhe 0
start_unit tfhe 0 "$host_env"
scheduling_classes > "$FHEVM_STATE_DIR/reported-after"
# Another role must not overwrite the TFHE unit's resolved environment.
write_unit_environment sns 0 "$host_env"
scheduling_classes > "$FHEVM_STATE_DIR/reported-other-role"
rm "$(unit_environment_file tfhe 0)"
if scheduling_classes > /dev/null; then exit 97; fi
`);
      const result = Bun.spawnSync(["bash", harness], {
        env: { ...process.env, FHEVM_STATE_DIR: dir,
          GPU_CONSENSUS_DEVICE_0: "0", GPU_CONSENSUS_STREAMS_PER_DEVICE_0: "16",
          GPU_CONSENSUS_ADAPTIVE_BATCH_EXECUTION_0: override, GPU_CONSENSUS_BATCH_EXECUTION_0: override },
        timeout: 10_000,
      });
      expect(result.exitCode, result.stderr.toString()).toBe(0);
      const expected = override || inherited;
      expect(readFileSync(path.join(dir, "executed"), "utf8")).toBe(`0 16 ${expected} ${expected}\n`.repeat(2));
      const before = readFileSync(path.join(dir, "reported-before"), "utf8");
      expect(before).toContain(`adaptive:${expected},batch:${expected}`);
      expect(readFileSync(path.join(dir, "reported-after"), "utf8")).toBe(before);
      expect(readFileSync(path.join(dir, "reported-other-role"), "utf8")).toBe(before);
      expect(statSync(path.join(runtime, "gpu-consensus-workers/invocations/fhevm-gpu-consensus-sns-0.env")).mode & 0o777).toBe(0o600);
    } finally { rmSync(dir, { recursive: true, force: true }); }
  });
}
