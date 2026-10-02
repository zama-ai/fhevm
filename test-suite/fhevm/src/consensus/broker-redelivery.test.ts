import { expect, test } from "bun:test";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

const runner = readFileSync(new URL("../../scripts/run-broker-redelivery.sh", import.meta.url), "utf8");
const acknowledgement = runner.match(/^ack\(\) \{[\s\S]*?^\}/m)?.[0];
if (!acknowledgement) throw new Error("broker acknowledgement function is missing");

for (const blocked of [false, true]) {
  test(`broker acknowledgement initializes a cold handshake directory and propagates failure (${blocked})`, () => {
    const directory = mkdtempSync(path.join(tmpdir(), "broker-handshake-"));
    const parent = path.join(directory, "absent parent");
    const handshakes = path.join(parent, "handshakes");
    try {
      if (blocked) writeFileSync(parent, "not a directory");
      // Execute only the real acknowledgement function. The Docker replacement
      // runs its remote shell locally; no live container or database is reachable.
      const result = Bun.spawnSync(["bash", "-c", `set -euo pipefail
docker() {
  [[ "$1" == exec && "$2" == -i && "$3" == fixture ]] || return 97
  shift 3
  "$@"
}
${acknowledgement}
ack
`], {
        env: {
          ...process.env, TEST_CONTAINER: "fixture", HANDSHAKE_DIR: handshakes,
          CASE_ID: "FM-BROKER-REDELIVERY", FAULT_AT: "2026-01-01T00:00:00Z",
        },
      });
      const receipt = path.join(handshakes, "failure-fault.json");
      if (blocked) {
        expect(result.exitCode).not.toBe(0);
        expect(existsSync(receipt)).toBe(false);
      } else {
        expect(result.stderr.toString()).toBe("");
        expect(result.exitCode).toBe(0);
        expect(JSON.parse(readFileSync(receipt, "utf8"))).toMatchObject({
          name: "failure-fault", ready: true,
          payload: { caseId: "FM-BROKER-REDELIVERY", applied: true, faultObservedAt: "2026-01-01T00:00:00Z" },
        });
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
}
