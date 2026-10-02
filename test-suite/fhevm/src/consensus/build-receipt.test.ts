import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, writeFileSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { loadInventory } from "./inventory";
import { receiptArtifacts } from "./build-provenance";
const cli = path.resolve(import.meta.dir, "../..");
test("actual CI receipt requires a cold clean build and binds successful immutable state", () => {
  const dir = mkdtempSync(path.join(tmpdir(), "build-receipt-"));
  try {
    const bin = path.join(dir, "bin"), state = path.join(dir, "state"), receipt = path.join(dir, "receipt.json");
    mkdirSync(bin); mkdirSync(path.join(state, "state"), { recursive: true });
    writeFileSync(path.join(bin, "git"), '#!/bin/bash\ncase "$3" in rev-parse) printf "%s\\n" "${TEST_REVISION}";; status) printf "%s" "${TEST_DIRTY:-}";; *) exit 91;; esac\n', { mode: 0o755 });
    const env = { ...process.env, PATH: `${bin}:${process.env.PATH}`, FHEVM_STATE_DIR: state, TEST_REVISION: "a".repeat(40) };
    const call = (command: string, suffix = "", extra = {}) => Bun.spawnSync([process.execPath, "scripts/checkout-build-receipt.ts", command, receipt, suffix], { cwd: cli, env: { ...env, ...extra } });
    expect(call("begin", "checkout", { TEST_DIRTY: " M source.ts" }).exitCode).not.toBe(0);
    expect(call("begin", "checkout").exitCode).toBe(0);
    const before = JSON.parse(readFileSync(receipt, "utf8"));
    const images = [{ ref: "suite:local", id: `sha256:${"1".repeat(64)}`, group: "test-suite" }, { ref: "worker:local", id: `sha256:${"2".repeat(64)}`, group: "coprocessor" }];
    const stateFile = path.join(state, "state/state.json");
    writeFileSync(stateFile, JSON.stringify({ updatedAt: before.startedAt, builtImages: images }));
    expect(call("begin", "checkout").exitCode).not.toBe(0);
    expect(call("finish", "", { TEST_REVISION: "b".repeat(40) }).exitCode).not.toBe(0);
    expect(call("finish").exitCode).toBe(0);
    expect(JSON.parse(readFileSync(receipt, "utf8")).images).toEqual(images);
    expect(JSON.parse(readFileSync(receipt, "utf8")).features).toBe("none");
    const identity = path.join(dir, "identities.env");
    writeFileSync(identity, `image_fhevm-test-suite-e2e-debug=${images[0].id} (suite:local)\nimage_coprocessor-tfhe-worker=${images[1].id} (worker:local)\n`);
    expect(call("attach", identity).exitCode).toBe(0);
    writeFileSync(identity, `image_fhevm-test-suite-e2e-debug=sha256:${"3".repeat(64)} (suite:local)\nimage_coprocessor-tfhe-worker=${images[1].id} (worker:local)\n`);
    expect(call("attach", identity).exitCode).not.toBe(0);
    writeFileSync(stateFile, JSON.stringify({ updatedAt: "2000-01-01T00:00:00Z", builtImages: images }));
    expect(call("finish").exitCode).not.toBe(0);
  } finally { rmSync(dir, { recursive: true, force: true }); }
});

test("actual record command refuses a stale running image before publishing PASS", () => {
  const dir = mkdtempSync(path.join(tmpdir(), "record-image-"));
  try {
    const bin = path.join(dir, "bin"); mkdirSync(bin);
    writeFileSync(path.join(bin, "git"), '#!/bin/bash\ncase "$3" in rev-parse) printf "%s\\n" "$TEST_REVISION";; status) :;; *) exit 91;; esac\n', { mode: 0o755 });
    writeFileSync(path.join(bin, "docker"), `#!/bin/bash
if [[ "$1" == ps ]]; then printf '%s\\n' fhevm-test-suite-e2e-debug coprocessor-tfhe-worker
elif [[ "$1" == inspect ]]; then
  name="\${!#}"
  if [[ "$3" == '{{.Config.Image}}' ]]; then
    [[ "$name" == fhevm-test-suite-e2e-debug ]] && echo suite:local || echo worker:local
  elif [[ "$3" == '{{.Image}}' ]]; then
    [[ "$name" == fhevm-test-suite-e2e-debug ]] && echo "$TEST_SUITE_IMAGE" || echo "$TEST_WORKER_IMAGE"
  else exit 91; fi
elif [[ "$1" == image ]]; then :
else exit 91; fi
`, { mode: 0o755 });
    const revision = "a".repeat(40), suite = `sha256:${"1".repeat(64)}`, worker = `sha256:${"2".repeat(64)}`;
    const observed = { "image_fhevm-test-suite-e2e-debug": `${suite} (suite:local)`, "image_coprocessor-tfhe-worker": `${worker} (worker:local)` };
    const identities = { ...observed, ...receiptArtifacts({ revision, mode: "checkout", features: "none", startedAt: "2026-09-13T00:00:00Z", completedAt: "2026-09-13T00:01:00Z",
      images: [{ ref: "suite:local", id: suite, group: "test-suite" }, { ref: "worker:local", id: worker, group: "coprocessor" }] }, observed) };
    const file = path.join(dir, "identity.env");
    writeFileSync(file, Object.entries(identities).map(([k,v]) => `${k}=${v}\n`).join(""));
    const entry = loadInventory().cases.find((c) => c.id === "MAT-01-BOUNDARY-FANOUT")!;
    const args = [process.execPath, "scripts/consensus-inventory.ts", "record", "--run", "current-image", "--case", entry.id,
      "--state", "PASS", "--revision", revision, "--backend-class", "cpu", "--hardware-class", "fixture", "--cleanup", "ok", "--results-dir", dir,
      ...[...new Set(entry.assertions.map((a) => a.split(":")[0]))].flatMap((a) => ["--assert", `${a}=pass`])];
    const env = { ...process.env, PATH: `${bin}:${process.env.PATH}`, FHEVM_STATE_DIR: path.join(dir, "state"),
      CONSENSUS_ARTIFACT_IDENTITIES_FILE: file, CONSENSUS_BUILD_MODE: "checkout", TEST_REVISION: revision, TEST_SUITE_IMAGE: suite, TEST_WORKER_IMAGE: worker };
    const stale = Bun.spawnSync(args, { cwd: cli, env: { ...env, TEST_WORKER_IMAGE: `sha256:${"3".repeat(64)}` } });
    expect(stale.exitCode).not.toBe(0); expect(stale.stderr.toString()).toContain("running image differs");
    expect(() => readFileSync(path.join(dir, "current-image.jsonl"))).toThrow();
    const current = Bun.spawnSync(args, { cwd: cli, env });
    expect(current.exitCode, current.stderr.toString()).toBe(0);
    expect(JSON.parse(readFileSync(path.join(dir, "current-image.jsonl"), "utf8")).artifactIdentities["image_coprocessor-tfhe-worker"]).toBe(observed["image_coprocessor-tfhe-worker"]);
  } finally { rmSync(dir, { recursive: true, force: true }); }
});
