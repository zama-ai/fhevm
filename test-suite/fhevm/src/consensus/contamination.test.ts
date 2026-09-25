import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

test("all live suite entry points refuse an unverified test-process shutdown", () => {
  const directory = mkdtempSync(path.join(tmpdir(), "consensus-contamination-"));
  const root = path.resolve(import.meta.dir, "../../../..");
  const marker = path.join(directory, "runtime/failure-matrix/uncancelled-phase");
  const env = { ...process.env, FHEVM_STATE_DIR: directory, REPO_ROOT: root };
  const suite = () => Bun.spawnSync(["bash", "-c", `
    source "$REPO_ROOT/test-suite/fhevm/scripts/lib/suite-identity.sh"
    suite_identity_host() { echo same; }
    suite_identity_container() { echo same; }
    suite_identity_assert test-container
  `], {env});
  try {
    mkdirSync(path.dirname(marker), {recursive: true});
    writeFileSync(marker, "phase_registry=retained-for-recovery\n");
    expect(suite().exitCode).toBe(1);
    for (const gate of ["locks", "exclusivity"]) {
      const result = Bun.spawnSync(["bash", path.join(root, "test-suite/fhevm/scripts/consensus-validity.sh"), gate], {env});
      expect(result.exitCode).toBe(3);
      expect(result.stderr.toString()).toContain("test process could not be stopped");
    }
    rmSync(marker);
    expect(suite().exitCode).toBe(0);
  } finally { rmSync(directory, {recursive: true, force: true}); }
});

for (const copy of ["same", "changed", "missing", "unavailable"] as const) {
  test(`exported suite identity functions reject stale copies and preserve source scope: ${copy}`, () => {
    const directory = mkdtempSync(path.join(tmpdir(), "consensus-exported-identity-"));
    const root = path.resolve(import.meta.dir, "../../../..");
    const host = path.join(directory, "host");
    const container = path.join(directory, "container");
    const source = path.join(host, "test-suite/e2e");
    try {
      for (const base of [source, container]) {
        mkdirSync(path.join(base, "test/node_modules"), { recursive: true });
        mkdirSync(path.join(base, "contracts"));
        writeFileSync(path.join(base, "test/example.ts"), "export const value = 1;\n");
        writeFileSync(path.join(base, "contracts/Fixture.sol"), "contract Fixture {}\n");
        // Neither files outside the source roots nor dependencies belong to the digest.
        writeFileSync(path.join(base, "hardhat.config.ts"), base);
        writeFileSync(path.join(base, "test/node_modules/ignored.ts"), base);
      }
      if (copy === "changed") writeFileSync(path.join(container, "test/example.ts"), "export const value = 2;\n");
      if (copy === "missing") rmSync(path.join(container, "test/example.ts"));
      const result = Bun.spawnSync(["bash", "-c", `
        set -uo pipefail
        source "$IDENTITY_HELPER"
        docker() {
          [[ "$1" == exec && "$2" == test-container && "$3" == sh && "$4" == -c ]] || return 99
          [[ "$COPY_STATE" != unavailable ]] || return 1
          local script="$5"
          [[ "$script" == 'cd /app/test-suite/e2e && '* ]] || return 99
          (cd "$CONTAINER_COPY" && sh -c "\${script#cd /app/test-suite/e2e && }")
        }
        # This is the real matrix boundary: functions and scalar configuration
        # survive, but a file-level Bash array cannot be exported to the child.
        while read -r _ _ name; do export -f "\${name?}"; done < <(declare -F)
        bash -c 'set -uo pipefail; suite_identity_assert test-container'
      `], { env: {
        ...process.env,
        IDENTITY_HELPER: path.join(root, "test-suite/fhevm/scripts/lib/suite-identity.sh"),
        REPO_ROOT: host,
        FHEVM_STATE_DIR: path.join(directory, "state"),
        CONTAINER_COPY: container,
        COPY_STATE: copy,
      } });
      expect(result.exitCode).toBe(copy === "same" ? 0 : 1);
      if (copy === "same") expect(result.stdout.toString()).toContain("runs this working tree's e2e suite");
      else if (copy === "unavailable") expect(result.stderr.toString()).toContain("cannot read the e2e suite inside");
      else {
        expect(result.stderr.toString()).toContain("DIFFERENT e2e suite");
        expect(result.stderr.toString()).toContain("    test/example.ts");
        expect(result.stderr.toString()).not.toContain("hardhat.config.ts");
        expect(result.stderr.toString()).not.toContain("ignored.ts");
      }
    } finally { rmSync(directory, { recursive: true, force: true }); }
  });
}
