/**
 * Upgrades testnet's five coprocessor operators from 0.13 to v0.14.5 one at a time, without Blue/Green,
 * while `erc20-traffic` keeps transferring between five wallets and checking every balance. Runs as the
 * `coprocessor-rolling-upgrade` test profile on the `three-of-five` scenario, booted from the
 * `testnet-before-coprocessor-v0.14` profile.
 */
import type { Stack } from "../../src/commands/stack";
import { DRIFT_CONSENSUS_WARNING, DRIFT_WARNING } from "../../src/drift";
import { TEST_SUITE_CONTAINER } from "../../src/layout";
import { toServiceName } from "../../src/utils/fs";
import { run } from "../../src/utils/process";

const TARGET_TAG = "v0.14.5";
const BASELINE_LOCK = "testnet-before-coprocessor-v0.14.json";
const OPERATORS = 5;
const THRESHOLD = 3;
const TRAFFIC_STOP_FILE = "/tmp/coprocessor-rolling-upgrade.stop";
// Locally a transfer and its two decryptions take about a minute, so each mix of versions sees a few.
const SOAK_MS = 3 * 60_000;

const logPhase = (label: string) => console.log(`\n[coprocessor rolling upgrade] ${label}`);

/** Every running service of the operator uses an image with `tag`. */
const assertOperatorRuns = async (index: number, tag: string) => {
  const prefix = toServiceName("", index);
  const rows = (await run(["docker", "ps", "--format", "{{.Names}}|{{.Image}}"])).stdout
    .trim()
    .split("\n")
    .map((row) => row.split("|"))
    .filter(([name, image]) => name!.startsWith(prefix) && image!.includes("/fhevm/coprocessor/"));
  const stale = rows.filter(([, image]) => !image!.endsWith(`:${tag}`));
  if (!rows.length || stale.length) {
    throw new Error(`coprocessor operator ${index} is not fully on ${tag}: ${stale.map((row) => row.join(" ")).join(", ")}`);
  }
};

/** Throws if any gw-listener logged a drift warning since `since`. */
const assertNoDrift = async (since: string) => {
  const drifted: string[] = [];
  for (let index = 0; index < OPERATORS; index += 1) {
    const container = toServiceName("gw-listener", index);
    const logs = await run(["docker", "logs", "--since", since, container]);
    const lines = `${logs.stdout}\n${logs.stderr}`
      .split("\n")
      .filter((line) => line.includes(DRIFT_WARNING) || line.includes(DRIFT_CONSENSUS_WARNING));
    drifted.push(...lines.map((line) => `${container}: ${line}`));
  }
  if (drifted.length) {
    throw new Error(`drift detected during the rolling upgrade:\n${drifted.join("\n")}`);
  }
};

export const runCoprocessorRollingUpgrade = async (stack: Stack) => {
  const state = await stack.readState();
  if (
    state.scenario.kind !== "coprocessor-consensus" ||
    state.scenario.topology.count !== OPERATORS ||
    state.scenario.topology.threshold !== THRESHOLD ||
    state.versions.lockName !== BASELINE_LOCK
  ) {
    throw new Error(
      "coprocessor-rolling-upgrade needs `fhevm-cli up --scenario three-of-five " +
        `--lock-file profiles/${BASELINE_LOCK} --override test-suite`,
    );
  }
  const startedAt = new Date().toISOString();

  logPhase("start traffic on the 0.13 fleet");
  let trafficError: unknown;
  const traffic = stack.test("erc20-traffic", { env: { TRAFFIC_STOP_FILE } }).catch((error) => {
    trafficError = error;
  });
  const assertTrafficRunning = () => {
    if (trafficError) throw trafficError;
  };
  try {
    await Bun.sleep(SOAK_MS);
    for (let index = 0; index < OPERATORS; index += 1) {
      assertTrafficRunning();
      logPhase(`upgrade operator ${index} to ${TARGET_TAG} (${index} of ${OPERATORS} already upgraded)`);
      await stack.upgradeCoprocessor(index, TARGET_TAG);
      await assertOperatorRuns(index, TARGET_TAG);
      await Bun.sleep(SOAK_MS);
      await assertNoDrift(startedAt);
    }
  } finally {
    await run(["docker", "exec", TEST_SUITE_CONTAINER, "touch", TRAFFIC_STOP_FILE], { allowFailure: true });
    await traffic;
  }
  assertTrafficRunning();
  await assertNoDrift(startedAt);
};
