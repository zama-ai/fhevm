import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

for (const mismatch of [false, true]) test(`topology probe publishes only after live Gateway verification (mismatch=${mismatch})`, () => {
  const dir = mkdtempSync(path.join(tmpdir(), "gateway-topology-"));
  try {
    mkdirSync(path.join(dir, "state"));
    writeFileSync(path.join(dir, "state/state.json"), JSON.stringify({
      scenario: { kind: "coprocessor-consensus", sourcePath: "/scenarios/two-of-three.yaml", topology: { count: 3, threshold: 2 }, hostChains: [{ key: "host", chainId: "12345" }] },
      discovery: { gateway: { GATEWAY_CONFIG_ADDRESS: `0x${"11".repeat(20)}` }, endpoints: { gateway: { http: "http://gateway-node:8546" } } },
    }));
    writeFileSync(path.join(dir, "docker"), `#!/bin/bash
if [[ "$1" == inspect ]]; then
  url=http://host-node:8545
  [[ "$2" != *consumer ]] || url=redis://listener-redis:6379/0
  printf '[{"Config":{"Cmd":["--url=%s"]},"State":{"Running":true}}]' "$url"
elif [[ "$1" == exec ]]; then
  [[ " $* " == *'assertGatewayTopology'* ]] || exit 90
  [[ " $* " == *',3,2]'* ]] || exit 91
  ${mismatch ? 'echo "observed threshold differs" >&2; exit 1' : 'exit 0'}
else exit 92
fi
`, { mode: 0o755 });
    const run = Bun.spawnSync([process.execPath, path.resolve(import.meta.dir, "../../scripts/observe-consensus-topology.ts"), "3"], {
      env: { ...process.env, FHEVM_STATE_DIR: dir, PATH: `${dir}:${process.env.PATH}`, CONSENSUS_SCENARIO: "two-of-three", CONSENSUS_THRESHOLD: "2" }, timeout: 5_000,
    });
    expect(run.exitCode).toBe(mismatch ? 1 : 0);
    if (mismatch) { expect(run.stdout.toString()).toBe(""); expect(run.stderr.toString()).toContain("observed threshold differs"); }
    else expect(run.stdout.toString()).toContain("CONSENSUS_THRESHOLD=2");
  } finally { rmSync(dir, { recursive: true, force: true }); }
});
